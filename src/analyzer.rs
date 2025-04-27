use scraper::{Html, Selector};
use url::Url;
use std::collections::HashSet;
use std::path::Path;

pub struct SecurityReport {
    pub mixed_content: Vec<String>,
    pub insecure_links: Vec<String>,
    pub inline_scripts: Vec<String>,
    pub external_resources: Vec<String>,
}

pub struct PerformanceReport {
    pub details: String,
    pub recommendations: Vec<String>,
}

pub struct Analyzer {
    base_url: Option<String>,
}

impl Analyzer {
    pub fn new(base_url: Option<String>) -> Self {
        Self { base_url }
    }

    pub fn analyze_security(&self, html: &str, file_path: &Path) -> SecurityReport {
        let document = Html::parse_document(html);
        let mut report = SecurityReport {
            mixed_content: Vec::new(),
            insecure_links: Vec::new(),
            inline_scripts: Vec::new(),
            external_resources: Vec::new(),
        };

        // Check for mixed content
        if let Some(base) = &self.base_url {
            if base.starts_with("https") {
                let selectors = [
                    ("img[src]", "src"),
                    ("script[src]", "src"),
                    ("link[href]", "href"),
                    ("iframe[src]", "src"),
                ];

                for (sel, attr) in selectors.iter() {
                    if let Ok(selector) = Selector::parse(sel) {
                        for element in document.select(&selector) {
                            if let Some(url) = element.value().attr(attr) {
                                if url.starts_with("http://") {
                                    report.mixed_content.push(url.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check for inline scripts
        if let Ok(script_selector) = Selector::parse("script:not([src])") {
            for script in document.select(&script_selector) {
                if !script.inner_html().trim().is_empty() {
                    report.inline_scripts.push(file_path.display().to_string());
                    break;
                }
            }
        }

        // Check external resources
        let external_selectors = [
            ("script[src]", "src"),
            ("link[href]", "href"),
            ("img[src]", "src"),
        ];

        let mut seen = HashSet::new();
        for (sel, attr) in external_selectors.iter() {
            if let Ok(selector) = Selector::parse(sel) {
                for element in document.select(&selector) {
                    if let Some(url) = element.value().attr(attr) {
                        if url.starts_with("http") && seen.insert(url) {
                            report.external_resources.push(url.to_string());

                            // Check for insecure links
                            if let Ok(parsed_url) = Url::parse(url) {
                                if parsed_url.scheme() == "http" {
                                    report.insecure_links.push(url.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        report
    }

    pub fn analyze_performance(&self, content: &str, _file_path: &Path) -> PerformanceReport {
        let document = Html::parse_document(content);
        let mut details = String::new();
        let mut recommendations = Vec::new();

        // Analyze page size
        let content_size = content.len();
        details.push_str(&format!("Page size: {:.2} KB\n", content_size as f64 / 1024.0));
        
        if content_size > 500_000 {
            recommendations.push("Page size exceeds 500KB. Consider optimizing images and removing unused resources.".to_string());
        }

        // Check image optimization
        if let Ok(selector) = Selector::parse("img") {
            let mut img_count = 0;
            let mut unoptimized = 0;
            
            for img in document.select(&selector) {
                img_count += 1;
                
                // Check for width/height attributes
                if img.value().attr("width").is_none() || img.value().attr("height").is_none() {
                    unoptimized += 1;
                }
            }

            details.push_str(&format!("Images: {} ({}% optimized)\n", 
                img_count,
                if img_count > 0 {
                    ((img_count - unoptimized) * 100) / img_count
                } else {
                    100
                }
            ));

            if unoptimized > 0 {
                recommendations.push(format!(
                    "Add width and height attributes to {} images to prevent layout shifts.",
                    unoptimized
                ));
            }
        }

        // Check resource loading
        if let Ok(selector) = Selector::parse("script:not([async]):not([defer])") {
            let blocking_scripts: Vec<_> = document.select(&selector).collect();
            if !blocking_scripts.is_empty() {
                details.push_str(&format!("Blocking scripts: {}\n", blocking_scripts.len()));
                recommendations.push("Add async or defer to non-critical scripts.".to_string());
            }
        }

        // Check render-blocking CSS
        if let Ok(selector) = Selector::parse("link[rel='stylesheet']") {
            let css_files: Vec<_> = document.select(&selector).collect();
            details.push_str(&format!("CSS files: {}\n", css_files.len()));
            
            if css_files.len() > 3 {
                recommendations.push("Consider combining CSS files to reduce HTTP requests.".to_string());
            }
        }

        // Calculate performance score
        let score = self.calculate_performance_score(&document);
        details.push_str(&format!("Performance score: {}/100\n", score));

        if score < 70 {
            recommendations.push("Overall performance needs improvement. Consider implementing the above recommendations.".to_string());
        }

        PerformanceReport { details, recommendations }
    }

    fn calculate_performance_score(&self, document: &Html) -> u32 {
        let mut score = 100;

        // Deduct points for various performance issues
        if let Ok(selector) = Selector::parse("script") {
            let script_count = document.select(&selector).count();
            if script_count > 10 {
                score -= ((script_count - 10) * 2).min(30) as u32;
            }
        }

        if let Ok(selector) = Selector::parse("link[rel='stylesheet']") {
            let css_count = document.select(&selector).count();
            if css_count > 3 {
                score -= ((css_count - 3) * 5).min(20) as u32;
            }
        }

        if let Ok(selector) = Selector::parse("img:not([loading='lazy'])") {
            let unlazy_images = document.select(&selector).count();
            if unlazy_images > 3 {
                score -= ((unlazy_images - 3) * 3).min(15) as u32;
            }
        }

        score
    }
}