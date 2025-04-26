use eldroid_ssg::seo::load_seo_config;
use eldroid_ssg::html::generate_html_with_seo;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use log::{error, info};
use std::process;

fn main() {
    env_logger::init();

    let input_dir = "content";
    let output_dir = "output";
    let components_dir = "components";
    let seo_config_path = "seo_config.toml";

    let seo_config = match load_seo_config(seo_config_path) {
        Some(config) => {
            info!("SEO configuration loaded successfully from {}", seo_config_path);
            Some(config)
        },
        None => {
            error!("Failed to load SEO configuration from {}", seo_config_path);
            None
        }
    };

    if !Path::new(output_dir).exists() {
        if let Err(err) = fs::create_dir(output_dir) {
            error!("Error creating output directory: {}", err);
            process::exit(1);
        }
    }

    let cache = Arc::new(Mutex::new(HashMap::new()));

    match fs::read_dir(input_dir) {
        Ok(entries) => {
            let entries: Vec<_> = entries.filter_map(Result::ok).collect();

            entries.into_par_iter().for_each(|entry| {
                let path = entry.path();
                let cache = Arc::clone(&cache);

                if path.is_file() {
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let output_content = {
                                let mut cache_lock = cache.lock().expect("Failed to lock cache");
                                generate_html_with_seo(&content, components_dir, &mut cache_lock, &mut HashSet::new(), &seo_config)
                            };

                            let output_path = Path::new(output_dir).join(path.file_name().unwrap());
                            if let Err(err) = fs::write(&output_path, output_content) {
                                error!("Error writing output file '{}': {}", output_path.display(), err);
                            } else {
                                info!("Successfully wrote output file: {}", output_path.display());
                            }
                        }
                        Err(err) => {
                            error!("Error reading file '{}': {}", path.display(), err);
                        }
                    }
                }
            });
        }
        Err(err) => {
            error!("Error reading input directory '{}': {}", input_dir, err);
            process::exit(1);
        }
    }
}
