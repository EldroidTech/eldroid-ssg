use std::collections::HashMap;
use std::path::Path;
use std::fs;
use regex::Regex;
use once_cell::sync::Lazy;

static IMAGE_EXTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(jpg|jpeg|png|gif|webp)$").unwrap());
static IMG_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).unwrap());
static SCRIPT_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<script([^>]*)>"#).unwrap());
static LINK_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<link[^>]+rel=["']stylesheet["'][^>]*>"#).unwrap());
static META_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<meta[^>]+name=["']([^"']+)["'][^>]*>"#).unwrap());
static MIXED_CONTENT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"http://[^"'\s>]+"#).unwrap());
static FONT_LOADING: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<link[^>]+rel=["']preload["'][^>]*as=["']font["'][^>]*>"#).unwrap());
static MINIFIED_FILE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\.(min\.(js|css))"#).unwrap());
static A11Y_ELEMENTS: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<(img|a|button|input|select|textarea)([^>]*)>"#).unwrap());

#[derive(Debug)]
pub struct PageAnalysis {
    pub page_size_bytes: usize,
    pub image_count: usize,
    pub large_images: Vec<String>,
    pub unoptimized_images: Vec<String>,
    pub blocking_scripts: usize,
    pub render_blocking_css: usize,
    pub missing_meta_tags: Vec<String>,
    pub mixed_content_urls: Vec<String>,
    pub unoptimized_fonts: bool,
    pub unminified_resources: Vec<String>,
    pub a11y_issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub perf_score: f32,
}

pub struct Analyzer {
    base_path: String,
}

impl Analyzer {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn analyze_page(&self, html_content: &str, file_path: &Path) -> PageAnalysis {
        let mut analysis = PageAnalysis {
            page_size_bytes: html_content.len(),
            image_count: 0,
            large_images: Vec::new(),
            unoptimized_images: Vec::new(),
            blocking_scripts: 0,
            render_blocking_css: 0,
            missing_meta_tags: Vec::new(),
            mixed_content_urls: Vec::new(),
            unoptimized_fonts: true, // Default to true until we find preloaded fonts
            unminified_resources: Vec::new(),
            a11y_issues: Vec::new(),
            recommendations: Vec::new(),
            perf_score: 100.0,
        };

        // Find and analyze images
        for cap in IMG_TAG.captures_iter(html_content) {
            analysis.image_count += 1;
            if let Some(src) = cap.get(1) {
                self.analyze_image(src.as_str(), &mut analysis, file_path);
            }
        }

        // Count blocking scripts
        for cap in SCRIPT_TAG.captures_iter(html_content) {
            if let Some(attrs) = cap.get(1) {
                let attrs = attrs.as_str();
                if !attrs.contains("defer") && !attrs.contains("async") {
                    analysis.blocking_scripts += 1;
                }
            }
        }

        // Count render-blocking CSS
        analysis.render_blocking_css = LINK_TAG.find_iter(html_content).count();

        // Check meta tags
        let important_meta_tags = ["description", "viewport", "robots"];
        let mut found_tags: HashMap<String, bool> = HashMap::new();
        
        for cap in META_TAG.captures_iter(html_content) {
            if let Some(name) = cap.get(1) {
                found_tags.insert(name.as_str().to_string(), true);
            }
        }

        for tag in important_meta_tags.iter() {
            if !found_tags.contains_key(*tag) {
                analysis.missing_meta_tags.push(tag.to_string());
            }
        }

        // Check for mixed content (HTTP in HTTPS)
        for cap in MIXED_CONTENT.captures_iter(html_content) {
            if let Some(url) = cap.get(0) {
                analysis.mixed_content_urls.push(url.as_str().to_string());
            }
        }

        // Check font loading optimization
        if !FONT_LOADING.is_match(html_content) {
            analysis.unoptimized_fonts = true;
        }

        // Check for unminified resources
        let mut found_resources = Vec::new();
        for cap in SCRIPT_TAG.captures_iter(html_content) {
            if let Some(attrs) = cap.get(1) {
                if let Some(src) = Regex::new(r#"src=["']([^"']+)["']"#).unwrap()
                    .captures(attrs.as_str())
                    .and_then(|c| c.get(1)) {
                    found_resources.push(src.as_str().to_string());
                }
            }
        }

        for resource in found_resources {
            if !MINIFIED_FILE.is_match(&resource) {
                analysis.unminified_resources.push(resource);
            }
        }

        // Check accessibility
        for cap in A11Y_ELEMENTS.captures_iter(html_content) {
            let tag = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let attrs = cap.get(2).map(|m| m.as_str()).unwrap_or("");

            match tag {
                "img" => {
                    if !attrs.contains("alt=") {
                        analysis.a11y_issues.push(format!("Image missing alt text"));
                    }
                }
                "a" => {
                    if !attrs.contains("aria-label=") && !attrs.contains(">") {
                        analysis.a11y_issues.push(format!("Link may need aria-label"));
                    }
                }
                "button" | "input" => {
                    if !attrs.contains("aria-label=") && !attrs.contains("aria-labelledby=") {
                        analysis.a11y_issues.push(format!("{} missing aria attributes", tag));
                    }
                }
                _ => {}
            }
        }

        // Calculate performance score
        let mut score_deductions = 0.0;
        
        // Page size deductions
        if analysis.page_size_bytes > 500_000 { score_deductions += 10.0; }
        if analysis.page_size_bytes > 1_000_000 { score_deductions += 15.0; }
        
        // Image optimizations
        score_deductions += (analysis.large_images.len() as f32) * 5.0;
        score_deductions += (analysis.unoptimized_images.len() as f32) * 3.0;
        
        // Script and CSS optimizations
        score_deductions += (analysis.blocking_scripts as f32) * 4.0;
        score_deductions += (analysis.render_blocking_css as f32) * 3.0;
        
        // Mixed content and security
        score_deductions += (analysis.mixed_content_urls.len() as f32) * 10.0;
        
        // Font optimization
        if analysis.unoptimized_fonts {
            score_deductions += 5.0;
        }
        
        // Resource minification
        score_deductions += (analysis.unminified_resources.len() as f32) * 3.0;
        
        // A11y issues
        score_deductions += (analysis.a11y_issues.len() as f32) * 2.0;

        analysis.perf_score = (100.0 - score_deductions).max(0.0);

        self.generate_enhanced_recommendations(&mut analysis);
        analysis
    }

    fn analyze_image(&self, src: &str, analysis: &mut PageAnalysis, file_path: &Path) {
        if !IMAGE_EXTS.is_match(src) {
            return;
        }

        let img_path = if src.starts_with('/') {
            Path::new(&self.base_path).join(&src[1..])
        } else {
            file_path.parent()
                .unwrap_or_else(|| Path::new(""))
                .join(src)
        };

        if let Ok(metadata) = fs::metadata(&img_path) {
            let size_kb = metadata.len() / 1024;
            if size_kb > 100 {
                analysis.large_images.push(src.to_string());
            }

            // Check if image could be optimized
            if let Ok(file) = fs::File::open(&img_path) {
                if let Ok(reader) = image::ImageReader::new(std::io::BufReader::new(file))
                    .with_guessed_format() {
                    if let Ok(img) = reader.decode() {
                        let (width, height) = (img.width(), img.height());
                        if width > 1920 || height > 1080 {
                            analysis.unoptimized_images.push(format!(
                                "{} ({}x{})", src, width, height
                            ));
                        }
                    }
                }
            }
        }
    }

    fn generate_enhanced_recommendations(&self, analysis: &mut PageAnalysis) {
        if analysis.page_size_bytes > 500_000 {
            analysis.recommendations.push(
                "Page size exceeds 500KB. Consider minifying HTML, CSS, and JavaScript.".to_string()
            );
        }

        if !analysis.large_images.is_empty() {
            analysis.recommendations.push(format!(
                "Large images detected ({}). Consider compressing: {}",
                analysis.large_images.len(),
                analysis.large_images.join(", ")
            ));
        }

        if !analysis.unoptimized_images.is_empty() {
            analysis.recommendations.push(format!(
                "Images with high resolution detected. Consider resizing: {}",
                analysis.unoptimized_images.join(", ")
            ));
        }

        if analysis.blocking_scripts > 0 {
            analysis.recommendations.push(format!(
                "Found {} blocking script(s). Consider adding 'defer' or 'async' attributes.",
                analysis.blocking_scripts
            ));
        }

        if analysis.render_blocking_css > 2 {
            analysis.recommendations.push(
                "Multiple render-blocking stylesheets detected. Consider combining CSS files.".to_string()
            );
        }

        if !analysis.missing_meta_tags.is_empty() {
            analysis.recommendations.push(format!(
                "Missing important meta tags: {}",
                analysis.missing_meta_tags.join(", ")
            ));
        }

        if !analysis.mixed_content_urls.is_empty() {
            analysis.recommendations.push(format!(
                "Security Issue: Found {} HTTP resources that should use HTTPS: {}",
                analysis.mixed_content_urls.len(),
                analysis.mixed_content_urls.join(", ")
            ));
        }

        if analysis.unoptimized_fonts {
            analysis.recommendations.push(
                "Font Loading: Consider using preload for critical fonts to improve performance.".to_string()
            );
        }

        if !analysis.unminified_resources.is_empty() {
            analysis.recommendations.push(format!(
                "Resource Optimization: Found {} unminified resources. Consider minifying: {}",
                analysis.unminified_resources.len(),
                analysis.unminified_resources.join(", ")
            ));
        }

        if !analysis.a11y_issues.is_empty() {
            analysis.recommendations.push(format!(
                "Accessibility Issues: Found {} issues: {}",
                analysis.a11y_issues.len(),
                analysis.a11y_issues.join(", ")
            ));
        }

        analysis.recommendations.push(format!(
            "Overall Performance Score: {:.1}%",
            analysis.perf_score
        ));

        // Sort recommendations by priority (security > performance > best practices)
        analysis.recommendations.sort_by(|a, b| {
            let a_priority = if a.contains("Security") { 0 }
                           else if a.contains("Performance") { 1 }
                           else { 2 };
            let b_priority = if b.contains("Security") { 0 }
                           else if b.contains("Performance") { 1 }
                           else { 2 };
            a_priority.cmp(&b_priority)
        });
    }
}