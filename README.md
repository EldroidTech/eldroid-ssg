# Eldroid SSG

A modern, fast static site generator written in Rust with built-in optimization features.

## Features

- ⚡ Fast incremental builds with parallel processing
- 🔍 Built-in SEO optimization
- 🚀 Automatic minification in release mode
- 📊 Performance analysis and recommendations
- 🔒 Security checks for mixed content and best practices
- 📱 Development server with hot reloading
- 🎯 Component-based architecture

## Quick Start

```bash
# Install from cargo
cargo install eldroid-ssg

# Create a new site
mkdir my-site && cd my-site
mkdir content components

# Start development server
eldroid-ssg --watch --port 3000

# Build for production
eldroid-ssg --release
```

## Project Structure

```
my-site/
├── content/          # Your content files
├── components/       # Reusable components
├── static/          # Static assets
└── seo_config.toml  # SEO configuration
```

## Configuration

### CLI Options

```bash
eldroid-ssg [OPTIONS]

Options:
  --input-dir <DIR>           Input directory [default: content]
  --output-dir <DIR>          Output directory [default: output]
  --components-dir <DIR>      Components directory [default: components]
  --release                   Enable release mode optimizations
  --analyze-performance       Enable performance analysis
  --enable-seo               Enable SEO features
  --seo-config <FILE>        SEO configuration file [default: seo_config.toml]
  --minify                   Force minification of HTML/CSS/JS
  --security-checks          Check for mixed content and security issues
  --watch                    Enable watch mode with dev server
  --port <PORT>             Dev server port [default: random]
  -h, --help                Print help
  -V, --version             Print version
```

### SEO Configuration

Create a `seo_config.toml` file:

```toml
site_name = "My Awesome Site"
base_url = "https://example.com"
default_description = "Welcome to my website"
default_keywords = ["blog", "technology", "rust"]
twitter_handle = "@myhandle"
facebook_app_id = "123456789"
google_site_verification = "verification_code"
```

### Page-specific SEO

Add SEO metadata to your pages using HTML comments:

```html
<!-- SEO {
  "title": "My Page Title",
  "description": "Page specific description",
  "keywords": ["page", "specific", "keywords"],
  "url": "/my-page",
  "canonical_url": "https://example.com/my-page",
  "structured_data": "{\"@type\": \"Article\", ...}"
} -->
<html>
...
</html>
```

## Development Mode

Run the development server with hot reloading:

```bash
eldroid-ssg --watch --port 3000
```

The development server will:
- Watch for changes in content and component files
- Incrementally rebuild only changed files
- Automatically reload connected browsers
- Provide real-time performance metrics

## Production Build

For production deployment:

```bash
eldroid-ssg --release --enable-seo
```

This will:
- Minify all HTML, CSS, and JavaScript
- Generate sitemap.xml and robots.txt
- Create an RSS feed
- Run security checks
- Optimize assets
- Generate performance reports

## Performance Analysis

When using `--analyze-performance`, you'll get detailed reports including:
- Page size and load time estimates
- Resource optimization suggestions
- Image optimization status
- Script loading optimization tips
- Overall performance score

## Security Checks

With `--security-checks` enabled, the generator will check for:
- Mixed content warnings
- Insecure external resources
- Inline script usage
- Best practice violations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/eldroid-ssg.git
cd eldroid-ssg
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

### Running Tests

The project includes:
- Unit tests for core functionality
- Integration tests for the full build process
- Performance benchmarks
- Security test cases

Run specific test categories:
```bash
cargo test --test unit_tests
cargo test --test integration_tests
cargo test --test security_tests
```

## License

MIT License

## Acknowledgments

- Built with Rust 🦀
- Uses various awesome open source libraries