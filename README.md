# Documentation

Full documentation for Eldroid SSG is now available in the `docs/` directory:

- [Features](docs/features.md)
- [Component System](docs/components.md)
- [Global Macros](docs/macros.md)
- [Usage & CLI](docs/usage.md)
- [SEO & Performance](docs/seo_performance.md)
- [Development & Contribution](docs/development.md)

See the respective files for detailed information on each topic.

---

# Eldroid SSG (Summary)

Eldroid SSG is a modern, fast static site generator written in Rust. It features:
- Fast incremental builds
- SEO optimization
- Component-based architecture
- Global macros
- Security and performance analysis

For full details, see the documentation in the `docs/` directory.

# Subdirectory Support

Eldroid SSG fully supports subdirectories in both the `content/` and `components/` directories. The output directory will automatically mirror the subdirectory structure of your `content/` directory, ensuring all generated files are placed in the correct relative paths.

- You can organize your content and components in nested folders for better structure and scalability.
- When the site is built, the output directory will preserve the same folder hierarchy as your content directory.
- Component auto-registration also supports nested folders in the `components/` directory.