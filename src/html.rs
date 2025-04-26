use scraper::{Html, Node, Selector};
use crate::seo::{SEOConfig, PageSEO};
use std::sync::Arc;

pub struct HtmlGenerator {
    head_selector: Selector,
    title_selector: Selector,
    meta_selector: Selector,
}

impl HtmlGenerator {
    pub fn new() -> Self {
        Self {
            head_selector: Selector::parse("head").unwrap(),
            title_selector: Selector::parse("title").unwrap(),
            meta_selector: Selector::parse("meta").unwrap(),
        }
    }

    pub fn generate(&self, content: &str) -> String {
        content.to_string()
    }

    fn inject_meta_tags(&self, document: &mut Html, page_seo: &PageSEO, site_seo: &SEOConfig) {
        let mut head = document.select(&self.head_selector).next().unwrap();
        
        // Set or update title
        let title = format!("{} | {}", page_seo.title, site_seo.site_name);
        if let Some(existing_title) = document.select(&self.title_selector).next() {
            let title_node = Node::Text(title.into());
            existing_title.first_child().unwrap().replace_with(title_node);
        } else {
            let title_elem = format!("<title>{}</title>", title);
            head.append(Node::from_html(&title_elem).unwrap());
        }

        // Add meta description
        let desc = format!(
            "<meta name=\"description\" content=\"{}\">",
            page_seo.description.as_deref().unwrap_or(&site_seo.default_description)
        );
        head.append(Node::from_html(&desc).unwrap());

        // Add canonical URL if present
        if let Some(canonical) = &page_seo.canonical_url {
            let canonical_tag = format!(
                "<link rel=\"canonical\" href=\"{}\">",
                canonical
            );
            head.append(Node::from_html(&canonical_tag).unwrap());
        }

        // Add Open Graph tags
        let og_tags = vec![
            ("og:title", &title),
            ("og:description", page_seo.description.as_deref().unwrap_or(&site_seo.default_description)),
            ("og:type", "website"),
            ("og:url", &page_seo.url),
        ];

        for (property, content) in og_tags {
            let meta = format!(
                "<meta property=\"{}\" content=\"{}\">",
                property, content
            );
            head.append(Node::from_html(&meta).unwrap());
        }

        // Add page-specific keywords if present
        if let Some(keywords) = &page_seo.keywords {
            let keywords_tag = format!(
                "<meta name=\"keywords\" content=\"{}\">",
                keywords.join(", ")
            );
            head.append(Node::from_html(&keywords_tag).unwrap());
        }

        // Add structured data if present
        if let Some(structured_data) = &page_seo.structured_data {
            let script = format!(
                "<script type=\"application/ld+json\">{}</script>",
                structured_data
            );
            head.append(Node::from_html(&script).unwrap());
        }
    }
}

pub fn generate_html_with_seo(content: &str, site_seo: &SEOConfig, generator: &Arc<HtmlGenerator>) -> String {
    // Extract page-specific SEO data from HTML comments
    let page_seo = match crate::seo::parse_page_seo(content) {
        Some(seo) => seo,
        None => return generator.generate(content),
    };

    // Parse HTML and inject SEO tags
    let mut document = Html::parse_document(content);
    generator.inject_meta_tags(&mut document, &page_seo, site_seo);
    
    document.html()
}