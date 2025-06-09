use std::path::{Path, PathBuf};
use std::fs;
use chrono::{DateTime, Utc, FixedOffset};
use crate::seo::SEOConfig;
use crate::markdown::BlogFrontMatter;
use yaml_front_matter::YamlFrontMatter;

pub fn generate_sitemap(processed_files: &[PathBuf], config: &SEOConfig, output_dir: &str) -> std::io::Result<()> {
    let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">"#);

    let base_url = config.base_url.as_deref().unwrap_or("");

    for file in processed_files {
        if let Some(relative_path) = file.strip_prefix(output_dir).ok() {
            if let Ok(content) = fs::read_to_string(file) {
                // Try to parse frontmatter for markdown files
                let front_matter = if file.extension().map_or(false, |ext| ext == "md") {
                    YamlFrontMatter::parse::<BlogFrontMatter>(&content).ok()
                } else {
                    None
                };

                let url_path = relative_path
                    .to_str()
                    .unwrap()
                    .replace("\\", "/")
                    .trim_start_matches('/')
                    .to_string();

                let full_url = format!("{}/{}", base_url.trim_end_matches('/'), url_path);

                sitemap.push_str("\n  <url>");
                sitemap.push_str(&format!("\n    <loc>{}</loc>", full_url));

                // Add image if available in frontmatter
                if let Some(yaml) = &front_matter {
                    if let Some(image) = &yaml.metadata.image {
                        sitemap.push_str(&format!(r#"
    <image:image>
      <image:loc>{}/{}</image:loc>
      <image:title>{}</image:title>
    </image:image>"#, base_url.trim_end_matches('/'), image.trim_start_matches('/'), yaml.metadata.title));
                    }

                    sitemap.push_str(&format!("\n    <lastmod>{}</lastmod>", yaml.metadata.date));
                } else {
                    // Use file modification time for non-markdown files
                    if let Ok(metadata) = fs::metadata(file) {
                        if let Ok(modified) = metadata.modified() {
                            let datetime: DateTime<Utc> = modified.into();
                            sitemap.push_str(&format!("\n    <lastmod>{}</lastmod>", 
                                datetime.format("%Y-%m-%dT%H:%M:%SZ")));
                        }
                    }
                }

                sitemap.push_str("\n  </url>");
            }
        }
    }

    sitemap.push_str("\n</urlset>");
    fs::write(Path::new(output_dir).join("sitemap.xml"), sitemap)?;
    Ok(())
}

pub fn generate_rss(processed_files: &[PathBuf], config: &SEOConfig, output_dir: &str) -> std::io::Result<()> {
    let base_url = config.base_url.as_deref().unwrap_or("");
    let mut rss = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/"
                     xmlns:dc="http://purl.org/dc/elements/1.1/"
                     xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>{}</title>
        <link>{}</link>
        <description>{}</description>
        <language>en-us</language>
        <lastBuildDate>{}</lastBuildDate>
        <atom:link href="{}/rss.xml" rel="self" type="application/rss+xml"/>"#,
        config.site_name,
        base_url,
        config.default_description,
        Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"),
        base_url
    );

    for file in processed_files {
        if file.extension().map_or(false, |ext| ext == "md") {
            if let Ok(content) = fs::read_to_string(file) {
                if let Ok(yaml_content) = YamlFrontMatter::parse::<BlogFrontMatter>(&content) {
                    let url_path = file
                        .strip_prefix(output_dir)
                        .unwrap()
                        .with_extension("html")
                        .to_str()
                        .unwrap()
                        .replace("\\", "/")
                        .trim_start_matches('/')
                        .to_string();

                    rss.push_str(&format!(r#"
        <item>
            <title>{}</title>
            <link>{}/{}</link>
            <description><![CDATA[{}]]></description>
            <pubDate>{}</pubDate>
            <guid isPermaLink="true">{}/{}</guid>"#,
                        yaml_content.metadata.title,
                        base_url.trim_end_matches('/'),
                        url_path,
                        yaml_content.metadata.description.unwrap_or_else(|| String::from("No description available")),
                        DateTime::parse_from_rfc3339(&yaml_content.metadata.date)
                            .unwrap_or_else(|_| DateTime::from_naive_utc_and_offset(
                                Utc::now().naive_utc(),
                                FixedOffset::east_opt(0).unwrap()
                            ))
                            .format("%a, %d %b %Y %H:%M:%S GMT"),
                        base_url.trim_end_matches('/'),
                        url_path
                    ));

                    // Add author if available
                    if let Some(author) = yaml_content.metadata.author {
                        rss.push_str(&format!("\n            <dc:creator>{}</dc:creator>", author));
                    }

                    // Add content
                    rss.push_str(&format!("\n            <content:encoded><![CDATA[{}]]></content:encoded>", 
                        markdown_to_html(&yaml_content.content)));

                    rss.push_str("\n        </item>");
                }
            }
        }
    }

    rss.push_str("\n    </channel>\n</rss>");
    fs::write(Path::new(output_dir).join("rss.xml"), rss)?;
    Ok(())
}

pub fn generate_robots_txt(config: &SEOConfig, output_dir: &str) -> std::io::Result<()> {
    let base_url = config.base_url.as_deref().unwrap_or("");
    let robots = format!(r#"User-agent: *
Allow: /

# Sitemaps
Sitemap: {}/sitemap.xml"#,
        base_url
    );

    fs::write(Path::new(output_dir).join("robots.txt"), robots)?;
    Ok(())
}

fn markdown_to_html(content: &str) -> String {
    use pulldown_cmark::{Parser, html};
    let parser = Parser::new(content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}