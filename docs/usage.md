# Usage & CLI Guide

## Quick Start

```bash
cargo install eldroid-ssg
mkdir my-site && cd my-site
mkdir content components static
eldroid-ssg --watch --port 3000
```

## Project Structure
```
my-site/
├── content/          # Content files (.html, .md)
│   ├── index.html    # Site homepage
│   ├── blog/         # Blog posts
│   └── pages/        # Static pages
├── components/       # Reusable components
│   ├── header.html
│   ├── footer.html
│   └── shared/      # Shared components
├── static/          # Static assets
│   ├── css/
│   ├── js/
│   └── images/
├── macros/         # Custom macros
│   └── site.ed
└── seo_config.toml # SEO configuration
```

## Configuration Files

### SEO Configuration
```toml
# seo_config.toml
[site]
name = "My Site"
description = "My awesome static site"
base_url = "https://example.com"
language = "en"

[meta]
keywords = ["blog", "technology", "rust"]
author = "Your Name"
twitter = "@username"

[advanced]
robots_txt = true
sitemap = true
feed = true
```

### Content Front Matter
```yaml
---
title: My First Post
description: An introduction to my blog
date: 2025-04-27
tags: ["rust", "static-site"]
template: blog
draft: false
---
```

## CLI Options

### Basic Usage
```bash
eldroid-ssg [OPTIONS]
```

### Common Options
```
--input-dir <DIR>           Input directory [default: content]
--output-dir <DIR>          Output directory [default: output]
--components-dir <DIR>      Components directory [default: components]
--port <PORT>               Dev server port [default: random]
--watch                     Enable watch mode with dev server
--release                   Enable release mode optimizations
```

### Advanced Options
```
--analyze-performance       Enable performance analysis
--enable-seo               Enable SEO features
--minify                   Force minification of HTML/CSS/JS
--security-checks          Check for mixed content and security
--seo-config <FILE>        SEO configuration file
```

## Development Mode

### Hot Reloading
The development server automatically:
- Watches for file changes
- Rebuilds affected pages
- Reloads the browser
- Reports errors in real-time

### Development Server Features
- Live reload support
- Source maps
- Performance metrics
- Error overlay
- Network request logging

## Production Build

### Optimization Features
1. **Asset Optimization**
   - Image compression
   - CSS/JS minification
   - HTML whitespace removal
   - Dead code elimination

2. **Performance**
   - Asset bundling
   - Cache optimization
   - Lazy loading
   - Critical CSS extraction

3. **SEO**
   - Meta tag generation
   - Sitemap creation
   - robots.txt generation
   - Schema.org markup

### Build Command
```bash
eldroid-ssg --release
```

## Advanced Usage

### Custom Build Scripts
Create a build script for complex workflows:
```bash
#!/bin/bash
# build.sh

# Pre-build tasks
npm run sass

# Build site
eldroid-ssg --release \
  --input-dir custom/content \
  --output-dir dist \
  --enable-seo \
  --minify

# Post-build tasks
./scripts/optimize-images.sh
```

### Environment Variables
```bash
ELDROID_ENV=production      # Set environment
ELDROID_PORT=4000          # Override port
ELDROID_LOG=debug          # Set log level
ELDROID_NO_MINIFY=1       # Disable minification
```

### Custom Templates
1. Create template in `components/`:
```html
<!-- components/blog.html -->
<article class="blog-post">
    <header>
        <h1>@{title}</h1>
        <time>@{date}</time>
    </header>
    <div class="content">
        @{yield}
    </div>
</article>
```

2. Use in content:
```html
<!-- content/blog/post.html -->
---
template: blog
title: My Post
date: 2025-04-27
---
<p>Content goes here...</p>
```

### Asset Processing
- Images are automatically optimized
- CSS is processed with PostCSS
- JavaScript is bundled and minified
- Fonts are subset and optimized

### Production Deployment
1. Build the site:
```bash
eldroid-ssg --release --enable-seo --minify
```

2. Verify the build:
```bash
eldroid-ssg --analyze-performance
```

3. Deploy the `output/` directory

## Troubleshooting

### Common Issues
1. **Build Errors**
   - Check file permissions
   - Verify component syntax
   - Review macro usage

2. **Performance Issues**
   - Enable performance analysis
   - Check asset sizes
   - Review component dependencies

3. **SEO Problems**
   - Validate seo_config.toml
   - Check meta tags
   - Verify canonical URLs

### Debug Mode
Enable debug logging:
```bash
RUST_LOG=debug eldroid-ssg --watch
```

### Support
- GitHub Issues: Report bugs and feature requests
- Documentation: Read the full docs
- Community: Join our Discord server
