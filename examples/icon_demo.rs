use gpui::*;
use fluix::{Icon, IconName};

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(900.), px(800.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Icon Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| IconDemo::new(window, cx))
        }).unwrap();
    });
}

struct IconDemo {}

impl IconDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for IconDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .bg(rgb(0xF5F5F5))
                    .p_8()
                    .gap_8()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w_full()
                            .max_w(px(800.))
                            .mx_auto()
                            .gap_8()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x333333))
                                            .child("Fluix Icon Component")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Scalable SVG icons for your UI")
                                    )
                            )
                            .child(self.render_size_section())
                            .child(self.render_color_section())
                            .child(self.render_icon_grid())
                    )
            )
    }
}

impl IconDemo {
    fn render_size_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p_6()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(8.))
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x333333))
                            .child("Icon Sizes")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("Different sizes from XSmall to XLarge")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_6()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Star).xsmall())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("XSmall")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Star).small())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Small")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Star).medium())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Medium")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Star).large())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Large")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Star).xlarge())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("XLarge")
                            )
                    )
            )
    }

    fn render_color_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p_6()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(8.))
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x333333))
                            .child("Icon Colors")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("Custom colors for different contexts")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_6()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Heart).large().color(rgb(0xEF4444)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Red")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Success).large().color(rgb(0x22C55E)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Green")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Info).large().color(rgb(0x3B82F6)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Blue")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Warning).large().color(rgb(0xF59E0B)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Orange")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Star).large().color(rgb(0x8B5CF6)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Purple")
                            )
                    )
            )
    }

    fn render_icon_grid(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p_6()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(8.))
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x333333))
                            .child("Available Icons")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("20 built-in icons ready to use")
                    )
            )
            .child(
                div()
                    .grid()
                    .grid_cols(5)
                    .gap_4()
                    .child(self.render_icon_item(IconName::ArrowLeft, "ArrowLeft"))
                    .child(self.render_icon_item(IconName::ArrowRight, "ArrowRight"))
                    .child(self.render_icon_item(IconName::ArrowUp, "ArrowUp"))
                    .child(self.render_icon_item(IconName::ArrowDown, "ArrowDown"))
                    .child(self.render_icon_item(IconName::Check, "Check"))
                    .child(self.render_icon_item(IconName::Close, "Close"))
                    .child(self.render_icon_item(IconName::Plus, "Plus"))
                    .child(self.render_icon_item(IconName::Minus, "Minus"))
                    .child(self.render_icon_item(IconName::Search, "Search"))
                    .child(self.render_icon_item(IconName::Settings, "Settings"))
                    .child(self.render_icon_item(IconName::Home, "Home"))
                    .child(self.render_icon_item(IconName::User, "User"))
                    .child(self.render_icon_item(IconName::Bell, "Bell"))
                    .child(self.render_icon_item(IconName::Star, "Star"))
                    .child(self.render_icon_item(IconName::Heart, "Heart"))
                    .child(self.render_icon_item(IconName::Menu, "Menu"))
                    .child(self.render_icon_item(IconName::Info, "Info"))
                    .child(self.render_icon_item(IconName::Warning, "Warning"))
                    .child(self.render_icon_item(IconName::Error, "Error"))
                    .child(self.render_icon_item(IconName::Success, "Success"))
            )
    }

    fn render_icon_item(&self, icon: IconName, name: &str) -> impl IntoElement {
        let name = name.to_string();
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p_4()
            .bg(rgb(0xF9FAFB))
            .border_1()
            .border_color(rgb(0xE5E7EB))
            .rounded(px(6.))
            .gap_2()
            .child(Icon::new(icon).large())
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x6B7280))
                    .child(name)
            )
    }
}
