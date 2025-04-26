use scraper::{Html, Node, Selector};
use crate::seo::{SEOConfig, PageSEO};
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use std::fs;

/// Represents a component's render function
pub type ComponentRenderFn = fn(HashMap<String, String>, &mut ComponentContext) -> String;

// Registry for components
pub struct ComponentRegistry {
    components: HashMap<String, ComponentRenderFn>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }
    pub fn register(&mut self, name: &str, render_fn: ComponentRenderFn) {
        self.components.insert(name.to_string(), render_fn);
    }
    pub fn get(&self, name: &str) -> Option<&ComponentRenderFn> {
        self.components.get(name)
    }
}

// Context for rendering, tracks call stack for circular detection
pub struct ComponentContext<'a> {
    pub registry: &'a ComponentRegistry,
    pub call_stack: Vec<String>,
}

impl<'a> ComponentContext<'a> {
    pub fn new(registry: &'a ComponentRegistry) -> Self {
        Self { registry, call_stack: Vec::new() }
    }
    pub fn is_circular(&self, name: &str) -> bool {
        self.call_stack.contains(&name.to_string())
    }
}

// Recursively render <el-component ... /> tags
pub fn render_components(input: &str, ctx: &mut ComponentContext) -> String {
    let mut output = String::new();
    let mut last = 0;
    let re = regex::Regex::new(r#"<el-component\s+([^/>]+)\s*/>"#).unwrap();
    for cap in re.captures_iter(input) {
        let m = cap.get(0).unwrap();
        output.push_str(&input[last..m.start()]);
        let attrs = &cap[1];
        let params = parse_attrs(attrs);
        let c_name = params.get("c_name").cloned().unwrap_or_default();
        if ctx.is_circular(&c_name) {
            output.push_str(&format!("<!-- Circular component: {} -->", c_name));
        } else if let Some(render_fn) = ctx.registry.get(&c_name) {
            ctx.call_stack.push(c_name.clone());
            let rendered = render_fn(params, ctx);
            output.push_str(&render_components(&rendered, ctx));
            ctx.call_stack.pop();
        } else {
            output.push_str(&format!("<!-- Unknown component: {} -->", c_name));
        }
        last = m.end();
    }
    output.push_str(&input[last..]);
    output
}

fn parse_attrs(attrs: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let re = regex::Regex::new(r#"(\w+)=[\"']([^\"']+)[\"']"#).unwrap();
    for cap in re.captures_iter(attrs) {
        map.insert(cap[1].to_string(), cap[2].to_string());
    }
    map
}

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

/// Loads global macros from a file (macros.toml) in the project root.
pub fn load_global_macros(path: &str) -> std::collections::HashMap<String, String> {
    let mut macros = std::collections::HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some((k, v)) = line.split_once('=') {
                macros.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
            }
        }
    }
    macros
}

/// Replaces all {{MACRO_NAME}} in the input string with their values from the macros map.
pub fn replace_global_macros(input: &str, macros: &std::collections::HashMap<String, String>) -> String {
    let mut output = input.to_string();
    for (k, v) in macros {
        let pattern = format!("{{{{{}}}}}", k);
        output = output.replace(&pattern, v);
    }
    output
}