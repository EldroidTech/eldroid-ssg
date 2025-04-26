use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use rayon::prelude::*; // Import Rayon prelude for parallel iterators

#[derive(Deserialize)]
struct SEOConfig {
    enable_seo: bool,
    default_title: Option<String>,
    default_description: Option<String>,
    meta_tags: HashMap<String, String>,
}

fn load_seo_config(config_path: &str) -> Option<SEOConfig> {
    if let Ok(config_content) = fs::read_to_string(config_path) {
        toml::from_str(&config_content).ok()
    } else {
        None
    }
}

fn generate_html(content: &str, _components_dir: &str, _cache: &mut HashMap<String, String>, _visited: &mut HashSet<String>) -> String {
    // Placeholder implementation for generate_html
    content.to_string()
}

fn generate_html_with_seo(content: &str, components_dir: &str, cache: &mut HashMap<String, String>, visited: &mut HashSet<String>, seo_config: &Option<SEOConfig>) -> String {
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

fn main() {
    let input_dir = "content";
    let output_dir = "output";
    let components_dir = "components";
    let seo_config_path = "seo_config.toml";

    let seo_config = load_seo_config(seo_config_path);

    if !Path::new(output_dir).exists() {
        if let Err(err) = fs::create_dir(output_dir) {
            eprintln!("Error creating output directory: {}", err);
            return;
        }
    }

    let cache = Arc::new(Mutex::new(HashMap::new()));

    match fs::read_dir(input_dir) {
        Ok(entries) => {
            let entries: Vec<_> = entries.filter_map(Result::ok).collect();

            entries.into_par_iter().for_each(|entry| {
                let path = entry.path();
                let cache = Arc::clone(&cache); // Clone the Arc for thread-safe access

                if path.is_file() {
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let mut visited = HashSet::new();
                            let output_content = {
                                let mut cache_lock = cache.lock().expect("Failed to lock cache");
                                generate_html_with_seo(&content, components_dir, &mut cache_lock, &mut visited, &seo_config)
                            };

                            let output_path = Path::new(output_dir).join(path.file_name().unwrap());
                            if let Err(err) = fs::write(output_path, output_content) {
                                eprintln!("Error writing output file: {}", err);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading file '{}': {}", path.display(), err);
                        }
                    }
                }
            });
        }
        Err(err) => {
            eprintln!("Error reading input directory '{}': {}", input_dir, err);
        }
    }
}
