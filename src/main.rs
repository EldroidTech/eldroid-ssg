use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::Mutex;
use rayon::prelude::*;
use log::{error, info};
use tokio;
use anyhow::{Result, anyhow};

use eldroid_ssg::{
    config::{CliArgs, BuildConfig},
    seo::{load_seo_config, SEOConfig},
    html::{generate_html_with_seo, HtmlGenerator},
    seo_gen::{generate_sitemap, generate_rss, generate_robots_txt},
    minify::Minifier,
    analyzer::Analyzer,
    variables::load_variables,
    macros::MacroProcessor,
    watcher::DevServer,
    troubleshooting::Troubleshooter,
    BlogPost,
    BlogProcessor,
};
use eldroid_ssg::template_gen::generate_template_site;

fn walk_dir_recursive(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walk_dir_recursive(&path));
            } else if path.is_file() && path.extension().map_or(false, |ext| ext == "html" || ext == "md") {
                files.push(path);
            }
        }
    }
    files
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    // Parse command line arguments
    let args = CliArgs::parse();
    let config = BuildConfig::from(&args);

    // Initialize troubleshooter
    let cache_dir = format!("{}/cache", args.output_dir);
    let troubleshooter = Troubleshooter::new(
        cache_dir,
        args.output_dir.clone(),
    );

    // Handle troubleshooting commands first
    if let Err(e) = handle_troubleshooting(&args, &troubleshooter) {
        error!("Troubleshooting error: {}", e);
        std::process::exit(1);
    }

    // Handle subcommands
    if let Some(cmd) = &args.command {
        match cmd {
            eldroid_ssg::config::Commands::InitTemplate { target } => {
                let target_path = std::path::Path::new(target);
                match generate_template_site(target_path) {
                    Ok(_) => {
                        println!("Template site generated at {}", target_path.display());
                        std::process::exit(0);
                    },
                    Err(e) => {
                        eprintln!("Failed to generate template site: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    let perf_dir = format!("{}/performance", args.output_dir);

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

    // Load variables configuration
    let variables = match load_variables(&args.variables_config) {
        Ok(vars) => {
            info!("Variables configuration loaded successfully");
            Some(vars)
        },
        Err(e) => {
            error!("Failed to load variables configuration: {}", e);
            None
        }
    };

    // Initialize macro processor
    let macro_processor = MacroProcessor::new();

    // Ensure output directories exist
    for dir in [&args.output_dir, &perf_dir] {
        if let Err(e) = fs::create_dir_all(dir) {
            error!("Failed to create directory {}: {}", dir, e);
            std::process::exit(1);
        }
    }

    // Initialize HtmlGenerator
    let html_gen = Arc::new(
        HtmlGenerator::new()
            .with_variables(variables.unwrap_or_default())
            .with_macros(macro_processor)
    );

    // Start development server if watch mode is enabled
    if args.watch {
        // Start watcher in development mode
        let dev_server = DevServer::new(
            args.input_dir.clone(),
            args.output_dir.clone(),
            format!("{}/components", args.input_dir), // Components directory
            args.port,
            args.ws_port
        );
        
        // Process files initially
        if let Err(e) = process_files(&args, &config, &html_gen, &minifier, &analyzer, &seo_config, &perf_dir) {
            error!("Failed to process files: {}", e);
            std::process::exit(1);
        }
        
        // Start the development server
        if let Err(e) = dev_server.start().await {
            error!("Failed to start development server: {}", e);
            std::process::exit(1);
        }
    } else {
        // One-time build
        if let Err(e) = process_files(&args, &config, &html_gen, &minifier, &analyzer, &seo_config, &perf_dir) {
            error!("Failed to process files: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_troubleshooting(args: &CliArgs, troubleshooter: &Troubleshooter) -> Result<()> {
    if args.clear_cache {
        troubleshooter.clear_cache()?;
    }

    if args.check_watchers {
        troubleshooter.check_watchers()?;
    }

    if args.check_image_processor {
        troubleshooter.check_image_processor()?;
    }

    if args.verify_assets {
        troubleshooter.verify_assets(&args.input_dir)?;
    }

    if args.analyze_bundles {
        troubleshooter.analyze_bundles()?;
    }

    if args.lint {
        troubleshooter.lint(&args.input_dir)?;
    }

    if args.memory_profile {
        // Wrap the build process in memory profiling
        troubleshooter.memory_profile(|| {
            process_files(args, &BuildConfig::from(args), &Arc::new(
                HtmlGenerator::new()
                    .with_variables(load_variables(&args.variables_config).unwrap_or_default())
                    .with_macros(MacroProcessor::new())
            ), &None, &None, &None, &format!("{}/performance", args.output_dir))
        })?;
    }

    Ok(())
}

fn process_files(
    args: &CliArgs,
    config: &BuildConfig,
    html_gen: &Arc<HtmlGenerator>,
    minifier: &Option<Minifier>,
    analyzer: &Option<Analyzer>,
    seo_config: &Option<SEOConfig>,
    perf_dir: &str,
) -> Result<()> {
    let processed_files = Arc::new(Mutex::new(Vec::new()));
    let content_files = walk_dir_recursive(Path::new(&args.input_dir));
    let mut blog_processor = BlogProcessor::with_option_components(
        Path::new(&args.input_dir).to_path_buf(),
        html_gen.get_variables().clone()
    );
    
    // Load posts for next/prev navigation
    blog_processor.load_posts()?;
    
    let file_results: Vec<Result<PathBuf>> = content_files
        .par_iter()
        .map(|file_path| -> Result<PathBuf> {
            // Read content
            let content = fs::read_to_string(file_path)?;
            
            // Process content based on file type
            let processed_content = if file_path.extension().map_or(false, |ext| ext == "md") {
                let post = BlogPost::from_file(file_path, Path::new(&args.input_dir))?;
                blog_processor.process_post(&post)?
            } else if let Some(seo) = seo_config {
                generate_html_with_seo(&content, seo, html_gen)
            } else {
                html_gen.generate(&content)
            };

            // Run analysis if enabled
            if let Some(analyzer) = analyzer {
                if config.security_checks {
                    let security_report = analyzer.analyze_security(&processed_content, file_path);
                    if !security_report.mixed_content.is_empty() {
                        error!("Mixed content found in {}: {:?}", file_path.display(), security_report.mixed_content);
                    }
                    if !security_report.insecure_links.is_empty() {
                        error!("Insecure links found in {}: {:?}", file_path.display(), security_report.insecure_links);
                    }
                }
                
                if config.analyze_performance {
                    let perf_report = analyzer.analyze_performance(&processed_content, file_path);
                    let perf_file = Path::new(perf_dir)
                        .join(file_path.file_name().unwrap())
                        .with_extension("perf.txt");
                    fs::write(&perf_file, format!(
                        "Performance Analysis for {}\n\n{}\n\nRecommendations:\n{}",
                        file_path.display(),
                        perf_report.details,
                        perf_report.recommendations.join("\n")
                    ))?;
                }
            }

            // Apply minification if enabled
            let final_content = if let Some(minifier) = minifier {
                minifier.minify_html(&processed_content)
            } else {
                processed_content
            };

            // Write output file
            let out_path = Path::new(&args.output_dir)
                .join(file_path.strip_prefix(&args.input_dir)?);
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Use .html extension for markdown files
            let out_path = if file_path.extension().map_or(false, |ext| ext == "md") {
                out_path.with_extension("html")
            } else {
                out_path
            };

            fs::write(&out_path, final_content)?;
            processed_files.lock().push(out_path.clone());
            Ok(out_path)
        })
        .collect();

    // Check for errors
    let errors: Vec<_> = file_results.iter()
        .filter_map(|r| r.as_ref().err())
        .collect();
    
    if !errors.is_empty() {
        error!("Failed to process some files:");
        for err in errors {
            error!("  {}", err);
        }
        return Err(anyhow!("Some files failed to process"));
    }

    // Generate SEO files if enabled
    if config.enable_seo {
        if let Some(seo) = seo_config {
            let processed = processed_files.lock();
            generate_sitemap(&processed, seo, &args.output_dir)?;
            generate_rss(&processed, seo, &args.output_dir)?;
            generate_robots_txt(seo, &args.output_dir)?;
        }
    }

    Ok(())
}
