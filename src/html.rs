use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use log::{error, debug};

fn find_component_file(components_dir: &str, component_name: &str) -> Option<PathBuf> {
    // First try direct path (allows for nested paths in component name)
    let direct_path = Path::new(components_dir).join(format!("{}.html", component_name));
    if direct_path.exists() {
        return Some(direct_path);
    }

    // Then search recursively through components directory
    let walker = walkdir::WalkDir::new(components_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.path().file_stem() {
                if file_name == component_name {
                    return Some(entry.path().to_path_buf());
                }
            }
        }
    }

    None
}

pub fn generate_html(content: &str, components_dir: &str, cache: &mut HashMap<String, String>, visited: &mut HashSet<String>) -> String {
    let mut result = String::new();
    let mut remaining_content = content.to_string();
    let component_tag_regex = regex::Regex::new("<component\\s+name=\"([^\"]+)\"\\s*/>").unwrap();

    while let Some(captures) = component_tag_regex.captures(&remaining_content) {
        let full_tag = captures.get(0).unwrap();
        let component_name = captures.get(1).unwrap().as_str();

        debug!("Processing component: {}", component_name);

        if visited.contains(component_name) {
            error!("Circular dependency detected for component: {}", component_name);
            result.push_str(&remaining_content[..full_tag.start()]);
            result.push_str(&format!("<!-- Circular dependency detected: {} -->", component_name));
            remaining_content = remaining_content[full_tag.end()..].to_string();
            continue;
        }

        visited.insert(component_name.to_string());

        let component_content = if let Some(cached) = cache.get(component_name) {
            debug!("Using cached component: {}", component_name);
            cached.clone()
        } else {
            match find_component_file(components_dir, component_name) {
                Some(component_path) => {
                    match fs::read_to_string(&component_path) {
                        Ok(content) => {
                            debug!("Loaded component from file: {}", component_path.display());
                            cache.insert(component_name.to_string(), content.clone());
                            content
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
        };

        let rendered_component = generate_html(&component_content, components_dir, cache, visited);
        result.push_str(&remaining_content[..full_tag.start()]);
        result.push_str(&rendered_component);

        visited.remove(component_name);
        remaining_content = remaining_content[full_tag.end()..].to_string();
    }

    result.push_str(&remaining_content);
    result
}

pub fn generate_html_with_seo(content: &str, components_dir: &str, cache: &mut HashMap<String, String>, visited: &mut HashSet<String>, seo_config: &Option<crate::seo::SEOConfig>) -> String {
    let mut output = generate_html(content, components_dir, cache, visited);

    if let Some(config) = seo_config {
        if config.enable_seo {
            let title = config.default_title.clone().unwrap_or_else(|| "Default Title".to_string());
            let description = config.default_description.clone().unwrap_or_else(|| "Default Description".to_string());

            let mut meta_tags = String::new();
            for (key, value) in &config.meta_tags {
                meta_tags.push_str(&format!("<meta name=\"{}\" content=\"{}\">\n", key, value));
            }

            output = format!(
                "<html><head><title>{}</title><meta name=\"description\" content=\"{}\">\n{}\n</head><body>{}</body></html>",
                title, description, meta_tags, output
            );
        }
    }

    output
}