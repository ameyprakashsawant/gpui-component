# Breadcrumb

A navigation component that shows the current location within a hierarchical structure, allowing users to understand where they are and navigate back to parent levels.

## Features

- Hierarchical navigation display
- Multiple separator styles (slash, chevron, dot, custom icons)
- Size variants (xs, sm, md, lg)
- Maximum items with ellipsis truncation
- Icon support for breadcrumb items
- Interactive navigation with click handlers
- Disabled state support
- Keyboard navigation

## Usage

### Basic Breadcrumb

```rust
use gpui_component::{Breadcrumb, BreadcrumbItem};

Breadcrumb::new()
    .child(BreadcrumbItem::new("Home"))
    .child(BreadcrumbItem::new("Documents"))
    .child(BreadcrumbItem::new("Current File"))
```

### Interactive Navigation

```rust
Breadcrumb::new()
    .child(
        BreadcrumbItem::new("Home")
            .on_click(|_, _, cx| {
                // Navigate to home
                println!("Navigate to home");
            })
    )
    .child(
        BreadcrumbItem::new("Documents")
            .on_click(|_, _, cx| {
                // Navigate to documents
                println!("Navigate to documents");
            })
    )
    .child(BreadcrumbItem::new("Current File"))
```

### With Icons

```rust
use gpui_component::IconName;

Breadcrumb::new()
    .child(
        BreadcrumbItem::new("Home")
            .icon(IconName::Building2)
            .on_click(|_, _, cx| { /* navigate */ })
    )
    .child(
        BreadcrumbItem::new("Documents")
            .icon(IconName::Folder)
            .on_click(|_, _, cx| { /* navigate */ })
    )
    .child(
        BreadcrumbItem::new("Current File")
            .icon(IconName::File)
    )
```

### Different Separators

```rust
use gpui_component::breadcrumb::BreadcrumbSeparator;

// Slash separator
Breadcrumb::new()
    .separator(BreadcrumbSeparator::Slash)
    .child(BreadcrumbItem::new("Home"))
    .child(BreadcrumbItem::new("Documents"))

// Chevron separator (default)
Breadcrumb::new()
    .separator(BreadcrumbSeparator::ChevronRight)
    .child(BreadcrumbItem::new("Home"))
    .child(BreadcrumbItem::new("Documents"))

// Dot separator
Breadcrumb::new()
    .separator(BreadcrumbSeparator::Dot)
    .child(BreadcrumbItem::new("Home"))
    .child(BreadcrumbItem::new("Documents"))

// Custom icon separator
Breadcrumb::new()
    .separator(BreadcrumbSeparator::Icon(IconName::ArrowRight))
    .child(BreadcrumbItem::new("Home"))
    .child(BreadcrumbItem::new("Documents"))
```

### Different Sizes

```rust
use gpui_component::Size;

// Extra small
Breadcrumb::new()
    .with_size(Size::XSmall)
    .child(BreadcrumbItem::new("Home"))

// Small
Breadcrumb::new()
    .with_size(Size::Small)
    .child(BreadcrumbItem::new("Home"))

// Medium (default)
Breadcrumb::new()
    .with_size(Size::Medium)
    .child(BreadcrumbItem::new("Home"))

// Large
Breadcrumb::new()
    .with_size(Size::Large)
    .child(BreadcrumbItem::new("Home"))
```

### Maximum Items with Ellipsis

```rust
// Show maximum 3 items, truncate middle items
Breadcrumb::new()
    .max_items(3)
    .child(BreadcrumbItem::new("Root"))
    .child(BreadcrumbItem::new("Very"))
    .child(BreadcrumbItem::new("Long"))
    .child(BreadcrumbItem::new("Path"))
    .child(BreadcrumbItem::new("Structure"))
    .child(BreadcrumbItem::new("Current"))
// Renders as: Root > ... > Current

// Show maximum 4 items
Breadcrumb::new()
    .max_items(4)
    .child(BreadcrumbItem::new("Home"))
    .child(BreadcrumbItem::new("Users"))
    .child(BreadcrumbItem::new("Documents"))
    .child(BreadcrumbItem::new("Projects"))
    .child(BreadcrumbItem::new("Current"))
// Renders as: Home > ... > Projects > Current
```

### Disabled State

```rust
Breadcrumb::new()
    .disabled(true)
    .child(BreadcrumbItem::new("Home").icon(IconName::Building2))
    .child(BreadcrumbItem::new("Documents").icon(IconName::Folder))
```

### Dynamic Breadcrumbs

