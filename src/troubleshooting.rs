use std::path::Path;
use anyhow::{Result, anyhow};
use log::{info, warn};
use std::fs;
use image::GenericImageView;

pub struct Troubleshooter {
    cache_dir: String,
    output_dir: String,
}

impl Troubleshooter {
    pub fn new(cache_dir: String, output_dir: String) -> Self {
        Self {
            cache_dir,
            output_dir,
        }
    }

    pub fn clear_cache(&self) -> Result<()> {
        info!("Clearing build cache...");
        if Path::new(&self.cache_dir).exists() {
            fs::remove_dir_all(&self.cache_dir)?;
            fs::create_dir_all(&self.cache_dir)?;
            info!("Cache cleared successfully");
        } else {
            warn!("No cache directory found at {}", self.cache_dir);
        }
        Ok(())
    }

    pub fn check_watchers(&self) -> Result<()> {
        let max_watchers = fs::read_to_string("/proc/sys/fs/inotify/max_user_watches")
            .unwrap_or_else(|_| String::from("unknown"));
        
        info!("File Watcher Status:");
        info!("  Max user watches: {}", max_watchers.trim());
        
        // Check if number of watches is too low
        if let Ok(watches) = max_watchers.trim().parse::<i32>() {
            if watches < 8192 {
                warn!("Low inotify watch limit detected. Consider increasing it:");
                warn!("  echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf");
                warn!("  sudo sysctl -p");
            }
        }
        
        Ok(())
    }

    pub fn check_image_processor(&self) -> Result<()> {
        info!("Checking image processing capabilities...");
        
        let checks = vec![
            ("imagemagick", "convert -version"),
            ("sharp", "npm list sharp"),
            ("libvips", "vips -v"),
        ];

        for (name, cmd) in checks {
            match std::process::Command::new("sh")
                .args(["-c", cmd])
                .output() {
                Ok(_) => info!("✓ {} is available", name),
                Err(_) => warn!("✗ {} is not installed", name),
            }
        }

        Ok(())
    }

    pub fn verify_assets(&self, input_dir: &str) -> Result<()> {
        info!("Verifying static assets...");
        
        let static_dir = Path::new(input_dir).join("static");
        if !static_dir.exists() {
            return Err(anyhow!("Static directory not found at {}", static_dir.display()));
        }

        let mut issues = Vec::new();
        
        // Walk through static directory
        for entry in walkdir::WalkDir::new(&static_dir)
            .into_iter()
            .filter_map(|e| e.ok()) {
                
            let path = entry.path();
            if path.is_file() {
                // Check file size
                if let Ok(metadata) = path.metadata() {
                    let size = metadata.len();
                    if size > 5_000_000 {  // 5MB
                        issues.push(format!("Large file detected: {} ({:.1}MB)", 
                            path.display(), size as f64 / 1_000_000.0));
                    }
                }
                
                // Check image dimensions for common formats
                if let Some(ext) = path.extension() {
                    if matches!(ext.to_str(), Some("jpg" | "jpeg" | "png" | "webp")) {
                        if let Ok(img) = image::open(path) {
                            let dims = img.dimensions();
                            if dims.0 > 2000 || dims.1 > 2000 {
                                issues.push(format!("Large image dimensions: {} ({}x{})", 
                                    path.display(), dims.0, dims.1));
                            }
                        }
                    }
                }
            }
        }

        if issues.is_empty() {
            info!("No asset issues found");
        } else {
            warn!("Asset issues found:");
            for issue in issues {
                warn!("  - {}", issue);
            }
        }

        Ok(())
    }

    pub fn analyze_bundles(&self) -> Result<()> {
        info!("Analyzing build bundles...");
        
        let mut total_size = 0;
        let mut bundles = Vec::new();
        
        // Walk through output directory
        for entry in walkdir::WalkDir::new(&self.output_dir)
            .into_iter()
            .filter_map(|e| e.ok()) {
                
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    let size = metadata.len();
                    total_size += size;
                    bundles.push((path.to_path_buf(), size));
                }
            }
        }
        
        // Sort bundles by size
        bundles.sort_by(|a, b| b.1.cmp(&a.1));
        
        info!("Bundle Analysis:");
        info!("  Total bundle size: {:.1}MB", total_size as f64 / 1_000_000.0);
        info!("  Largest bundles:");
        for (path, size) in bundles.iter().take(5) {
            info!("    - {}: {:.1}KB", 
                path.strip_prefix(&self.output_dir).unwrap().display(),
                *size as f64 / 1_000.0);
        }
        
        Ok(())
    }

    pub fn lint(&self, input_dir: &str) -> Result<()> {
        info!("Running code quality checks...");
        
        let mut issues = Vec::new();
        
        // Walk through content files
        for entry in walkdir::WalkDir::new(input_dir)
            .into_iter()
            .filter_map(|e| e.ok()) {
                
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    match ext.to_str() {
                        Some("html") => {
                            // Basic HTML validation
                            if let Ok(content) = fs::read_to_string(path) {
                                if content.contains("javascript:") {
                                    issues.push(format!("{}: Unsafe javascript: protocol usage", path.display()));
                                }
                                if content.contains("http:") {
                                    issues.push(format!("{}: Mixed content (http:// links)", path.display()));
                                }
                            }
                        },
                        Some("css") => {
                            // Basic CSS validation
                            if let Ok(content) = fs::read_to_string(path) {
                                if content.contains("!important") {
                                    issues.push(format!("{}: Use of !important", path.display()));
                                }
                            }
                        },
                        Some("md") => {
                            // Basic Markdown validation
                            if let Ok(content) = fs::read_to_string(path) {
                                if !content.starts_with("# ") {
                                    issues.push(format!("{}: Missing top-level heading", path.display()));
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        
        if issues.is_empty() {
            info!("No linting issues found");
        } else {
            warn!("Linting issues found:");
            for issue in issues {
                warn!("  - {}", issue);
            }
        }
        
        Ok(())
    }

    pub fn memory_profile<F>(&self, build_fn: F) -> Result<()> 
    where F: FnOnce() -> Result<()>
    {
        info!("Starting memory profiling...");
        
        let start_mem = get_memory_usage()?;
        info!("Initial memory usage: {:.1}MB", start_mem as f64 / 1_000_000.0);
        
        // Run the build
        let start = std::time::Instant::now();
        build_fn()?;
        let duration = start.elapsed();
        
        let end_mem = get_memory_usage()?;
        info!("Final memory usage: {:.1}MB", end_mem as f64 / 1_000_000.0);
        info!("Memory delta: {:.1}MB", (end_mem - start_mem) as f64 / 1_000_000.0);
        info!("Build time: {:.2}s", duration.as_secs_f64());
        
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn get_memory_usage() -> Result<u64> {
    let status = fs::read_to_string("/proc/self/status")?;
    for line in status.lines() {
        if line.starts_with("VmRSS:") {
            if let Some(kb) = line.split_whitespace().nth(1) {
                return Ok(kb.parse::<u64>()? * 1024);
            }
        }
    }
    Err(anyhow!("Could not find VmRSS in /proc/self/status"))
}

#[cfg(not(target_os = "linux"))]
fn get_memory_usage() -> Result<u64> {
    sys_info::mem_info()
        .map(|info| info.total)
        .map_err(|e| anyhow!("Failed to get memory info: {}", e))
}
