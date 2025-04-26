# Component System

Eldroid SSG supports a flexible component system using custom tags:

```html
<el-component c_name="my_component" param1="foo" param2="bar" />
```

## Key Points
- **Auto-registration:** All files in the `components/` directory are registered as components by filename (without extension).
- **Nesting:** Components can include other components.
- **Circular Dependency Detection:** If a component includes itself (directly or indirectly), a warning comment is inserted to prevent infinite recursion.

## Usage
1. Place your component files (e.g., `header.html`, `footer.html`) in the `components/` directory.
2. Use `<el-component c_name="header" ... />` in your content or other components.
3. During build, components are recursively rendered and replaced in the output.

## Example
```html
<el-component c_name="header" site_title="My Site" />
<main>
  Welcome!
  <el-component c_name="footer" />
</main>
```

If a circular dependency is detected, you will see:
```html
<!-- Circular component: header -->
```
