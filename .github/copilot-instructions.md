# GPUI Component AI Development Guide

This project is a **UI component library for GPUI** - a cross-platform desktop UI framework in Rust. It provides 60+ components inspired by shadcn/ui and native desktop controls.

## Architecture Overview

### Crate Structure

- `crates/ui/` - Main component library (`gpui-component`)
- `crates/story/` - Component gallery/examples for development
- `crates/assets/` - Optional bundled icon assets
- `crates/macros/` - Procedural macros
- `examples/` - Standalone example applications

### Core Pattern: RenderOnce Components

All components implement the `RenderOnce` trait, not `Render`. They are **stateless** and configured via builder pattern:

```rust
Button::new("id")
    .primary()
    .size(Size::Large)
    .label("Click me")
    .on_click(|_, _, _| println!("clicked"))
```

### Required Initialization

**CRITICAL**: Always call `gpui_component::init(cx)` before using any components. This initializes themes, global state, and key bindings.

## Essential Patterns

### Root Component Requirement

Every window must wrap its content in a `Root` component:

```rust
cx.new(|cx| Root::new(view, window, cx))
```

### Theme System

- Access theme via `cx.theme()` (requires `use ActiveTheme`)
- Themes support light/dark modes with automatic system sync
- Use theme colors: `cx.theme().foreground`, `cx.theme().primary`, etc.
- Pre-built themes in `themes/` directory (catppuccin, gruvbox, etc.)

### Size System

Components support consistent sizing: `xs`, `sm`, `md`, `lg`

```rust
Button::new("id").size(Size::Large)
Input::new("field").size(Size::Small)
```

### Component Variants

Most components have semantic variants:

```rust
Button::new("save").primary()     // Blue primary button
Button::new("delete").danger()    // Red destructive button
Alert::new().warning()            // Yellow warning alert
```

## Development Workflow

### Running the Component Gallery

```bash
cargo run  # Launches the story app with all component examples
```

### Running Examples

```bash
cargo run --example hello_world
cargo run --example input
```

### Feature Flags

- `webview` - Enables WebView component (experimental)
- `tree-sitter-languages` - Syntax highlighting for editor/markdown
- `decimal` - Decimal number support
- `inspector` - Debug UI inspector

### Testing Components

Add new component stories in `crates/story/src/` following existing patterns. The story app serves as both documentation and visual regression testing.

## Key Integration Points

### GPUI Dependencies

- Built on GPUI 0.2.2 framework
- Uses Zed's custom forks: `zed-reqwest`, `zed-sum-tree`
- Text handling via `ropey` rope data structure

### Styling Extensions

- Custom styled extensions in `src/styled.rs`
- Shorthand functions: `h_flex()`, `v_flex()` instead of `div().h_flex()`
- Theme-aware styling via `ActiveTheme` trait

### State Management

- Global state for text views and other shared state
- Component state typically managed in parent views
- Use GPUI's context system for communication

### Asset Management

- Icons referenced by `IconName` enum
- SVG assets loaded from `assets/icons/` directory
- Use `gpui-component-assets` crate for defaults or provide your own

## Anti-Patterns to Avoid

- **Don't** implement `Render` trait on components - use `RenderOnce`
- **Don't** forget to call `gpui_component::init(cx)` - components won't work
- **Don't** skip the `Root` wrapper - window-level features will break
- **Don't** hardcode colors - use theme system: `cx.theme().primary`
- **Don't** create stateful components - keep state in parent views

## Common Tasks

### Adding New Components

1. Create module in `crates/ui/src/[component]/`
2. Implement `RenderOnce` with builder pattern
3. Add to `lib.rs` exports
4. Create story in `crates/story/src/[component]_story.rs`
5. Register story in story app

### Customizing Themes

- Modify theme JSON files in `themes/`
- Use `ThemeRegistry` to register custom themes
- Override specific colors via `ThemeColor` struct

### Working with Layout

- Dock layouts for resizable panels: `dock::Dock`
- Virtualized lists/tables for large datasets
- Flex layouts via `h_flex()`, `v_flex()` helpers
