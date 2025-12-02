use crate::{
    ActiveTheme, Icon, IconName, Sizable, Size,
    h_flex,
};
use gpui::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use std::rc::Rc;

/// The visual style of the rating component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingVariant {
    /// Star-based rating (⭐)
    Star,
    /// Heart-based rating (❤️)
    Heart,
    /// Thumb-based rating (👍/👎)
    Thumb,
    /// Custom icon
    Custom(IconName),
}

impl Default for RatingVariant {
    fn default() -> Self {
        Self::Star
    }
}

/// Callback function for rating changes
pub type RatingCallback = Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>;

/// A rating component that allows users to select a rating value
///
/// # Examples
///
/// ```rust
/// use gpui_component::rating::*;
///
/// // Basic star rating
/// Rating::new("rating")
///     .value(3.5)
///     .on_change(|rating, _, _| println!("Rating: {}", rating))
///
/// // Read-only heart rating
/// Rating::new("readonly")
///     .value(4.0)
///     .variant(RatingVariant::Heart)
///     .readonly(true)
///
/// // Custom size and max rating
/// Rating::new("custom")
///     .max_rating(10)
///     .size(Size::Large)
///     .precision(true)
/// ```
#[derive(IntoElement)]
pub struct Rating {
    id: ElementId,
    value: f32,
    max_rating: u8,
    variant: RatingVariant,
    size: Size,
    readonly: bool,
    precision: bool, // Allow half ratings
    show_text: bool, // Show numeric value
    disabled: bool,
    on_change: Option<RatingCallback>,
}

impl Rating {
    /// Create a new Rating component with the given ID
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.0,
            max_rating: 5,
            variant: RatingVariant::default(),
            size: Size::Medium,
            readonly: false,
            precision: false,
            show_text: false,
            disabled: false,
            on_change: None,
        }
    }

    /// Set the current rating value (0.0 to max_rating)
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, self.max_rating as f32);
        self
    }

    /// Set the maximum rating value (default: 5)
    pub fn max_rating(mut self, max: u8) -> Self {
        self.max_rating = max.max(1);
        self.value = self.value.clamp(0.0, self.max_rating as f32);
        self
    }

    /// Set the rating variant (Star, Heart, Thumb, or Custom)
    pub fn variant(mut self, variant: RatingVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Enable/disable read-only mode
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Enable/disable half-star precision
    pub fn precision(mut self, precision: bool) -> Self {
        self.precision = precision;
        self
    }

    /// Show numeric text alongside rating
    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    /// Enable/disable the component
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the size of the rating component
    pub fn size(self, size: Size) -> Self {
        self.with_size(size)
    }

    /// Set the callback to be called when the rating value changes
    pub fn on_rating<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32, &mut Window, &mut App) + 'static,
    {
        self.on_change = Some(Rc::new(callback));
        self
    }

    /// Get the icon for the given variant
    fn get_icon(&self, filled: bool) -> IconName {
        match self.variant {
            RatingVariant::Star => {
                if filled {
                    IconName::Star
                } else {
                    IconName::StarOff
                }
            }
            RatingVariant::Heart => {
                if filled {
                    IconName::Heart
                } else {
                    IconName::HeartOff
                }
            }
            RatingVariant::Thumb => {
                if filled {
                    IconName::ThumbsUp
                } else {
                    IconName::ThumbsDown
                }
            }
            RatingVariant::Custom(icon) => icon,
        }
    }

    /// Get the color for a rating item based on its state
    fn get_color(&self, cx: &App, filled: bool, hovered: bool) -> Hsla {
        if self.disabled {
            return cx.theme().muted_foreground;
        }

        match self.variant {
            RatingVariant::Star => {
                if filled || hovered {
                    cx.theme().warning
                } else {
                    cx.theme().muted_foreground
                }
            }
            RatingVariant::Heart => {
                if filled || hovered {
                    cx.theme().danger
                } else {
                    cx.theme().muted_foreground
                }
            }
            RatingVariant::Thumb => {
                if filled || hovered {
                    cx.theme().success
                } else {
                    cx.theme().muted_foreground
                }
            }
            RatingVariant::Custom(_) => {
                if filled || hovered {
                    cx.theme().primary
                } else {
                    cx.theme().muted_foreground
                }
            }
        }
    }

    /// Handle click on a rating item
    fn handle_click(&self, rating_value: f32, window: &mut Window, cx: &mut App) {
        if self.readonly || self.disabled {
            return;
        }

        let new_value = if self.precision {
            rating_value
        } else {
            rating_value.ceil()
        };

        if let Some(callback) = &self.on_change {
            callback(new_value, window, cx);
        }
    }
}

impl Sizable for Rating {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for Rating {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon_size = match self.size {
            Size::Size(px) => px,
            Size::XSmall => px(12.),
            Size::Small => px(16.),
            Size::Medium => px(20.),
            Size::Large => px(24.),
        };

        let gap = match self.size {
            Size::Size(px) => px / 4.0,
            Size::XSmall => px(2.),
            Size::Small => px(3.),
            Size::Medium => px(4.),
            Size::Large => px(5.),
        };

        h_flex()
            .id(self.id.clone())
            .gap(gap)
            .items_center()
            .children((1..=self.max_rating).map(|i| {
                let rating_value = i as f32;
                let is_filled = self.value >= rating_value;
                let is_half_filled = self.precision 
                    && self.value >= rating_value - 0.5 
                    && self.value < rating_value;
                
                let icon = self.get_icon(is_filled || is_half_filled);
                
                div()
                    .relative()
                    .child(
                        div()
                            .child(Icon::new(icon).size(icon_size).text_color(self.get_color(cx, is_filled, false)))
                            .when(!self.readonly && !self.disabled, |div| {
                                div.cursor_pointer()
                                    .hover(|div| {
                                        div.bg(cx.theme().accent.opacity(0.1))
                                    })
                                    .on_mouse_down(gpui::MouseButton::Left, {
                                        let callback_self = self.clone();
                                        move |_, window, cx| {
                                            window.prevent_default();
                                            callback_self.handle_click(rating_value, window, cx);
                                        }
                                    })
                            })
                    )
                    .when(is_half_filled, |container| {
                        container.child(
                            div()
                                .absolute()
                                .top_0()
                                .left_0()
                                .w_1_2()
                                .h_full()
                                .overflow_hidden()
                                .child(
                                    Icon::new(self.get_icon(true))
                                        .size(icon_size)
                                        .text_color(self.get_color(cx, true, false))
                                )
                        )
                    })
            }))
            .when(self.show_text, |this| {
                this.child(
                    div()
                        .ml(px(8.))
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(format!("{:.1}", self.value))
                )
            })
    }
}

// Implement Clone for the callback handling
impl Clone for Rating {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            value: self.value,
            max_rating: self.max_rating,
            variant: self.variant,
            size: self.size,
            readonly: self.readonly,
            precision: self.precision,
            show_text: self.show_text,
            disabled: self.disabled,
            on_change: self.on_change.clone(),
        }
    }
}