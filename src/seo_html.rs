use crate::seo::{SEOConfig, PageSEO};
use crate::seo_types::JsonLd;

pub fn generate_meta_tags(page: &PageSEO, config: &SEOConfig) -> String {
    let mut meta = String::new();

    // Basic meta tags
    meta.push_str(&format!(r#"<title>{}</title>
<meta name="description" content="{}" />
"#, 
        page.title,
        page.description.as_deref().unwrap_or(&config.default_description)
    ));

    if let Some(keywords) = &page.keywords {
        meta.push_str(&format!(r#"<meta name="keywords" content="{}" />
"#, keywords.join(", ")));
    }

    // Canonical URL
    let canonical = format!("{}/{}", 
        config.base_url.as_deref().unwrap_or(""),
        page.path.trim_start_matches('/')
    );
    meta.push_str(&format!(r#"<link rel="canonical" href="{}" />
"#, canonical));

    // Open Graph tags
    meta.push_str(&format!(r#"<meta property="og:title" content="{}" />
<meta property="og:type" content="article" />
<meta property="og:url" content="{}" />
"#, 
        page.title,
        canonical
    ));

    if let Some(desc) = &page.description {
        meta.push_str(&format!(r#"<meta property="og:description" content="{}" />
"#, desc));
    }

    if let Some(image) = &page.image {
        meta.push_str(&format!(r#"<meta property="og:image" content="{}" />
<meta property="og:image:alt" content="{}" />
"#, image, page.title));
    }

    // Twitter Card tags
    meta.push_str(r#"<meta name="twitter:card" content="summary_large_image" />"#);

    if let Some(social) = &config.social_media {
        if let Some(site) = &social.twitter_site {
            meta.push_str(&format!(r#"
<meta name="twitter:site" content="{}" />"#, site));
        }
        if let Some(creator) = &social.twitter_creator {
            meta.push_str(&format!(r#"
<meta name="twitter:creator" content="{}" />"#, creator));
        }
    }

    meta.push_str(&format!(r#"
<meta name="twitter:title" content="{}" />"#, page.title));

    if let Some(desc) = &page.description {
        meta.push_str(&format!(r#"
<meta name="twitter:description" content="{}" />"#, desc));
    }

    // Article meta tags for blog posts
    if page.schema_type.as_deref() == Some("BlogPosting") {
        if let Some(author) = &page.author {
            meta.push_str(&format!(r#"
<meta property="article:author" content="{}" />"#, author));
        }
        if let Some(date) = &page.published_date {
            meta.push_str(&format!(r#"
<meta property="article:published_time" content="{}" />"#, date.to_rfc3339()));
        }
        if let Some(date) = &page.last_modified {
            meta.push_str(&format!(r#"
<meta property="article:modified_time" content="{}" />"#, date.to_rfc3339()));
        }
        if let Some(section) = &page.category {
            meta.push_str(&format!(r#"
<meta property="article:section" content="{}" />"#, section));
        }
        if let Some(tags) = &page.tags {
            for tag in tags {
                meta.push_str(&format!(r#"
<meta property="article:tag" content="{}" />"#, tag));
            }
        }
    }

    // Add structured data
    let jsonld = JsonLd::new_article(page, config);
    meta.push_str(&format!(r#"
<script type="application/ld+json">
{}
</script>"#, serde_json::to_string_pretty(&jsonld).unwrap()));

    meta
}

pub fn inject_meta_tags(html: &str, meta_tags: &str) -> String {
    if let Some(head_pos) = html.find("</head>") {
        let (before, after) = html.split_at(head_pos);
        format!("{}\n{}\n{}", before, meta_tags, after)
    } else {
        format!("<html><head>{}</head>{}</html>", meta_tags, html)
    }
}
