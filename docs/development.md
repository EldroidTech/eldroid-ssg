# Development Guide

## Project Architecture

### Core Components
```
src/
├── main.rs         # CLI and program entry point
├── lib.rs          # Core library functionality
├── html.rs         # HTML generation and manipulation
├── seo.rs          # SEO functionality
├── analyzer.rs     # Performance analysis
├── config.rs       # Configuration management
├── minify.rs       # Asset minification
├── watcher.rs      # File watching and hot reload
└── seo_gen.rs      # SEO tag generation
```

### Component Pipeline
1. **Input Processing**
   - Parse HTML/component files
   - Extract frontmatter
   - Process macros
   
2. **Component Resolution**
   - Register components
   - Build dependency graph
   - Detect circular dependencies
   
3. **Optimization**
   - Minify assets
   - Generate responsive images
   - Bundle resources
   
4. **Output Generation**
   - Inject SEO tags
   - Apply security headers
   - Generate final HTML

## Contributing

### Setting Up Development Environment
1. Install Rust and Cargo
2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/eldroid-ssg.git
   cd eldroid-ssg
   ```
3. Install development dependencies:
   ```bash
   cargo install cargo-watch cargo-edit cargo-audit
   ```

### Development Workflow
1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Run tests while developing:
   ```bash
   cargo watch -x test
   ```

3. Run the development server:
   ```bash
   cargo run -- --watch --port 3000
   ```

### Testing
- Unit tests are in each module file
- Integration tests are in `tests/`
- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`
- Generate test coverage: `cargo tarpaulin`

### Performance Testing
```bash
cargo run --release -- --analyze-performance --input-dir examples/large-site
```

### Debug Logging
Enable debug logs with environment variables:
```bash
RUST_LOG=debug cargo run
```

Log levels:
- `error`: Critical errors
- `warn`: Warning conditions
- `info`: General information
- `debug`: Detailed information
- `trace`: Very detailed debugging

### Profiling
1. CPU profiling:
   ```bash
   cargo run --release -- --profile-cpu
   ```

2. Memory profiling:
   ```bash
   cargo run --release -- --profile-memory
   ```

3. View flamegraphs:
   ```bash
   cargo flamegraph
   ```

## Architecture Deep Dive

### HTML Processing
The HTML processing pipeline:
1. Parse input HTML using `scraper`
2. Build DOM tree
3. Process component tags
4. Apply transformations
5. Generate final HTML

### Component System
Components are processed through:
1. Registration phase
2. Dependency resolution
3. Parameter validation
4. Recursive rendering
5. Cache management

### Performance Optimization
- Incremental build system
- Component level caching
- Parallel processing
- Smart asset bundling
- Lazy evaluation

### Security Features
- Input sanitization
- Resource validation
- Header management
- CSP generation
- Mixed content detection

## Debugging

### Common Issues
1. Circular Dependencies
   - Check component imports
   - Use dependency graph visualization
   - Enable trace logging

2. Performance Problems
   - Run with `--analyze-performance`
   - Check component render times
   - Monitor memory usage
   - Review asset sizes

3. Build Failures
   - Check syntax errors
   - Validate component parameters
   - Review macro usage
   - Check file permissions

### Debugging Tools
1. VS Code Configuration
   ```json
   {
     "version": "0.2.0",
     "configurations": [
       {
         "type": "lldb",
         "request": "launch",
         "name": "Debug eldroid-ssg",
         "cargo": {
           "args": ["build"]
         },
         "args": ["--watch", "--port", "3000"]
       }
     ]
   }
   ```

2. Environment Variables
   ```bash
   ELDROID_DEBUG=1
   RUST_BACKTRACE=1
   RUST_LOG=debug
   ```

### Performance Monitoring
Enable detailed metrics:
```toml
# Config.toml
[development]
metrics_enabled = true
log_level = "debug"
profile_memory = true
trace_components = true
```
