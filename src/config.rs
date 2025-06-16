use serde::Deserialize;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Input directory containing content files
    #[arg(long, default_value = "content")]
    pub input_dir: String,

    /// Output directory for generated files
    #[arg(long, default_value = "output")]
    pub output_dir: String,

    /// Components directory
    #[arg(long, default_value = "components")]
    pub components_dir: String,

    /// Variables configuration file path
    #[arg(long, default_value = "variables.toml")]
    pub variables_config: PathBuf,

    /// Release mode with additional optimizations
    #[arg(long)]
    pub release: bool,

    /// Enable performance analysis
    #[arg(long)]
    pub analyze_performance: bool,

    /// Enable SEO features
    #[arg(long)]
    pub enable_seo: bool,

    /// SEO configuration file path
    #[arg(long, default_value = "seo_config.toml")]
    pub seo_config: PathBuf,

    /// Force minification of HTML/CSS/JS
    #[arg(long)]
    pub minify: bool,

    /// Check for mixed content and security issues
    #[arg(long)]
    pub security_checks: bool,

    /// Enable watch mode with development server
    #[arg(long)]
    pub watch: bool,

    /// Development server port (random if not specified)
    #[arg(long)]
    pub port: Option<u16>,

    /// Live reload WebSocket port (random if not specified)
    #[arg(long)]
    pub ws_port: Option<u16>,

    /// Clear build cache and temporary files
    #[arg(long)]
    pub clear_cache: bool,

    /// Check status of file watchers
    #[arg(long)]
    pub check_watchers: bool,

    /// Verify image processor setup and capabilities
    #[arg(long)]
    pub check_image_processor: bool,

    /// Check integrity and references of static assets
    #[arg(long)]
    pub verify_assets: bool,

    /// Analyze build bundle sizes and dependencies
    #[arg(long)]
    pub analyze_bundles: bool,

    /// Run code quality and style checks
    #[arg(long)]
    pub lint: bool,

    /// Profile memory usage during build
    #[arg(long)]
    pub memory_profile: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a starter template site with sample pages, components, and blogs
    InitTemplate {
        /// Target directory for the generated site
        #[arg(long, default_value = "sample-site")]
        target: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    #[serde(default)]
    pub release: bool,
    #[serde(default)]
    pub analyze_performance: bool,
    #[serde(default)]
    pub enable_seo: bool,
    #[serde(default)]
    pub minify: bool,
    #[serde(default)]
    pub security_checks: bool,
    #[serde(default)]
    pub watch: bool,
    pub port: Option<u16>,
    pub ws_port: Option<u16>,
    pub variables_config: PathBuf,
    #[serde(default)]
    pub clear_cache: bool,
    #[serde(default)]
    pub check_watchers: bool,
    #[serde(default)]
    pub check_image_processor: bool,
    #[serde(default)]
    pub verify_assets: bool,
    #[serde(default)]
    pub analyze_bundles: bool,
    #[serde(default)]
    pub lint: bool,
    #[serde(default)]
    pub memory_profile: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            release: false,
            analyze_performance: false,
            enable_seo: false,
            minify: false,
            security_checks: false,
            watch: false,
            port: None,
            ws_port: None,
            variables_config: PathBuf::from("variables.toml"),
            clear_cache: false,
            check_watchers: false,
            check_image_processor: false,
            verify_assets: false,
            analyze_bundles: false,
            lint: false,
            memory_profile: false,
        }
    }
}

impl From<&CliArgs> for BuildConfig {
    fn from(args: &CliArgs) -> Self {
        let mut config = BuildConfig {
            release: args.release,
            analyze_performance: args.analyze_performance,
            enable_seo: args.enable_seo,
            minify: args.minify,
            security_checks: args.security_checks,
            watch: args.watch,
            port: args.port,
            ws_port: args.ws_port,
            variables_config: args.variables_config.clone(),
            clear_cache: args.clear_cache,
            check_watchers: args.check_watchers,
            check_image_processor: args.check_image_processor,
            verify_assets: args.verify_assets,
            analyze_bundles: args.analyze_bundles,
            lint: args.lint,
            memory_profile: args.memory_profile,
        };

        // In release mode, enable security checks and minification by default
        if config.release {
            config.minify = true;
            config.security_checks = true;
        }

        config
    }
}