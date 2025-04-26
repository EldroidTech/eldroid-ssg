use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use regex::Regex;
use once_cell::sync::Lazy;

static IMAGE_EXTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(jpg|jpeg|png|gif|webp)$").unwrap());

#[derive(Debug)]
pub struct PageAnalysis {
    pub page_size_bytes: usize,
    pub image_count: usize,
    pub large_images: Vec<String>,
    pub unoptimized_images: Vec<String>,
    pub blocking_scripts: usize,
    pub render_blocking_css: usize,
    pub missing_meta_tags: Vec<String>,
    pub recommendations: Vec<String>,
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
            recommendations: Vec::new(),
        };

        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html_content.as_bytes())
            .unwrap();

        self.analyze_node(&dom.document, &mut analysis, file_path);
        self.generate_recommendations(&mut analysis);

        analysis
    }

    fn analyze_node(&self, handle: &Handle, analysis: &mut PageAnalysis, file_path: &Path) {
        let node = handle;
        match node.data {
            NodeData::Element { ref name, ref attrs, .. } => {
                let tag_name = name.local.as_ref();
                let attrs = attrs.borrow();

                match tag_name {
                    "img" => {
                        analysis.image_count += 1;
                        if let Some(src) = attrs.iter().find(|attr| attr.name.local.as_ref() == "src") {
                            let src_path = src.value.to_string();
                            self.analyze_image(&src_path, analysis, file_path);
                        }
                    }
                    "script" => {
                        if !attrs.iter().any(|attr| attr.name.local.as_ref() == "defer" 
                            || attr.name.local.as_ref() == "async") {
                            analysis.blocking_scripts += 1;
                        }
                    }
                    "link" => {
                        let is_stylesheet = attrs.iter().any(|attr| 
                            attr.name.local.as_ref() == "rel" && 
                            attr.value.to_string() == "stylesheet"
                        );
                        if is_stylesheet {
                            analysis.render_blocking_css += 1;
                        }
                    }
                    "meta" => {
                        // Check for important meta tags
                        self.check_meta_tags(&attrs, analysis);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        for child in node.children.borrow().iter() {
            self.analyze_node(child, analysis, file_path);
        }
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
                if let Ok(reader) = image::io::Reader::new(std::io::BufReader::new(file))
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

    fn check_meta_tags(&self, attrs: &[html5ever::interface::Attribute], analysis: &mut PageAnalysis) {
        let important_meta_tags = ["description", "viewport", "robots"];
        let mut found_tags = HashMap::new();

        for attr in attrs {
            if attr.name.local.as_ref() == "name" {
                found_tags.insert(attr.value.to_string(), true);
            }
        }

        for tag in important_meta_tags.iter() {
            if !found_tags.contains_key(*tag) {
                analysis.missing_meta_tags.push(tag.to_string());
            }
        }
    }

    fn generate_recommendations(&self, analysis: &mut PageAnalysis) {
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
    }
}