# Eldroid SSG

A modern, blazing-fast static site generator written in Rust. It features:

- Fast incremental builds with hot reloading
- Component-based architecture
- SEO optimization
- Global macros system
- Security and performance analysis
- Live development server with WebSocket-based hot reload
- Built-in diagram support for Mermaid, Graphviz, and PlantUML in markdown
- Template generator for instant starter sites

## Quick Start

```bash
# Install Eldroid SSG
cargo install eldroid-ssg

# Generate a starter site with all features
eldroid-ssg init-template --target mysite
cd mysite

# Start development server with hot reload
eldroid-ssg --watch --port 3000
```

## Documentation

Full documentation is available in the `docs/` directory:

- [Features](docs/features.md) - Overview of all features
- [Usage & CLI](docs/usage.md) - Complete usage guide and CLI options
- [Component System](docs/components.md) - Component-based architecture
- [Global Macros](docs/macros.md) - Macro system documentation
- [SEO & Performance](docs/seo_performance.md) - SEO and performance optimization
- [Development & Contribution](docs/development.md) - Development guide
- [Diagrams](docs/diagrams.md) - Diagram support in markdown

## Key Features

### Fast Development Server
- Hot reloading with WebSocket-based live updates
- Automatic browser refresh on file changes
- Real-time error reporting
- Performance metrics

### Component System
- Reusable components with parameter support
- Automatic component registration
- Nested components with infinite depth
- Hot reloading support

### Build Optimization
- Smart incremental builds
- Asset optimization (images, CSS, JS)
- SEO enhancement
- Security checks

### SEO Features
- Automatic meta tag generation
- Structured data support
- Sitemap and robots.txt generation
- Social media tags

### Diagram Support
- Render diagrams from markdown code blocks
- Supports `mermaid`, `graphviz`/`dot`, and `plantuml`
- Configurable diagram rendering options

## Project Structure

```
my-site/
├── content/          # Content files (.html, .md)
│   ├── index.html    # Site homepage
│   └── blog/         # Blog posts
├── components/       # Reusable components
│   ├── header.html
│   └── footer.html
├── static/          # Static assets
│   ├── css/
│   ├── js/
│   └── images/
└── seo_config.toml  # SEO configuration
```

## Contributing

Contributions are welcome! See [Development & Contribution](docs/development.md) for guidelines.