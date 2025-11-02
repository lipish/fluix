use gpui::*;
use fluix::{Icon, IconName};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(1200.), px(1000.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix All Icons Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| IconAllDemo::new(window, cx))
        }).unwrap();
    });
}

struct IconAllDemo {
    scroll_handle: ScrollHandle,
}

impl IconAllDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for IconAllDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()
            .child(
                div()
                    .id("scroll-container")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
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
                                    .max_w(px(1100.))
                                    .mx_auto()
                                    .gap_8()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_2xl()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x333333))
                                                    .child("All Icons")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("31 built-in icons available in Fluix")
                                            )
                                    )
                                    .child(self.render_icon_grid())
                            )
                    )
            )
    }
}

impl IconAllDemo {
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
                    .grid()
                    .grid_cols(4)
                    .gap_6()
                    .child(self.render_icon_item(IconName::ArrowLeft, "ArrowLeft"))
                    .child(self.render_icon_item(IconName::ArrowRight, "ArrowRight"))
                    .child(self.render_icon_item(IconName::ArrowUp, "ArrowUp"))
                    .child(self.render_icon_item(IconName::ArrowDown, "ArrowDown"))
                    .child(self.render_icon_item(IconName::Check, "Check"))
                    .child(self.render_icon_item(IconName::ChevronUpDown, "ChevronUpDown"))
                    .child(self.render_icon_item(IconName::ChevronUp, "ChevronUp"))
                    .child(self.render_icon_item(IconName::ChevronDown, "ChevronDown"))
                    .child(self.render_icon_item(IconName::Close, "Close"))
                    .child(self.render_icon_item(IconName::Plus, "Plus"))
                    .child(self.render_icon_item(IconName::Minus, "Minus"))
                    .child(self.render_icon_item(IconName::Search, "Search"))
                    .child(self.render_icon_item(IconName::Settings, "Settings"))
                    .child(self.render_icon_item(IconName::Home, "Home"))
                    .child(self.render_icon_item(IconName::User, "User"))
                    .child(self.render_icon_item(IconName::UserPlus, "UserPlus"))
                    .child(self.render_icon_item(IconName::Bell, "Bell"))
                    .child(self.render_icon_item(IconName::Star, "Star"))
                    .child(self.render_icon_item(IconName::Heart, "Heart"))
                    .child(self.render_icon_item(IconName::Menu, "Menu"))
                    .child(self.render_icon_item(IconName::Info, "Info"))
                    .child(self.render_icon_item(IconName::Warning, "Warning"))
                    .child(self.render_icon_item(IconName::Error, "Error"))
                    .child(self.render_icon_item(IconName::Success, "Success"))
                    .child(self.render_icon_item(IconName::AlertCircle, "AlertCircle"))
                    .child(self.render_icon_item(IconName::AlertTriangle, "AlertTriangle"))
                    .child(self.render_icon_item(IconName::UnfoldMore, "UnfoldMore"))
                    .child(self.render_icon_item(IconName::Send, "Send"))
                    .child(self.render_icon_item(IconName::Attachment, "Attachment"))
                    .child(self.render_icon_item(IconName::Image, "Image"))
                    .child(self.render_icon_item(IconName::LogIn, "LogIn"))
                    .child(self.render_icon_item(IconName::Task, "Task"))
            )
    }

    fn render_icon_item(&self, icon: IconName, name: &str) -> impl IntoElement {
        let name = name.to_string();
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p_6()
            .bg(rgb(0xF9FAFB))
            .border_1()
            .border_color(rgb(0xE5E7EB))
            .rounded(px(8.))
            .gap_3()
            .child(Icon::new(icon).xlarge())
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6B7280))
                    .child(name)
            )
    }
}

