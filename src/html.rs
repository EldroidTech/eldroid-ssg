use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use log::{error, debug};

fn normalize_component_name(name: &str) -> String {
    // Convert Windows path separators to Unix style
    name.replace('\\', "/")
}

fn resolve_component_path(base_dir: &Path, current_component_dir: Option<&Path>, component_name: &str) -> PathBuf {
    let normalized_name = normalize_component_name(component_name);
    
    if normalized_name.starts_with('/') {
        // Absolute path within components directory
        Path::new(base_dir).join(&normalized_name[1..])
    } else if let Some(current_dir) = current_component_dir {
        // Relative to current component's directory
        current_dir.join(&normalized_name)
    } else {
        // Relative to components root
        Path::new(base_dir).join(normalized_name)
    }
}

fn find_component_file(components_dir: &str, current_component_dir: Option<&Path>, component_name: &str) -> Option<PathBuf> {
    let component_path = resolve_component_path(
        Path::new(components_dir),
        current_component_dir,
        component_name
    );

    // Try exact path first
    let with_extension = component_path.with_extension("html");
    if with_extension.exists() {
        return Some(with_extension);
    }

    // Fallback to case-insensitive search
    let walker = walkdir::WalkDir::new(components_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.path().file_stem() {
                if file_name.to_string_lossy().eq_ignore_ascii_case(component_name) {
                    return Some(entry.path().to_path_buf());
                }
            }
        }
    }

    None
}

fn get_component_dir(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

pub fn generate_html(
    content: &str, 
    components_dir: &str, 
    cache: &mut HashMap<String, String>, 
    visited: &mut HashSet<String>,
    current_component_dir: Option<&Path>
) -> String {
    let mut result = String::new();
    let mut remaining_content = content.to_string();
    let component_tag_regex = regex::Regex::new("<component\\s+name=\"([^\"]+)\"\\s*/>").unwrap();

    while let Some(captures) = component_tag_regex.captures(&remaining_content) {
        let full_tag = captures.get(0).unwrap();
        let component_name = captures.get(1).unwrap().as_str();
        let normalized_name = normalize_component_name(component_name);

        debug!("Processing component: {}", normalized_name);

        if visited.contains(&normalized_name) {
            error!("Circular dependency detected for component: {}", normalized_name);
            result.push_str(&remaining_content[..full_tag.start()]);
            result.push_str(&format!("<!-- Circular dependency detected: {} -->", normalized_name));
            remaining_content = remaining_content[full_tag.end()..].to_string();
            continue;
        }

        visited.insert(normalized_name.clone());

        let component_content = if let Some(cached) = cache.get(&normalized_name) {
            debug!("Using cached component: {}", normalized_name);
            cached.clone()
        } else {
            match find_component_file(components_dir, current_component_dir, &normalized_name) {
                Some(component_path) => {
                    match fs::read_to_string(&component_path) {
                        Ok(content) => {
                            debug!("Loaded component from file: {}", component_path.display());
                            cache.insert(normalized_name.clone(), content.clone());
                            // Generate HTML with the component's directory as the new base for relative paths
                            let component_dir = get_component_dir(&component_path);
                            generate_html(&content, components_dir, cache, visited, component_dir.as_deref())
                        }
                        Err(err) => {
                            error!("Failed to read component file '{}': {}", component_path.display(), err);
                            format!("<!-- Failed to read component: {} -->", normalized_name)
                        }
                    }
                }
                None => {
                    error!("Component not found: {}", normalized_name);
                    format!("<!-- Component not found: {} -->", normalized_name)
                }
            }
        };

        result.push_str(&remaining_content[..full_tag.start()]);
        result.push_str(&component_content);

        visited.remove(&normalized_name);
        remaining_content = remaining_content[full_tag.end()..].to_string();
    }

    result.push_str(&remaining_content);
    result
}

pub fn generate_html_with_seo(content: &str, components_dir: &str, cache: &mut HashMap<String, String>, visited: &mut HashSet<String>, seo_config: &Option<crate::seo::SEOConfig>) -> String {
    let mut output = generate_html(content, components_dir, cache, visited, None);

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