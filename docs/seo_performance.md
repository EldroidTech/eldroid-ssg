# SEO & Performance Guide

## SEO Configuration

### Basic Setup
```toml
# seo_config.toml
site_name = "My Awesome Site"
default_description = "A site built with Eldroid SSG"
default_keywords = ["static", "site", "generator", "rust"]
google_site_verification = "your-verification-id"
```

### Per-Page SEO
Add SEO metadata to any page using HTML meta tags or front matter:

```html
<!-- content/about.html -->
<meta name="description" content="About our company and mission" />
<meta name="keywords" content="about, company, mission, team" />
<link rel="canonical" href="https://example.com/about" />
```

### Structured Data
Add JSON-LD structured data for rich search results:

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "Your Article Title",
  "author": {
    "@type": "Person",
    "name": "Author Name"
  }
}
</script>
```

### Social Media Integration
Open Graph and Twitter Card tags are automatically generated from your SEO configuration:

```html
<meta property="og:title" content="Page Title" />
<meta property="og:description" content="Page description" />
<meta property="og:image" content="https://example.com/image.jpg" />
<meta name="twitter:card" content="summary_large_image" />
```

## Performance Optimization

### Asset Optimization
Eldroid automatically optimizes assets in release mode:

1. Images
   - Converts to WebP when supported
   - Generates responsive sizes
   - Adds loading="lazy" for images below the fold
   - Optimizes quality vs size ratio

2. Stylesheets
   - Minifies CSS
   - Removes unused styles
   - Inlines critical CSS
   - Defers non-critical styles

3. JavaScript
   - Tree shaking to remove unused code
   - Code splitting for optimal loading
   - Async loading when possible
   - Source map generation in development

### Caching Strategy
Eldroid implements optimal caching with:

```nginx
# Example Nginx configuration
location /static/ {
    expires 1y;
    add_header Cache-Control "public, no-transform";
}

location /assets/ {
    expires 1M;
    add_header Cache-Control "public, no-transform";
}
```

### Performance Monitoring
Enable performance analysis with:

```bash
eldroid-ssg --analyze-performance
```

This generates reports including:
- Page load metrics
- Asset size breakdown
- Component render times
- Network waterfall diagrams
- Optimization suggestions

### Best Practices
1. Image Optimization
   - Use appropriate image formats
   - Implement responsive images
   - Optimize image quality
   - Enable lazy loading

2. Resource Loading
   - Minimize render-blocking resources
   - Implement resource hints
   - Use HTTP/2 server push
   - Enable Brotli compression

3. Component Performance
   - Keep components small and focused
   - Avoid deep nesting
   - Use lazy loading for heavy components
   - Implement code splitting

## Security Headers
Eldroid automatically adds security headers in production:

```http
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Content-Type-Options: nosniff
X-Frame-Options: SAMEORIGIN
Content-Security-Policy: default-src 'self'
Referrer-Policy: strict-origin-when-cross-origin
```

## Monitoring and Analytics
Enable built-in monitoring:

```toml
# seo_config.toml
[monitoring]
enable_analytics = true
performance_budget = true
error_tracking = true
```

This provides:
- Real-time performance metrics
- Error tracking and reporting
- User behavior analytics
- Performance budget alerts

## Development Server Performance

### Real-time Monitoring
Enable real-time performance monitoring in development:

```toml
# seo_config.toml
[monitoring]
enable_dev_metrics = true
watch_memory = true
track_rebuild_time = true
log_performance = true
```

### Metrics Dashboard
The development server provides:
- Component render times
- Build duration tracking
- Memory usage graphs
- WebSocket connection status
- File change statistics
- Cache hit rates

### Performance Budgets
Set performance budgets for development:

```toml
[performance.budget]
total_bundle_size = "500kb"
page_load_time = "300ms"
first_paint = "100ms"
rebuild_time = "50ms"
```

### Debug Logging
Enable detailed performance logging:

```bash
RUST_LOG=debug eldroid-ssg --watch --analyze-performance
```
