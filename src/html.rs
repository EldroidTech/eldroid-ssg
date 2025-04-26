use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use log::{error, debug};
use once_cell::sync::Lazy;
use regex::Regex;

// Compile regex pattern once
static COMPONENT_TAG_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"<component\s+name=["']([^"']+)["']\s*/>"#).unwrap()
});

// Cache for component file paths to avoid repeated filesystem searches
type ComponentPathCache = HashMap<String, Option<PathBuf>>;

fn normalize_component_name(name: &str) -> String {
    name.replace('\\', "/")
}

fn resolve_component_path(base_dir: &Path, current_component_dir: Option<&Path>, component_name: &str) -> PathBuf {
    let normalized_name = normalize_component_name(component_name);
    
    if normalized_name.starts_with('/') {
        Path::new(base_dir).join(&normalized_name[1..])
    } else if let Some(current_dir) = current_component_dir {
        current_dir.join(&normalized_name)
    } else {
        Path::new(base_dir).join(normalized_name)
    }
}

fn find_component_file(
    components_dir: &str,
    current_component_dir: Option<&Path>,
    component_name: &str,
    path_cache: &mut ComponentPathCache,
) -> Option<PathBuf> {
    let cache_key = match current_component_dir {
        Some(dir) => format!("{}:{}", dir.display(), component_name),
        None => component_name.to_string(),
    };

    if let Some(cached_path) = path_cache.get(&cache_key) {
        return cached_path.clone();
    }

    let component_path = resolve_component_path(
        Path::new(components_dir),
        current_component_dir,
        component_name
    );

    let with_extension = component_path.with_extension("html");
    if with_extension.exists() {
        path_cache.insert(cache_key, Some(with_extension.clone()));
        return Some(with_extension);
    }

    // Fallback to case-insensitive search only if needed
    let walker = walkdir::WalkDir::new(components_dir)
        .follow_links(true)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file());

    for entry in walker {
        if let Some(file_name) = entry.path().file_stem() {
            if file_name.to_string_lossy().eq_ignore_ascii_case(component_name) {
                let path = entry.path().to_path_buf();
                path_cache.insert(cache_key, Some(path.clone()));
                return Some(path);
            }
        }
    }

    path_cache.insert(cache_key, None);
    None
}

fn get_component_dir(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

#[derive(Default)]
pub struct HtmlGenerator {
    content_cache: HashMap<String, String>,
    path_cache: ComponentPathCache,
}

impl HtmlGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_html(
        &mut self,
        content: &str, 
        components_dir: &str, 
        visited: &mut HashSet<String>,
        current_component_dir: Option<&Path>
    ) -> String {
        let mut result = String::with_capacity(content.len());
        let mut remaining_content = content;

        while let Some(captures) = COMPONENT_TAG_REGEX.captures(remaining_content) {
            let full_tag = captures.get(0).unwrap();
            let component_name = captures.get(1).unwrap().as_str();
            let normalized_name = normalize_component_name(component_name);

            debug!("Processing component: {}", normalized_name);

            result.push_str(&remaining_content[..full_tag.start()]);

            if visited.contains(&normalized_name) {
                error!("Circular dependency detected for component: {}", normalized_name);
                result.push_str(&format!("<!-- Circular dependency detected: {} -->", normalized_name));
            } else {
                visited.insert(normalized_name.clone());
                let component_content = self.load_component(
                    &normalized_name,
                    components_dir,
                    current_component_dir,
                    visited
                );
                result.push_str(&component_content);
                visited.remove(&normalized_name);
            }

            remaining_content = &remaining_content[full_tag.end()..];
        }

        result.push_str(remaining_content);
        result
    }

    fn load_component(
        &mut self,
        component_name: &str,
        components_dir: &str,
        current_component_dir: Option<&Path>,
        visited: &mut HashSet<String>
    ) -> String {
        if let Some(cached) = self.content_cache.get(component_name) {
            debug!("Using cached component content: {}", component_name);
            return cached.clone();
        }

        match find_component_file(components_dir, current_component_dir, component_name, &mut self.path_cache) {
            Some(component_path) => {
                match fs::read_to_string(&component_path) {
                    Ok(content) => {
                        debug!("Loaded component from file: {}", component_path.display());
                        let component_dir = get_component_dir(&component_path);
                        let rendered = self.generate_html(&content, components_dir, visited, component_dir.as_deref());
                        self.content_cache.insert(component_name.to_string(), rendered.clone());
                        rendered
                    }
                    Err(err) => {
                        error!("Failed to read component file '{}': {}", component_path.display(), err);
                        format!("<!-- Failed to read component: {} -->", component_name)
                    }
                }
            }
            None => {
                error!("Component not found: {}", component_name);
                format!("<!-- Component not found: {} -->", component_name)
            }
        }
    }
}

