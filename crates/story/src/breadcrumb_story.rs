use gpui::{
    App, AppContext as _, Context, Entity, Focusable, IntoElement, ParentElement as _, Render,
    Styled as _, Window,
};

use gpui_component::{
    breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbSeparator},
    button::{Button, ButtonVariants as _},
    h_flex, v_flex,
    label::Label,
    IconName, Size, Sizable as _, Disableable as _,
};

use crate::section;

pub struct BreadcrumbStory {
    focus_handle: gpui::FocusHandle,
    current_path: Vec<String>,
}

impl BreadcrumbStory {
    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            current_path: vec![
                "Home".to_string(),
                "Documents".to_string(),
                "Projects".to_string(),
                "GPUI Component".to_string(),
            ],
        })
    }

    fn navigate_to(&mut self, index: usize, cx: &mut Context<Self>) {
        self.current_path.truncate(index + 1);
        cx.notify();
    }
}

impl super::Story for BreadcrumbStory {
    fn title() -> &'static str {
        "Breadcrumb"
    }

    fn description() -> &'static str {
        "Navigation component showing the current location within a hierarchy."
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for BreadcrumbStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for BreadcrumbStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .p_4()
            .child(
                section("Basic Breadcrumb")
                    .child(
                        Breadcrumb::new()
                            .child(BreadcrumbItem::new("Home").on_click({
                                let entity = cx.entity();
                                move |_event, _window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.navigate_to(0, cx);
                                    })
                                }
                            }))
                            .child(BreadcrumbItem::new("Library").on_click({
                                let entity = cx.entity();
                                move |_event, _window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.navigate_to(1, cx);
                                    })
                                }
                            }))
                            .child(BreadcrumbItem::new("Data"))
                    )
                    .child(Label::new("Click on breadcrumb items to navigate"))
            )
            .child(
                section("With Icons")
                    .child(
                        Breadcrumb::new()
                            .child(
                                BreadcrumbItem::new("Home")
                                    .icon(IconName::Building2)
                                    .on_click({
                                        let entity = cx.entity();
                                        move |_event, _window, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.navigate_to(0, cx);
                                            })
                                        }
                                    })
                            )
                            .child(
                                BreadcrumbItem::new("Documents")
                                    .icon(IconName::Folder)
                                    .on_click({
                                        let entity = cx.entity();
                                        move |_event, _window, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.navigate_to(1, cx);
                                            })
                                        }
                                    })
                            )
                            .child(
                                BreadcrumbItem::new("Projects")
                                    .icon(IconName::FolderOpen)
                                    .on_click({
                                        let entity = cx.entity();
                                        move |_event, _window, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.navigate_to(2, cx);
                                            })
                                        }
                                    })
                            )
                            .child(
                                BreadcrumbItem::new("Current File")
                                    .icon(IconName::File)
                            )
                    )
            )
            .child(
                section("Different Separators")
                    .child(v_flex().gap_3()
                        .child(v_flex().gap_2()
                            .child(Label::new("Chevron (Default)"))
                            .child(
                                Breadcrumb::new()
                                    .separator(BreadcrumbSeparator::ChevronRight)
                                    .child(BreadcrumbItem::new("Home"))
                                    .child(BreadcrumbItem::new("Documents"))
                                    .child(BreadcrumbItem::new("File"))
                            )
                        )
                        .child(v_flex().gap_2()
                            .child(Label::new("Slash"))
                            .child(
                                Breadcrumb::new()
                                    .separator(BreadcrumbSeparator::Slash)
                                    .child(BreadcrumbItem::new("Home"))
                                    .child(BreadcrumbItem::new("Documents"))
                                    .child(BreadcrumbItem::new("File"))
                            )
                        )
                        .child(v_flex().gap_2()
                            .child(Label::new("Dot"))
                            .child(
                                Breadcrumb::new()
                                    .separator(BreadcrumbSeparator::Dot)
                                    .child(BreadcrumbItem::new("Home"))
                                    .child(BreadcrumbItem::new("Documents"))
                                    .child(BreadcrumbItem::new("File"))
                            )
                        )
                        .child(v_flex().gap_2()
                            .child(Label::new("Custom Icon (Arrow)"))
                            .child(
                                Breadcrumb::new()
                                    .separator(BreadcrumbSeparator::Icon(IconName::ArrowRight))
                                    .child(BreadcrumbItem::new("Home"))
                                    .child(BreadcrumbItem::new("Documents"))
                                    .child(BreadcrumbItem::new("File"))
                            )
                        )
                    )
            )
            .child(
                section("Different Sizes")
                    .child(v_flex().gap_4()
                        .child(v_flex().gap_2()
                            .child(Label::new("Extra Small"))
                            .child(
                                Breadcrumb::new()
                                    .with_size(Size::XSmall)
                                    .child(BreadcrumbItem::new("Home").icon(IconName::Building2))
                                    .child(BreadcrumbItem::new("Documents").icon(IconName::Folder))
                                    .child(BreadcrumbItem::new("File").icon(IconName::File))
                            )
                        )
                        .child(v_flex().gap_2()
                            .child(Label::new("Small"))
                            .child(
                                Breadcrumb::new()
                                    .with_size(Size::Small)
                                    .child(BreadcrumbItem::new("Home").icon(IconName::Building2))
                                    .child(BreadcrumbItem::new("Documents").icon(IconName::Folder))
                                    .child(BreadcrumbItem::new("File").icon(IconName::File))
                            )
                        )
                        .child(v_flex().gap_2()
                            .child(Label::new("Medium (Default)"))
                            .child(
                                Breadcrumb::new()
                                    .with_size(Size::Medium)
                                    .child(BreadcrumbItem::new("Home").icon(IconName::Building2))
                                    .child(BreadcrumbItem::new("Documents").icon(IconName::Folder))
                                    .child(BreadcrumbItem::new("File").icon(IconName::File))
                            )
                        )
                        .child(v_flex().gap_2()
                            .child(Label::new("Large"))
                            .child(
                                Breadcrumb::new()
                                    .with_size(Size::Large)
                                    .child(BreadcrumbItem::new("Home").icon(IconName::Building2))
                                    .child(BreadcrumbItem::new("Documents").icon(IconName::Folder))
                                    .child(BreadcrumbItem::new("File").icon(IconName::File))
                            )
                        )
                    )
            )
            .child(
                section("With Maximum Items (Ellipsis)")
                    .child(v_flex().gap_3()
                        .child(v_flex().gap_2()
                            .child(Label::new("Max 3 items"))
                            .child(
                                Breadcrumb::new()
                                    .max_items(3)
                                    .child(BreadcrumbItem::new("Root"))
                                    .child(BreadcrumbItem::new("Very"))
                                    .child(BreadcrumbItem::new("Long"))
                                    .child(BreadcrumbItem::new("Path"))
                                    .child(BreadcrumbItem::new("Structure"))
                                    .child(BreadcrumbItem::new("Current"))
                            )
                        )
                        .child(v_flex().gap_2()
                            .child(Label::new("Max 4 items"))
                            .child(
                                Breadcrumb::new()
                                    .max_items(4)
                                    .child(BreadcrumbItem::new("Home").icon(IconName::Building2))
                                    .child(BreadcrumbItem::new("Users").icon(IconName::User))
                                    .child(BreadcrumbItem::new("John").icon(IconName::User))
                                    .child(BreadcrumbItem::new("Documents").icon(IconName::Folder))
                                    .child(BreadcrumbItem::new("Projects").icon(IconName::FolderOpen))
                                    .child(BreadcrumbItem::new("GPUI").icon(IconName::File))
                                    .child(BreadcrumbItem::new("Component").icon(IconName::File))
                            )
                        )
                    )
            )
            .child(
                section("Interactive Navigation")
                    .child(v_flex().gap_3()
                        .child(
                            Breadcrumb::new()
                                .children(
                                    self.current_path
                                        .iter()
                                        .enumerate()
                                        .map(|(i, path)| {
                                            BreadcrumbItem::new(path.clone())
                                                .icon(match i {
                                                    0 => IconName::Building2,
                                                    _ if i == self.current_path.len() - 1 => IconName::File,
                                                    _ => IconName::Folder,
                                                })
                                                .on_click({
                                                    let entity = cx.entity();
                                                    move |_event, _window, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.navigate_to(i, cx);
                                                        })
                                                    }
                                                })
                                        })
                                        .collect::<Vec<_>>()
                                )
                        )
                        .child(Label::new("Click on any breadcrumb item to navigate to that level"))
                        .child(
                            h_flex().gap_2()
                                .child(
                                    Button::new("add_level")
                                        .label("Add Level")
                                        .small()
                                        .on_click({
                                            let entity = cx.entity();
                                            move |_event, _window, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.current_path.push(format!("Level {}", this.current_path.len()));
                                                    cx.notify();
                                                })
                                            }
                                        })
                                )
                                .child(
                                    Button::new("remove_level")
                                        .label("Remove Level")
                                        .small()
                                        .on_click({
                                            let entity = cx.entity();
                                            move |_event, _window, cx| {
                                                entity.update(cx, |this, cx| {
                                                    if this.current_path.len() > 1 {
                                                        this.current_path.pop();
                                                        cx.notify();
                                                    }
                                                })
                                            }
                                        })
                                )
                                .child(
                                    Button::new("reset_path")
                                        .label("Reset")
                                        .small()
                                        .danger()
                                        .on_click({
                                            let entity = cx.entity();
                                            move |_event, _window, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.current_path = vec!["Home".to_string()];
                                                    cx.notify();
                                                })
                                            }
                                        })
                                )
                        )
                    )
            )
            .child(
                section("Disabled State")
                    .child(
                        Breadcrumb::new()
                            .disabled(true)
                            .child(BreadcrumbItem::new("Home").icon(IconName::Building2))
                            .child(BreadcrumbItem::new("Documents").icon(IconName::Folder))
                            .child(BreadcrumbItem::new("Current File").icon(IconName::File))
                    )
            )
    }
}