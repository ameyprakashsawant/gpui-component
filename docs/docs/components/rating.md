# Rating

A customizable rating component that allows users to select and display ratings using stars or other icons.

## Features

- Multiple rating variants (5-star, 10-star, custom max)
- Read-only and interactive modes
- Half-star precision support
- Customizable icons and colors
- Size variants (xs, sm, md, lg)
- Hover effects and animations
- Keyboard navigation support

## Usage

### Basic Rating

```rust
use gpui_component::{Rating, Size};

// Basic 5-star rating
Rating::new("rating")
    .value(3.5)
    .on_change(|value, _, cx| {
        println!("Rating changed to: {}", value);
    })

// Read-only rating
Rating::new("readonly")
    .value(4.0)
    .readonly(true)
```

### Different Sizes

```rust
// Extra small
Rating::new("xs").with_size(Size::XSmall).value(3.0)

// Small
Rating::new("sm").with_size(Size::Small).value(3.0)

// Medium (default)
Rating::new("md").with_size(Size::Medium).value(3.0)

// Large
Rating::new("lg").with_size(Size::Large).value(3.0)
```

### Custom Maximum Value

```rust
// 10-star rating system
Rating::new("ten_star")
    .max_value(10.0)
    .value(7.5)
    .on_change(|value, _, cx| {
        println!("Rating: {}/10", value);
    })

// Custom scale (e.g., 1-7)
Rating::new("custom")
    .max_value(7.0)
    .value(5.0)
```

### Half-Star Precision

```rust
// Enable half-star ratings
Rating::new("half_star")
    .precision(0.5)
    .value(3.5)
    .on_change(|value, _, cx| {
        println!("Rating: {}", value);
    })

// Integer-only ratings
Rating::new("integer")
    .precision(1.0)
    .value(4.0)
```

### Custom Icons

```rust
use gpui_component::IconName;

// Custom filled/empty icons
Rating::new("custom_icons")
    .filled_icon(IconName::Heart)
    .empty_icon(IconName::HeartOutline)
    .value(3.0)
```

### Event Handling

```rust
Rating::new("interactive")
    .value(0.0)
    .on_change(|value, _, cx| {
        // Handle rating change
        println!("New rating: {}", value);
    })
    .on_hover(|value, _, cx| {
        // Handle hover (optional)
        if let Some(hovered_value) = value {
            println!("Hovering over: {}", hovered_value);
        }
    })
```

## Props

| Prop          | Type       | Default       | Description                                                |
| ------------- | ---------- | ------------- | ---------------------------------------------------------- |
| `value`       | `f32`      | `0.0`         | Current rating value                                       |
| `max_value`   | `f32`      | `5.0`         | Maximum rating value                                       |
| `precision`   | `f32`      | `1.0`         | Rating precision (0.5 for half-stars, 1.0 for whole stars) |
| `readonly`    | `bool`     | `false`       | Whether the rating is read-only                            |
| `disabled`    | `bool`     | `false`       | Whether the rating is disabled                             |
| `filled_icon` | `IconName` | `Star`        | Icon for filled stars                                      |
| `empty_icon`  | `IconName` | `StarOutline` | Icon for empty stars                                       |
| `size`        | `Size`     | `Medium`      | Size variant                                               |

## Events

| Event       | Type                                         | Description                     |
| ----------- | -------------------------------------------- | ------------------------------- |
| `on_change` | `fn(f32, &mut Window, &mut Context)`         | Fired when rating value changes |
| `on_hover`  | `fn(Option<f32>, &mut Window, &mut Context)` | Fired when hovering over rating |

## Examples

### Product Rating Display

```rust
Rating::new("product_rating")
    .value(4.2)
    .readonly(true)
    .with_size(Size::Small)
```

### User Review Form

```rust
Rating::new("user_review")
    .max_value(5.0)
    .precision(0.5)
    .value(0.0)
    .on_change(|value, _, cx| {
        // Update form state
        cx.notify();
    })
```

### Custom Heart Rating

```rust
Rating::new("favorite")
    .filled_icon(IconName::Heart)
    .empty_icon(IconName::HeartOutline)
    .max_value(5.0)
    .value(3.0)
    .with_size(Size::Large)
```

## Accessibility

- Supports keyboard navigation (arrow keys, space/enter)
- Proper ARIA labels and roles
- Focus management
- Screen reader compatible

## Styling

The Rating component uses theme colors and can be customized through the theme system:

- `theme.primary` - Filled star color
- `theme.muted_foreground` - Empty star color
- `theme.accent` - Hover color
- `theme.foreground` - Focus outline

## Notes

- The component automatically handles half-star display based on precision
- Hover effects are only active in interactive mode
- The component is fully controlled - you must handle `on_change` to update the value
- All rating calculations respect the precision setting
