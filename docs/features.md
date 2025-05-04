# Features

Eldroid SSG provides a modern static site generation experience with the following features:

## Core Features

### Fast Incremental Builds
- Only rebuilds files that have changed or are affected by changes
- Smart dependency tracking between components and content
- Typical rebuild times under 100ms for single file changes
- Full build optimization in release mode

### Component-Based Architecture
- Reusable components with parameter support
- Automatic component registration from the `components/` directory
- Nested component support with infinite depth
- Circular dependency detection and prevention
- Support for component hot reloading during development
- Flexible parameter passing with type checking

### Global Macros
- Define site-wide variables and functions
- Support for conditional content generation
- Built-in date and time formatting macros
- Custom macro definitions with Rust-like syntax
- Scope-aware macro resolution

## Performance & Optimization

### SEO Optimization
- Automatic meta tag generation
- Open Graph protocol support
- Twitter Card integration
- Structured data (JSON-LD) injection
- Canonical URL management
- Sitemap.xml generation
- robots.txt configuration

### Performance Analysis
- Detailed build time metrics
- Asset size tracking
- Component render time analysis
- Waterfall diagrams for page loading
- Suggestions for performance improvements
- Bundle size analysis

### Security Checks
- Mixed content detection
- Secure header recommendations
- External link auditing
- Content Security Policy (CSP) validation
- Insecure resource detection
- SSL/TLS configuration checking

### Asset Optimization
- Automatic image optimization
  - WebP conversion
  - Responsive image generation
  - Lazy loading implementation
- CSS minification and bundling
- JavaScript optimization
  - Tree shaking
  - Code splitting
  - Module bundling
- Font subsetting and optimization
- Cache control header management

## Development Experience

### Hot Reloading
- Real-time preview server
- Instant component updates
- CSS hot reload without page refresh
- Automatic browser refresh on content changes
- WebSocket-based state synchronization

### Development Server with Hot Reloading
- WebSocket-based live reload system
- Intelligent file watching with debouncing
- Real-time component updates without page refresh
- Automatic browser synchronization
- Built-in error reporting and overlay
- Performance metrics in development
- Support for multiple file types:
  - HTML/MD content files
  - Component templates
  - Static assets (CSS, JS, images)
  - Configuration files

### Development Tools
- Build process logging
- Error reporting with source maps
- Performance profiling
- Memory usage tracking
- Network request monitoring
- Component dependency graph visualization

### Performance Monitoring
- Build-time performance tracking
- Component render time analysis
- Asset size monitoring
- Network request waterfall
- Memory usage tracking
- Cache hit rate statistics
- WebSocket connection metrics