pub fn generate_html_with_seo(
    content: &str,
    components_dir: &str,
    generator: &mut HtmlGenerator,
    seo_config: &Option<crate::seo::SEOConfig>
) -> String {
    let mut visited = HashSet::new();
    let mut output = generator.generate_html(content, components_dir, &mut visited, None);

    if let Some(config) = seo_config {
        if config.enable_seo {
            let page_seo = crate::seo::parse_page_seo(&output);
            let merged_seo = config.merge_with_page(page_seo.as_ref());
            
            let mut head_tags = String::with_capacity(1024);

            // Title
            if let Some(title) = &merged_seo.title {
                head_tags.push_str(&format!("<title>{}</title>\n", title));
            }

            // Basic meta tags
            if let Some(desc) = &merged_seo.description {
                head_tags.push_str(&format!("<meta name=\"description\" content=\"{}\">\n", desc));
            }

            // Language
            if let Some(lang) = &config.default_language {
                head_tags.push_str(&format!("<meta http-equiv=\"content-language\" content=\"{}\">\n", lang));
            }

            // Alternate languages
            if let Some(alternates) = &config.alternate_languages {
                for (lang, url) in alternates {
                    head_tags.push_str(&format!(
                        "<link rel=\"alternate\" hreflang=\"{}\" href=\"{}\">\n",
                        lang, url
                    ));
                }
            }

            // Canonical URL
            if let Some(canonical) = &merged_seo.canonical_url {
                head_tags.push_str(&format!("<link rel=\"canonical\" href=\"{}\">\n", canonical));
            }

            // Robots meta
            if let Some(robots) = &merged_seo.robots {
                let mut directives = Vec::new();
                directives.push(if robots.index { "index" } else { "noindex" });
                directives.push(if robots.follow { "follow" } else { "nofollow" });
                if !robots.archive { directives.push("noarchive"); }
                if !robots.imageindex { directives.push("noimageindex"); }
                head_tags.push_str(&format!("<meta name=\"robots\" content=\"{}\">\n", directives.join(", ")));
            }

            // Custom meta tags
            if let Some(meta_tags) = &merged_seo.meta_tags {
                for (key, value) in meta_tags {
                    head_tags.push_str(&format!("<meta name=\"{}\" content=\"{}\">\n", key, value));
                }
            }

            // Open Graph tags
            if let Some(og) = &merged_seo.open_graph {
                if let Some(title) = &og.title {
                    head_tags.push_str(&format!("<meta property=\"og:title\" content=\"{}\">\n", title));
                }
                if let Some(desc) = &og.description {
                    head_tags.push_str(&format!("<meta property=\"og:description\" content=\"{}\">\n", desc));
                }
                if let Some(image) = &og.image {
                    head_tags.push_str(&format!("<meta property=\"og:image\" content=\"{}\">\n", image));
                }
                if let Some(url) = &og.url {
                    head_tags.push_str(&format!("<meta property=\"og:url\" content=\"{}\">\n", url));
                }
                if let Some(site_name) = &og.site_name {
                    head_tags.push_str(&format!("<meta property=\"og:site_name\" content=\"{}\">\n", site_name));
                }
                if let Some(locale) = &og.locale {
                    head_tags.push_str(&format!("<meta property=\"og:locale\" content=\"{}\">\n", locale));
                }
                if let Some(og_type) = &og.og_type {
                    head_tags.push_str(&format!("<meta property=\"og:type\" content=\"{}\">\n", og_type));
                }
            }

            // Twitter Card tags
            if let Some(twitter) = &merged_seo.twitter_card {
                head_tags.push_str(&format!("<meta name=\"twitter:card\" content=\"{}\">\n", twitter.card_type));
                if let Some(site) = &twitter.site {
                    head_tags.push_str(&format!("<meta name=\"twitter:site\" content=\"{}\">\n", site));
                }
                if let Some(creator) = &twitter.creator {
                    head_tags.push_str(&format!("<meta name=\"twitter:creator\" content=\"{}\">\n", creator));
                }
                if let Some(image) = &twitter.image {
                    head_tags.push_str(&format!("<meta name=\"twitter:image\" content=\"{}\">\n", image));
                }
            }

            // JSON-LD structured data
            if let Some(json_ld_items) = &merged_seo.json_ld {
                for json_ld in json_ld_items {
                    let mut schema = json_ld.properties.clone();
                    schema.insert("@type".to_string(), serde_json::Value::String(json_ld.schema_type.clone()));
                    schema.insert("@context".to_string(), serde_json::Value::String("https://schema.org".to_string()));
                    
                    head_tags.push_str("<script type=\"application/ld+json\">\n");
                    head_tags.push_str(&serde_json::to_string_pretty(&schema).unwrap_or_default());
                    head_tags.push_str("\n</script>\n");
                }
            }

            output = format!(
                "<html><head>{}</head><body>{}</body></html>",
                head_tags, output
            );
        }
    }

    output
}