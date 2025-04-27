# Troubleshooting Guide

## Common Issues

### Build Failures

#### 1. Component Resolution Errors
```
Error: Failed to resolve component "header"
```

**Possible Causes:**
- Component file missing
- Incorrect component name
- Case sensitivity issues
- Path resolution problems

**Solutions:**
1. Check component exists in `components/` directory
2. Verify component name matches filename
3. Ensure correct case in component name
4. Check component path if in subdirectory

#### 2. Macro Processing Errors
```
Error: Invalid macro syntax in "page.html"
```

**Possible Causes:**
- Syntax errors in macro
- Missing macro parameters
- Undefined macros
- Circular macro references

**Solutions:**
1. Validate macro syntax
2. Check required parameters
3. Verify macro is defined
4. Check for circular dependencies

#### 3. Memory Issues
```
Error: Process out of memory
```

**Possible Causes:**
- Large number of pages
- Complex component tree
- Memory leaks
- Resource-heavy operations

**Solutions:**
1. Increase Node.js memory limit:
   ```bash
   export NODE_OPTIONS="--max-old-space-size=8192"
   ```
2. Enable incremental builds
3. Optimize component structure
4. Use lazy loading

### Runtime Issues

#### 1. Performance Problems

**Symptoms:**
- Slow build times
- High memory usage
- Browser performance issues
- Long load times

**Diagnosis:**
```bash
# Performance analysis
eldroid-ssg --analyze-performance

# Memory profiling
eldroid-ssg --memory-profile
```

**Solutions:**
1. Enable optimization features:
   ```toml
   # optimization.toml
   [build]
   optimize = true
   cache = true
   parallel = true
   ```

2. Implement lazy loading:
   ```html
   <el-component c_name="heavy_component" lazy={true} />
   ```

3. Use code splitting:
   ```html
   <el-component c_name="dynamic_import" chunk="feature" />
   ```

#### 2. Hot Reload Issues

**Symptoms:**
- Changes not reflecting
- Partial updates
- Browser not refreshing
- Build hanging

**Diagnosis:**
```bash
eldroid-ssg --watch --debug
```

**Solutions:**
1. Clear cache:
   ```bash
   eldroid-ssg --clear-cache
   ```

2. Enable verbose logging:
   ```bash
   ELDROID_LOG=debug eldroid-ssg --watch
   ```

3. Check file watchers:
   ```bash
   eldroid-ssg --check-watchers
   ```

### Asset Processing

#### 1. Image Optimization

**Symptoms:**
- Failed image processing
- Missing responsive images
- High image load times
- WebP conversion issues

**Solutions:**
1. Check image processor:
   ```bash
   eldroid-ssg --check-image-processor
   ```

2. Configure optimization:
   ```toml
   [images]
   optimize = true
   max_width = 2000
   quality = 85
   ```

3. Verify image paths:
   ```bash
   eldroid-ssg --verify-assets
   ```

#### 2. CSS/JS Processing

**Symptoms:**
- Missing styles
- JavaScript errors
- Bundle size issues
- Source map problems

**Solutions:**
1. Check bundle analysis:
   ```bash
   eldroid-ssg --analyze-bundles
   ```

2. Enable debugging:
   ```toml
   [build]
   source_maps = true
   verbose = true
   ```

3. Validate syntax:
   ```bash
   eldroid-ssg --lint
   ```

## Debug Tools

### 1. Logging

#### Enable Debug Logs
```bash
# Basic debug logging
ELDROID_LOG=debug eldroid-ssg

# Verbose logging
ELDROID_LOG=trace eldroid-ssg

# Component-specific logging
ELDROID_LOG=debug ELDROID_COMPONENT_DEBUG=true eldroid-ssg
```

#### Log Levels
- `error`: Critical errors
- `warn`: Warning conditions
- `info`: General information
- `debug`: Detailed debugging
- `trace`: Very detailed tracing

### 2. Component Debugging

#### Inspect Components
```bash
# Check component structure
eldroid-ssg --inspect-component header

# View component tree
eldroid-ssg --component-tree

# Trace component rendering
eldroid-ssg --trace-components
```

#### Profile Components
```bash
# Performance profiling
eldroid-ssg --profile-components

# Memory analysis
eldroid-ssg --analyze-memory

# Dependency check
eldroid-ssg --check-dependencies
```

### 3. Build Analysis

#### Build Information
```bash
# Full build report
eldroid-ssg --build-report

# Cache analysis
eldroid-ssg --analyze-cache

# Asset report
eldroid-ssg --asset-report
```

## Recovery Steps

### 1. Clean Build
```bash
# Clear cache
rm -rf .cache

# Remove output
rm -rf output

# Clean build
eldroid-ssg --clean
```

### 2. Cache Reset
```bash
# Clear component cache
eldroid-ssg --clear-component-cache

# Reset build cache
eldroid-ssg --reset-cache

# Clear all caches
eldroid-ssg --clear-all-caches
```

### 3. Configuration Reset
```bash
# Reset to defaults
eldroid-ssg --reset-config

# Generate new config
eldroid-ssg --init

# Validate config
eldroid-ssg --validate-config
```

## Best Practices

### 1. Development
- Use development mode
- Enable source maps
- Monitor build times
- Check memory usage
- Profile regularly

### 2. Production
- Run full optimization
- Test thoroughly
- Monitor metrics
- Check bundle sizes
- Validate output

### 3. Maintenance
- Regular updates
- Clean builds
- Cache management
- Performance monitoring
- Error tracking

## Getting Help

### 1. Documentation
- Read error messages carefully
- Check documentation
- Search known issues
- Review examples
- Check change logs

### 2. Community
- GitHub Issues
- Discord community
- Stack Overflow
- Reddit community
- Technical blog

### 3. Support
- Bug reports
- Feature requests
- Questions
- Documentation improvements
- Community contributions