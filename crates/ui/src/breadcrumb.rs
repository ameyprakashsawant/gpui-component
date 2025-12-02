use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, ClickEvent, ElementId, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window,
};

use crate::{h_flex, ActiveTheme, Disableable, Icon, IconName, Size, Sizable, StyledExt};

/// Separator styles for breadcrumbs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    /// Forward slash separator (/)
    Slash,
    /// Chevron right separator (>)  
    ChevronRight,
    /// Dot separator (•)
    Dot,
    /// Custom icon separator
    Icon(IconName),
}

impl Default for BreadcrumbSeparator {
    fn default() -> Self {
        Self::ChevronRight
    }
}

/// A breadcrumb navigation element.
#[derive(IntoElement)]
pub struct Breadcrumb {
    style: StyleRefinement,
    items: Vec<BreadcrumbItem>,
    separator: BreadcrumbSeparator,
    size: Size,
    disabled: bool,
    max_items: Option<usize>,
}

/// Item for the [`Breadcrumb`].
#[derive(IntoElement, Clone)]
pub struct BreadcrumbItem {
    id: ElementId,
    style: StyleRefinement,
    label: SharedString,
    icon: Option<IconName>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    disabled: bool,
    is_last: bool,
}

impl BreadcrumbItem {
    /// Create a new BreadcrumbItem with the given id and label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: ElementId::Integer(0),
            style: StyleRefinement::default(),
            label: label.into(),
            icon: None,
            on_click: None,
            disabled: false,
            is_last: false,
        }
    }

    /// Set an icon for this breadcrumb item
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// For internal use only.
    fn is_last(mut self, is_last: bool) -> Self {
        self.is_last = is_last;
        self
    }
}

impl Styled for BreadcrumbItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl From<&'static str> for BreadcrumbItem {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for BreadcrumbItem {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SharedString> for BreadcrumbItem {
    fn from(value: SharedString) -> Self {
        Self::new(value)
    }
}

impl RenderOnce for BreadcrumbItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut item_container = h_flex().items_center().gap_1();
        
        // Add icon if present
        if let Some(icon) = self.icon {
            item_container = item_container.child(Icon::new(icon).size_3p5());
        }
        
        // Add label
        item_container = item_container.child(self.label);
        
        div()
            .id(self.id)
            .child(item_container)
            .text_color(cx.theme().muted_foreground)
            .when(self.is_last, |this| {
                this.text_color(cx.theme().foreground).font_medium()
            })
            .when(self.disabled, |this| {
                this.text_color(cx.theme().muted_foreground).opacity(0.5)
            })
            .refine_style(&self.style)
            .when(!self.disabled && !self.is_last, |this| {
                this.hover(|this| this.text_color(cx.theme().foreground))
                    .when_some(self.on_click, |this, on_click| {
                        this.cursor_pointer().on_click(move |event, window, cx| {
                            on_click(event, window, cx);
                        })
                    })
            })
    }
}

impl Breadcrumb {
    /// Create a new breadcrumb.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            style: StyleRefinement::default(),
            separator: BreadcrumbSeparator::default(),
            size: Size::Medium,
            disabled: false,
            max_items: None,
        }
    }

    /// Add an [`BreadcrumbItem`] to the breadcrumb.
    pub fn child(mut self, item: impl Into<BreadcrumbItem>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Add multiple [`BreadcrumbItem`] items to the breadcrumb.
    pub fn children(mut self, items: impl IntoIterator<Item = impl Into<BreadcrumbItem>>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }

    /// Set the separator style
    pub fn separator(mut self, separator: BreadcrumbSeparator) -> Self {
        self.separator = separator;
        self
    }

    /// Set the maximum number of items to display (will show ellipsis if exceeded)
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
        self
    }

    /// Get the items to display (with ellipsis handling)
    fn get_display_items(&self) -> (Vec<&BreadcrumbItem>, bool) {
        if let Some(max) = self.max_items {
            if self.items.len() > max && max >= 3 {
                let mut display_items = Vec::new();
                
                // Show first item
                display_items.push(&self.items[0]);
                
                // Add the last (max - 2) items
                let start_idx = self.items.len() - (max - 2);
                for item in &self.items[start_idx..] {
                    display_items.push(item);
                }
                
                return (display_items, true); // true = show ellipsis
            } else if let Some(max) = self.max_items {
                // Just show the last max items
                let start_idx = self.items.len().saturating_sub(max);
                return (self.items[start_idx..].iter().collect(), false);
            }
        }
        
        (self.items.iter().collect(), false)
    }
}

#[derive(IntoElement)]
struct BreadcrumbSeparatorElement {
    separator: BreadcrumbSeparator,
}

impl BreadcrumbSeparatorElement {
    fn new(separator: BreadcrumbSeparator) -> Self {
        Self { separator }
    }
}

impl RenderOnce for BreadcrumbSeparatorElement {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.separator {
            BreadcrumbSeparator::Slash => div()
                .child("/")
                .text_color(cx.theme().muted_foreground)
                .into_any_element(),
            BreadcrumbSeparator::ChevronRight => Icon::new(IconName::ChevronRight)
                .text_color(cx.theme().muted_foreground)
                .size_3p5()
                .into_any_element(),
            BreadcrumbSeparator::Dot => div()
                .child("•")
                .text_color(cx.theme().muted_foreground)
                .into_any_element(),
            BreadcrumbSeparator::Icon(icon) => Icon::new(icon)
                .text_color(cx.theme().muted_foreground)
                .size_3p5()
                .into_any_element(),
        }
    }
}

impl Disableable for Breadcrumb {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Sizable for Breadcrumb {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Breadcrumb {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let (display_items, show_ellipsis) = self.get_display_items();
        let items_count = display_items.len();

        let mut children = vec![];
        
        for (ix, item) in display_items.into_iter().enumerate() {
            let is_last = ix == items_count - 1;

            // Add ellipsis after first item if needed
            if show_ellipsis && ix == 1 {
                children.push(BreadcrumbSeparatorElement::new(self.separator).into_any_element());
                children.push(
                    div()
                        .child("...")
                        .text_color(cx.theme().muted_foreground)
                        .into_any_element()
                );
            }
            
            // Add separator before item (except first and after ellipsis)
            if ix > 0 && !(show_ellipsis && ix == 1) {
                children.push(BreadcrumbSeparatorElement::new(self.separator).into_any_element());
            }

            let item = item.clone().id(ix);
            children.push(item.is_last(is_last).into_any_element());
        }

        h_flex()
            .gap_1p5()
            .items_center()
            .when(self.size == Size::XSmall, |this| this.text_xs())
            .when(self.size == Size::Small, |this| this.text_sm())
            .when(self.size == Size::Medium, |this| this.text_sm())
            .when(self.size == Size::Large, |this| this.text_base())
            .text_color(cx.theme().muted_foreground)
            .when(self.disabled, |this| {
                this.opacity(0.5).cursor_not_allowed()
            })
            .refine_style(&self.style)
            .children(children)
    }
}
