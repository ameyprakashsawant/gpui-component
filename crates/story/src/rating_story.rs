use gpui::{
    App, AppContext as _, Context, Entity, Focusable, IntoElement, ParentElement as _, Render,
    Styled as _, Window,
};

use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex, v_flex,
    rating::{Rating, RatingVariant},
    label::Label,
    Sizable as _,
};

use crate::section;

pub struct RatingStory {
    focus_handle: gpui::FocusHandle,
    basic_rating: f32,
    heart_rating: f32,
    thumb_rating: f32,
    precision_rating: f32,
    custom_max_rating: f32,
}

impl RatingStory {
    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            basic_rating: 3.5,
            heart_rating: 2.0,
            thumb_rating: 1.0,
            precision_rating: 3.5,
            custom_max_rating: 7.5,
        })
    }
}

impl super::Story for RatingStory {
    fn title() -> &'static str {
        "Rating"
    }

    fn description() -> &'static str {
        "Displays a rating component with various styles and configurations."
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for RatingStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RatingStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .p_4()
            .child(
                section("Basic Star Rating")
                    .child(
                        Rating::new("basic")
                            .value(self.basic_rating)
                            .on_rating({
                                let entity = cx.entity();
                                move |rating: f32, _window: &mut Window, cx: &mut App| {
                                    entity.update(cx, |this, cx| {
                                        this.basic_rating = rating;
                                        cx.notify();
                                    })
                                }
                            })
                    )
                    .child(Label::new(format!("Current rating: {:.1}", self.basic_rating)))
            )
            .child(
                section("Read-only Ratings")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(v_flex().gap_2()
                                .child(Label::new("5 stars"))
                                .child(Rating::new("readonly5").value(5.0).readonly(true))
                            )
                            .child(v_flex().gap_2()
                                .child(Label::new("3.5 stars"))
                                .child(Rating::new("readonly3").value(3.5).readonly(true))
                            )
                            .child(v_flex().gap_2()
                                .child(Label::new("1 star"))
                                .child(Rating::new("readonly1").value(1.0).readonly(true))
                            )
                            .child(v_flex().gap_2()
                                .child(Label::new("0 stars"))
                                .child(Rating::new("readonly0").value(0.0).readonly(true))
                            )
                    )
            )
            .child(
                section("Heart Rating")
                    .child(
                        Rating::new("heart")
                            .variant(RatingVariant::Heart)
                            .value(self.heart_rating)
                            .on_rating({
                                let entity = cx.entity();
                                move |rating: f32, _window: &mut Window, cx: &mut App| {
                                    entity.update(cx, |this, cx| {
                                        this.heart_rating = rating;
                                        cx.notify();
                                    })
                                }
                            })
                    )
                    .child(Label::new(format!("Hearts: {:.1}", self.heart_rating)))
            )
            .child(
                section("Thumb Rating")
                    .child(
                        Rating::new("thumb")
                            .variant(RatingVariant::Thumb)
                            .max_rating(1)
                            .value(self.thumb_rating)
                            .on_rating({
                                let entity = cx.entity();
                                move |rating: f32, _window: &mut Window, cx: &mut App| {
                                    entity.update(cx, |this, cx| {
                                        this.thumb_rating = rating;
                                        cx.notify();
                                    })
                                }
                            })
                    )
                    .child(Label::new(format!("Thumbs up: {}", if self.thumb_rating > 0.0 { "Yes" } else { "No" })))
            )
            .child(
                section("Precision Rating")
                    .child(
                        Rating::new("precision")
                            .value(self.precision_rating)
                            .precision(true)
                            .show_text(true)
                            .on_rating({
                                let entity = cx.entity();
                                move |rating: f32, _window: &mut Window, cx: &mut App| {
                                    entity.update(cx, |this, cx| {
                                        this.precision_rating = rating;
                                        cx.notify();
                                    })
                                }
                            })
                    )
            )
            .child(
                section("Different Sizes")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(
                                h_flex()
                                    .gap_4()
                                    .items_center()
                                    .child(Label::new("Extra Small"))
                                    .child(Rating::new("size_xs").value(3.0).size(gpui_component::Size::XSmall))
                            )
                            .child(
                                h_flex()
                                    .gap_4()
                                    .items_center()
                                    .child(Label::new("Small"))
                                    .child(Rating::new("size_sm").value(3.0).size(gpui_component::Size::Small))
                            )
                            .child(
                                h_flex()
                                    .gap_4()
                                    .items_center()
                                    .child(Label::new("Medium"))
                                    .child(Rating::new("size_md").value(3.0).size(gpui_component::Size::Medium))
                            )
                            .child(
                                h_flex()
                                    .gap_4()
                                    .items_center()
                                    .child(Label::new("Large"))
                                    .child(Rating::new("size_lg").value(3.0).size(gpui_component::Size::Large))
                            )
                    )
            )
            .child(
                section("Custom Max Rating")
                    .child(
                        Rating::new("custom_max")
                            .max_rating(10)
                            .value(self.custom_max_rating)
                            .show_text(true)
                            .on_rating({
                                let entity = cx.entity();
                                move |rating: f32, _window: &mut Window, cx: &mut App| {
                                    entity.update(cx, |this, cx| {
                                        this.custom_max_rating = rating;
                                        cx.notify();
                                    })
                                }
                            })
                    )
            )
            .child(
                section("Disabled Rating")
                    .child(
                        Rating::new("disabled")
                            .value(3.0)
                            .disabled(true)
                    )
            )
            .child(
                section("Interactive Controls")
                    .child(v_flex().gap_3()
                        .child(Rating::new("controlled").value(self.basic_rating))
                        .child(
                            h_flex().gap_2()
                                .child(
                                    Button::new("set_low")
                                        .label("Set Low (1.5)")
                                        .small()
                                        .on_click({
                                            let entity = cx.entity();
                                            move |_event, _window: &mut Window, cx: &mut App| {
                                                entity.update(cx, |this, cx| {
                                                    this.basic_rating = 1.5;
                                                    cx.notify();
                                                })
                                            }
                                        })
                                )
                                .child(
                                    Button::new("set_mid")
                                        .label("Set Mid (3.0)")
                                        .small()
                                        .on_click({
                                            let entity = cx.entity();
                                            move |_event, _window: &mut Window, cx: &mut App| {
                                                entity.update(cx, |this, cx| {
                                                    this.basic_rating = 3.0;
                                                    cx.notify();
                                                })
                                            }
                                        })
                                )
                                .child(
                                    Button::new("set_high")
                                        .label("Set High (4.5)")
                                        .small()
                                        .on_click({
                                            let entity = cx.entity();
                                            move |_event, _window: &mut Window, cx: &mut App| {
                                                entity.update(cx, |this, cx| {
                                                    this.basic_rating = 4.5;
                                                    cx.notify();
                                                })
                                            }
                                        })
                                )
                                .child(
                                    Button::new("reset_all")
                                        .label("Reset All")
                                        .small()
                                        .danger()
                                        .on_click({
                                            let entity = cx.entity();
                                            move |_event, _window: &mut Window, cx: &mut App| {
                                                entity.update(cx, |this, cx| {
                                                    this.basic_rating = 0.0;
                                                    this.heart_rating = 0.0;
                                                    this.thumb_rating = 0.0;
                                                    this.precision_rating = 0.0;
                                                    this.custom_max_rating = 0.0;
                                                    cx.notify();
                                                })
                                            }
                                        })
                                )
                        )
                    )
            )
    }
}