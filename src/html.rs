use scraper::{Html, Selector, Node};
use log::warn;
use std::path::Path;
use crate::seo::{PageSEO, SEOConfig};

pub struct HtmlGenerator;

impl HtmlGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, content: &str) -> String {
        content.to_string()
    }
}

pub fn generate_html_with_seo(content: &str, site_seo: &SEOConfig, html_gen: &HtmlGenerator) -> String {
    let html = html_gen.generate(content);
    if let Some(page_seo) = crate::seo::parse_page_seo(&html) {
        update_seo_tags(&html, &page_seo, site_seo, Path::new(""))
    } else {
        let default_page_seo = PageSEO {
            title: site_seo.site_name.clone(),
            description: Some(site_seo.default_description.clone()),
            keywords: Some(site_seo.default_keywords.clone()),
            url: "".to_string(),
            canonical_url: None,
            structured_data: None,
        };
        update_seo_tags(&html, &default_page_seo, site_seo, Path::new(""))
    }
}

pub fn update_seo_tags(html_str: &str, page_seo: &PageSEO, site_seo: &SEOConfig, file_path: &Path) -> String {
    let mut document = Html::parse_document(html_str);
    let head_selector = Selector::parse("head").unwrap();
    let title_selector = Selector::parse("title").unwrap();
    let meta_desc_selector = Selector::parse("meta[name='description']").unwrap();
    let canonical_selector = Selector::parse("link[rel='canonical']").unwrap();

    if let Some(head) = document.select(&head_selector).next() {
        let head_id = head.id();

        // Update title
        let title = format!("{} | {}", &page_seo.title, site_seo.site_name);
        let title_html = format!("<head><title>{}</title></head>", title);
        let title_frag = Html::parse_fragment(&title_html);
        
        // Remove existing title and add new one
        {
            let existing_title_id = document.select(&title_selector)
                .next()
                .map(|el| el.id());
            
            if let Some(id) = existing_title_id {
                document.tree.get_mut(id).unwrap().detach();
            }

            if let Some(title_elem) = title_frag.select(&Selector::parse("title").unwrap()).next() {
                document.tree.get_mut(head_id).unwrap()
                    .append(Node::Element(title_elem.value().clone()));
            }
        }

        // Update description
        if let Some(description) = &page_seo.description {
            let desc_html = format!("<head><meta name=\"description\" content=\"{}\"></head>", description);
            let desc_frag = Html::parse_fragment(&desc_html);
            
            let existing_desc_id = document.select(&meta_desc_selector)
                .next()
                .map(|el| el.id());
            
            if let Some(id) = existing_desc_id {
                document.tree.get_mut(id).unwrap().detach();
            }
            
            if let Some(meta_elem) = desc_frag.select(&Selector::parse("meta").unwrap()).next() {
                document.tree.get_mut(head_id).unwrap()
                    .append(Node::Element(meta_elem.value().clone()));
            }
        }

        // Update canonical URL
        if let Some(canonical_url) = &page_seo.canonical_url {
            let canonical_html = format!("<head><link rel=\"canonical\" href=\"{}\"></head>", canonical_url);
            let canonical_frag = Html::parse_fragment(&canonical_html);
            
            let existing_canonical_id = document.select(&canonical_selector)
                .next()
                .map(|el| el.id());
            
            if let Some(id) = existing_canonical_id {
                document.tree.get_mut(id).unwrap().detach();
            }
            
            if let Some(link_elem) = canonical_frag.select(&Selector::parse("link").unwrap()).next() {
                document.tree.get_mut(head_id).unwrap()
                    .append(Node::Element(link_elem.value().clone()));
            }
        }

        // Update Open Graph tags
        let og_tags = vec![
            ("og:title".to_string(), page_seo.title.clone()),
            ("og:description".to_string(), page_seo.description.clone().unwrap_or_else(|| site_seo.default_description.clone())),
            ("og:type".to_string(), "website".to_string()),
            ("og:url".to_string(), page_seo.url.clone()),
            ("og:site_name".to_string(), site_seo.site_name.clone()),
        ];

        for (property, content) in og_tags {
            let meta_html = format!("<head><meta property=\"{}\" content=\"{}\"></head>", property, content);
            let meta_frag = Html::parse_fragment(&meta_html);
            if let Some(meta_elem) = meta_frag.select(&Selector::parse("meta").unwrap()).next() {
                document.tree.get_mut(head_id).unwrap()
                    .append(Node::Element(meta_elem.value().clone()));
            }
        }

        // Update keywords if available
        if let Some(keywords) = &page_seo.keywords {
            let keywords_html = format!("<head><meta name=\"keywords\" content=\"{}\"></head>", keywords.join(", "));
            let keywords_frag = Html::parse_fragment(&keywords_html);
            if let Some(meta_elem) = keywords_frag.select(&Selector::parse("meta").unwrap()).next() {
                document.tree.get_mut(head_id).unwrap()
                    .append(Node::Element(meta_elem.value().clone()));
            }
        }

        // Add Google Analytics if configured
        if let Some(ga_id) = &site_seo.google_site_verification {
            let script_html = format!(
                "<head><script async src=\"https://www.googletagmanager.com/gtag/js?id={}\"></script>\
                <script>\
                window.dataLayer = window.dataLayer || [];\
                function gtag(){{dataLayer.push(arguments);}}\
                gtag('js', new Date());\
                gtag('config', '{}');\
                </script></head>",
                ga_id, ga_id
            );
            let script_frag = Html::parse_fragment(&script_html);
            for script_elem in script_frag.select(&Selector::parse("script").unwrap()) {
                document.tree.get_mut(head_id).unwrap()
                    .append(Node::Element(script_elem.value().clone()));
            }
        }
    } else {
        warn!("No <head> tag found in {}", file_path.display());
    }

    document.html()
}