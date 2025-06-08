pub mod config;
pub mod analyzer;
pub mod html;
pub mod minify;
pub mod seo;
pub mod seo_gen;
pub mod variables;
pub mod macros;
pub mod watcher;
pub mod markdown;

// Re-export commonly used types
pub use config::{CliArgs, BuildConfig};
pub use analyzer::{Analyzer, SecurityReport, PerformanceReport};
pub use html::{HtmlGenerator, generate_html_with_seo}; 
pub use minify::Minifier;
pub use seo::{SEOConfig, PageSEO, load_seo_config};
pub use seo_gen::{generate_sitemap, generate_rss, generate_robots_txt};
pub use variables::{Variables, load_variables};
pub use macros::MacroProcessor;
pub use watcher::DevServer;
pub use markdown::*;