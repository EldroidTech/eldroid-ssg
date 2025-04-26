use serde::Deserialize;
use std::path::PathBuf;
use clap::Parser;

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
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub release: bool,
    pub analyze_performance: bool,
    pub enable_seo: bool,
    pub minify: bool,
    pub security_checks: bool,
    pub watch: bool,
    pub port: Option<u16>,
    pub ws_port: Option<u16>,
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
        };

        // In release mode, enable security checks and minification by default
        if config.release {
            config.minify = true;
            config.security_checks = true;
        }

        config
    }
}