use clap::Parser;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use parking_lot::Mutex;
use rayon::prelude::*;
use log::{error, info};

use eldroid_ssg::{
    config::{CliArgs, BuildConfig},
    seo::load_seo_config,
    html::{generate_html_with_seo, HtmlGenerator},
    seo_gen::{generate_sitemap, generate_rss, generate_robots_txt},
    minify::Minifier,
    analyzer::Analyzer,
};

fn main() {
    env_logger::init();

    // Parse command line arguments
    let args = CliArgs::parse();
    let config = BuildConfig::from(&args);

    // Initialize components
    let minifier = if config.minify {
        Some(Minifier::default())
    } else {
        None
    };

    let analyzer = if config.analyze_performance || config.security_checks {
        let base_url = load_seo_config(&args.seo_config)
            .and_then(|cfg| cfg.base_url);
        Some(Analyzer::new(base_url))
    } else {
        None
    };

    // Load SEO config if enabled
    let seo_config = if config.enable_seo {
        match load_seo_config(&args.seo_config) {
            Some(config) => {
                info!("SEO configuration loaded successfully");
                Some(config)
            },
            None => {
                error!("Failed to load SEO configuration");
                None
            }
        }
    } else {
        None
    };

    // Ensure output directories exist
    let perf_dir = format!("{}/performance", args.output_dir);
    for dir in [&args.output_dir, &perf_dir] {
        if let Err(e) = fs::create_dir_all(dir) {
            error!("Failed to create directory {}: {}", dir, e);
            std::process::exit(1);
        }
    }

    // Process all content files
    let html_gen = Arc::new(HtmlGenerator::new());
    let processed_files = Arc::new(Mutex::new(Vec::new()));

    let walk_dir_recursive = |dir: &Path| -> Vec<std::path::PathBuf> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(walk_dir_recursive(&path));
                } else if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
                    files.push(path);
                }
            }
        }
        files
    };

    let content_files = walk_dir_recursive(Path::new(&args.input_dir));
    
    content_files.par_iter().for_each(|file_path| {
        if let Ok(content) = fs::read_to_string(file_path) {
            let mut final_content = if let Some(seo) = &seo_config {
                generate_html_with_seo(&content, seo, &html_gen)
            } else {
                html_gen.generate(&content)
            };

            // Analyze if enabled
            if let Some(analyzer) = &analyzer {
                if config.security_checks {
                    let security_report = analyzer.analyze_security(&final_content, file_path);
                    if !security_report.mixed_content.is_empty() {
                        error!("Mixed content found in {}: {:?}", file_path.display(), security_report.mixed_content);
                    }
                    if !security_report.insecure_links.is_empty() {
                        error!("Insecure links found in {}: {:?}", file_path.display(), security_report.insecure_links);
                    }
                }

                if config.analyze_performance {
                    let perf_report = analyzer.analyze_performance(&final_content, file_path);
                    let perf_file = Path::new(&perf_dir)
                        .join(file_path.file_name().unwrap())
                        .with_extension("perf.txt");
                    
                    if let Err(e) = fs::write(&perf_file, format!(
                        "Performance Analysis for {}\n\n{}\n\nRecommendations:\n{}", 
                        file_path.display(),
                        perf_report.details,
                        perf_report.recommendations.join("\n")
                    )) {
                        error!("Failed to write performance report: {}", e);
                    }
                }
            }

            // Minify if enabled
            if let Some(minifier) = &minifier {
                final_content = minifier.minify_html(&final_content);
            }

            let out_path = Path::new(&args.output_dir)
                .join(file_path.strip_prefix(&args.input_dir).unwrap());

            if let Some(parent) = out_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    error!("Failed to create directory {}: {}", parent.display(), e);
                    return;
                }
            }

            if let Err(e) = fs::write(&out_path, final_content) {
                error!("Failed to write file {}: {}", out_path.display(), e);
                return;
            }

            processed_files.lock().push(out_path);
        }
    });

    // Generate SEO files if enabled
    if config.enable_seo {
        if let Some(seo) = &seo_config {
            let processed = processed_files.lock();
            if let Err(e) = generate_sitemap(&processed, seo, &args.output_dir) {
                error!("Failed to generate sitemap: {}", e);
            }
            if let Err(e) = generate_rss(&processed, seo, &args.output_dir) {
                error!("Failed to generate RSS feed: {}", e);
            }
            if let Err(e) = generate_robots_txt(seo, &args.output_dir) {
                error!("Failed to generate robots.txt: {}", e);
            }
        }
    }
}
