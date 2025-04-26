# Global Macros

Eldroid SSG supports global macros for text replacement in all pages and components.

## How It Works
- Define macros in a `macros.toml` file in your project root.
- Use `{{MACRO_NAME}}` anywhere in your content or components.
- All macros are replaced with their values during the build process.

## Example `macros.toml`
```toml
SITE_NAME = "My Awesome Site"
YEAR = "2025"
```

## Example Usage
```html
<footer>
  &copy; {{YEAR}} {{SITE_NAME}}
</footer>
```

## Notes
- Macros are replaced everywhere, including inside components.
- If a macro is not defined, the placeholder remains unchanged.
