# Usage & CLI

## Quick Start

```bash
cargo install eldroid-ssg
mkdir my-site && cd my-site
mkdir content components
eldroid-ssg --watch --port 3000
```

## Project Structure
```
my-site/
├── content/          # Your content files
├── components/       # Reusable components
├── static/           # Static assets
└── seo_config.toml   # SEO configuration
```

## CLI Options

```
eldroid-ssg [OPTIONS]

Options:
  --input-dir <DIR>           Input directory [default: content]
  --output-dir <DIR>          Output directory [default: output]
  --components-dir <DIR>      Components directory [default: components]
  --release                   Enable release mode optimizations
  --analyze-performance       Enable performance analysis
  --enable-seo                Enable SEO features
  --seo-config <FILE>         SEO configuration file [default: seo_config.toml]
  --minify                    Force minification of HTML/CSS/JS
  --security-checks           Check for mixed content and security issues
  --watch                     Enable watch mode with dev server
  --port <PORT>               Dev server port [default: random]
  -h, --help                  Print help
  -V, --version               Print version
```

## Development Mode
- Hot reloading
- Incremental builds
- Real-time performance metrics

## Production Build
- Minification
- Asset optimization
- SEO and security checks
