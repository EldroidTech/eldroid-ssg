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
            let title = config.default_title.as_deref().unwrap_or("Default Title");
            let description = config.default_description.as_deref().unwrap_or("Default Description");

            let meta_tags: String = config.meta_tags
                .iter()
                .map(|(key, value)| format!("<meta name=\"{}\" content=\"{}\">\n", key, value))
                .collect();

            output = format!(
                "<html><head><title>{}</title><meta name=\"description\" content=\"{}\">\n{}</head><body>{}</body></html>",
                title, description, meta_tags, output
            );
        }
    }

    output
}