use scraper::{Html, Selector, Node};
use log::warn;
use std::path::Path;
use crate::seo::{PageSEO, SEOConfig};
use crate::variables::Variables;
use crate::macros::MacroProcessor;

pub struct HtmlGenerator {
    variables: Option<Variables>,
    macro_processor: Option<MacroProcessor>,
    dev_mode: bool,
    ws_port: Option<u16>,
}

impl HtmlGenerator {
    pub fn new() -> Self {
        Self {
            variables: None,
            macro_processor: None,
            dev_mode: false,
            ws_port: None,
        }
    }

    pub fn with_variables(mut self, vars: Variables) -> Self {
        self.variables = Some(vars);
        self
    }

    pub fn with_macros(mut self, processor: MacroProcessor) -> Self {
        self.macro_processor = Some(processor);
        self
    }

    pub fn with_dev_mode(mut self, enabled: bool, ws_port: Option<u16>) -> Self {
        self.dev_mode = enabled;
        self.ws_port = ws_port;
        self
    }

    pub fn generate(&self, content: &str) -> String {
        let mut processed = content.to_string();

        // Process variables if configured
        if let Some(vars) = &self.variables {
            processed = vars.substitute(&processed);
        }

        // Process macros if configured
        if let Some(processor) = &self.macro_processor {
            processed = processor.process(&processed);
        }

        // Inject hot reload script in dev mode
        if self.dev_mode {
            if let Some(port) = self.ws_port {
                processed = self.inject_hot_reload(&processed, port);
            }
        }

        processed
    }

    fn inject_hot_reload(&self, html: &str, ws_port: u16) -> String {
        let hot_reload_script = format!(
            r#"<script>
            // Hot Reload Client
            (function() {{
                const ws = new WebSocket(`ws://localhost:{}/ws`);
                
                // Create error overlay
                const errorOverlay = document.createElement('div');
                errorOverlay.style.cssText = `
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    background: rgba(200, 0, 0, 0.85);
                    color: white;
                    padding: 20px;
                    font-family: monospace;
                    font-size: 14px;
                    z-index: 9999;
                    display: none;
                    white-space: pre-wrap;
                    max-height: 50vh;
                    overflow-y: auto;
                `;
                document.body.appendChild(errorOverlay);

                ws.onmessage = (event) => {{
                    try {{
                        const data = JSON.parse(event.data);
                        
                        if (data.type === 'css') {{
                            // Handle CSS hot reload
                            const links = document.querySelectorAll('link[rel="stylesheet"]');
                            links.forEach(link => {{
                                if (link.href.includes(data.path)) {{
                                    const newHref = link.href.split('?')[0] + '?t=' + Date.now();
                                    link.href = newHref;
                                }}
                            }});
                        }} else if (data.type === 'error') {{
                            // Show error overlay
                            errorOverlay.textContent = data.message;
                            errorOverlay.style.display = 'block';
                            setTimeout(() => {{
                                errorOverlay.style.display = 'none';
                            }}, 5000);
                        }} else if (event.data === 'reload') {{
                            window.location.reload();
                        }}
                    }} catch (e) {{
                        if (event.data === 'reload') {{
                            window.location.reload();
                        }}
                    }}
                }};

                ws.onclose = () => {{
                    setTimeout(() => {{
                        window.location.reload();
                    }}, 1000);
                }};
            }})();
            </script>"#,
            ws_port
        );

        if let Some(body_end) = html.rfind("</body>") {
            format!("{}{}{}", &html[..body_end], hot_reload_script, &html[body_end..])
        } else {
            format!("{}{}", html, hot_reload_script)
        }
    }

    pub fn get_variables(&self) -> &Option<Variables> {
        &self.variables
    }

    pub fn get_macro_processor(&self) -> &Option<MacroProcessor> {
        &self.macro_processor
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
            url: Some("".to_string()),
            canonical_url: None,
            path: "".to_string(),
            image: None,
            author: None,
            published_date: None,
            last_modified: None,
            category: None,
            tags: None,
            schema_type: None,
            structured_data: None,
            change_frequency: None,
            priority: None,
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
            ("og:url".to_string(), page_seo.url.clone().unwrap_or_default()),
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