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

## Setting Up the Development Environment

### Prerequisites
- Rust toolchain (1.75.0 or later)
- Cargo package manager
- Node.js (optional, for asset processing)

### Local Development

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

## Development Server

### Architecture
The development server consists of two main components:
1. Static file server (HTTP)
2. WebSocket server for live reloading

### Implementation Details
- File watching using `notify` crate
- WebSocket communication for real-time updates
- Debounced rebuilds for performance
- Intelligent cache invalidation
- Memory-efficient file tracking

### Working with Hot Reload
1. Start the development server:
```bash
cargo run -- --watch --port 3000
```

2. The server will:
   - Watch for file changes in content/, components/, and static/
   - Rebuild only affected pages
   - Notify connected browsers via WebSocket
   - Display build status and errors

3. Development Features:
   - Instant feedback on file changes
   - Source map support for debugging
   - Performance metrics dashboard
   - Error overlay for build failures
   - Network request logging

## Project Structure

### Core Components
- `src/watcher.rs`: Development server and hot reload
- `src/html.rs`: HTML generation and component system
- `src/seo.rs`: SEO optimization
- `src/macros.rs`: Macro processing system

### Adding New Features

1. Create feature branch:
```bash
git checkout -b feature/new-feature
```

2. Implement changes following the project structure
3. Add tests in the appropriate test modules
4. Update documentation
5. Submit pull request

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test
```

### Test Categories
1. Unit Tests: Individual component functionality
2. Integration Tests: Component interaction
3. End-to-End Tests: Full build process
4. Development Server Tests: Hot reload functionality

## Contributing

### Guidelines
1. Follow Rust coding standards
2. Add tests for new features
3. Update documentation
4. Use descriptive commit messages

### Pull Request Process
1. Create feature branch
2. Implement changes
3. Add tests
4. Update docs
5. Submit PR

### Development Tips
- Use `RUST_LOG=debug` for detailed logging
- Test hot reload with various file types
- Verify WebSocket reconnection handling
- Check memory usage during long sessions
- Test with different project sizes

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