```rust
// Build breadcrumbs from a path vector
let path = vec!["Home", "Documents", "Projects", "Current"];

let breadcrumb = Breadcrumb::new()
    .children(
        path.iter()
            .enumerate()
            .map(|(i, segment)| {
                BreadcrumbItem::new(segment.to_string())
                    .icon(match i {
                        0 => IconName::Building2,
                        _ if i == path.len() - 1 => IconName::File,
                        _ => IconName::Folder,
                    })
                    .on_click(move |_, _, cx| {
                        println!("Navigate to index: {}", i);
                    })
            })
            .collect::<Vec<_>>()
    );
```

## Props

### Breadcrumb Props

| Prop        | Type                  | Default        | Description                             |
| ----------- | --------------------- | -------------- | --------------------------------------- |
| `separator` | `BreadcrumbSeparator` | `ChevronRight` | Separator style between items           |
| `max_items` | `Option<usize>`       | `None`         | Maximum items to show before truncating |
| `size`      | `Size`                | `Medium`       | Size variant                            |
| `disabled`  | `bool`                | `false`        | Whether the breadcrumb is disabled      |

### BreadcrumbItem Props

| Prop       | Type               | Default | Description                  |
| ---------- | ------------------ | ------- | ---------------------------- |
| `label`    | `String`           | -       | Text label for the item      |
| `icon`     | `Option<IconName>` | `None`  | Optional icon for the item   |
| `disabled` | `bool`             | `false` | Whether the item is disabled |

## Events

### BreadcrumbItem Events

| Event      | Type                                         | Description                |
| ---------- | -------------------------------------------- | -------------------------- |
| `on_click` | `fn(&MouseEvent, &mut Window, &mut Context)` | Fired when item is clicked |

## Separator Types

```rust
pub enum BreadcrumbSeparator {
    Slash,           // "/"
    ChevronRight,    // ">"
    Dot,             // "•"
    Icon(IconName),  // Custom icon
}
```

## Examples

### File System Navigation

```rust
Breadcrumb::new()
    .separator(BreadcrumbSeparator::Slash)
    .child(
        BreadcrumbItem::new("C:")
            .icon(IconName::HardDrive)
            .on_click(|_, _, cx| { /* navigate to C: */ })
    )
    .child(
        BreadcrumbItem::new("Users")
            .icon(IconName::Folder)
            .on_click(|_, _, cx| { /* navigate to Users */ })
    )
    .child(
        BreadcrumbItem::new("Documents")
            .icon(IconName::FolderOpen)
    )
```

### Website Navigation

```rust
Breadcrumb::new()
    .child(
        BreadcrumbItem::new("Home")
            .icon(IconName::Building2)
            .on_click(|_, _, cx| { /* navigate to home */ })
    )
    .child(
        BreadcrumbItem::new("Products")
            .on_click(|_, _, cx| { /* navigate to products */ })
    )
    .child(
        BreadcrumbItem::new("Laptops")
            .on_click(|_, _, cx| { /* navigate to laptops */ })
    )
    .child(BreadcrumbItem::new("MacBook Pro"))
```

### Settings Navigation

```rust
Breadcrumb::new()
    .separator(BreadcrumbSeparator::Icon(IconName::ArrowRight))
    .max_items(4)
    .child(
        BreadcrumbItem::new("Settings")
            .icon(IconName::Settings)
            .on_click(|_, _, cx| { /* navigate to settings */ })
    )
    .child(
        BreadcrumbItem::new("System")
            .on_click(|_, _, cx| { /* navigate to system */ })
    )
    .child(
        BreadcrumbItem::new("Display")
            .on_click(|_, _, cx| { /* navigate to display */ })
    )
    .child(BreadcrumbItem::new("Resolution"))
```

## Accessibility

- Supports keyboard navigation (tab, enter, space)
- Proper ARIA labels and navigation roles
- Focus management and visual indicators
- Screen reader compatible with semantic navigation
- Ellipsis items are properly announced

## Styling

The Breadcrumb component uses theme colors:

- `theme.foreground` - Text color
- `theme.muted_foreground` - Separator and disabled state
- `theme.accent` - Hover and focus states
- `theme.primary` - Active/current item

## Best Practices

1. **Keep it concise**: Use `max_items` for long paths
2. **Meaningful labels**: Use clear, descriptive names
3. **Consistent icons**: Use consistent icon styles throughout
4. **Click handling**: Make parent levels clickable for navigation
5. **Current item**: The last item typically shouldn't be clickable

## Notes

- The last breadcrumb item is typically the current page and should not be clickable
- When using `max_items`, the component automatically handles ellipsis placement
- Icons are optional but help with visual hierarchy
- The component is fully responsive and adapts to different screen sizes
- Use semantic HTML structure for better accessibility
