use eldroid_ssg::seo::load_seo_config;
use eldroid_ssg::html::{generate_html_with_seo, HtmlGenerator};
use eldroid_ssg::seo_gen::{generate_sitemap, generate_rss, generate_robots_txt};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use parking_lot::Mutex;
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

    // Create a thread-safe HtmlGenerator
    let generator = Arc::new(Mutex::new(HtmlGenerator::new()));
    // Store processed pages for sitemap and RSS generation
    let processed_pages = Arc::new(Mutex::new(Vec::new()));

    // Process files in parallel chunks for better performance
    match fs::read_dir(input_dir) {
        Ok(entries) => {
            let entries: Vec<_> = entries
                .filter_map(Result::ok)
                .filter(|e| e.path().is_file())
                .collect();

            // Process files in chunks to balance parallelism and resource usage
            entries.par_chunks(4).for_each(|chunk| {
                for entry in chunk {
                    let path = entry.path();
                    let generator = Arc::clone(&generator);
                    let processed_pages = Arc::clone(&processed_pages);

                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let output_content = {
                                let mut generator = generator.lock();
                                generate_html_with_seo(
                                    &content,
                                    components_dir,
                                    &mut generator,
                                    &seo_config
                                )
                            };

                            let rel_path = path.strip_prefix(input_dir)
                                .unwrap_or(&path)
                                .to_string_lossy()
                                .into_owned();

                            // Extract PageSEO for sitemap/RSS
                            if let Some(config) = &seo_config {
                                if config.enable_seo {
                                    if let Some(page_seo) = eldroid_ssg::seo::parse_page_seo(&content) {
                                        processed_pages.lock().push((rel_path.clone(), page_seo));
                                    }
                                }
                            }

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

            // Generate sitemap, RSS feed, and robots.txt after processing all files
            if let Some(config) = &seo_config {
                if config.enable_seo {
                    if let Some(base_url) = &config.base_url {
                        let pages = processed_pages.lock();
                        let output_path = Path::new(output_dir);

                        if let Err(e) = generate_sitemap(base_url, &pages, output_path) {
                            error!("Failed to generate sitemap: {}", e);
                        }

                        if let Err(e) = generate_rss(config, &pages, output_path) {
                            error!("Failed to generate RSS feed: {}", e);
                        }

                        if let Err(e) = generate_robots_txt(config, output_path) {
                            error!("Failed to generate robots.txt: {}", e);
                        }
                    } else {
                        error!("base_url is required in SEO config for sitemap generation");
                    }
                }
            }
        }
        Err(err) => {
            error!("Error reading input directory '{}': {}", input_dir, err);
            process::exit(1);
        }
    }
}
