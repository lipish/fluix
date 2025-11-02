use gpui::*;
use fluix::{Combobox, SelectOption};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(600.), px(400.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Combobox Pre-selected Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| ComboboxPreselectedDemo::new(window, cx))
        }).unwrap();
    });
}

struct ComboboxPreselectedDemo {
    scroll_handle: ScrollHandle,
    combobox: Entity<Combobox>,
}

impl ComboboxPreselectedDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let combobox = cx.new(|cx| {
            Combobox::new(cx)
                .placeholder("Search or select a language...")
                .value("rust")
                .input_value("Rust")
                .options(vec![
                    SelectOption::new("rust", "Rust"),
                    SelectOption::new("typescript", "TypeScript"),
                    SelectOption::new("python", "Python"),
                    SelectOption::new("go", "Go"),
                ])
        });

        Self {
            scroll_handle,
            combobox,
        }
    }
}

impl Render for ComboboxPreselectedDemo {
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
                            .h_full()
                            .bg(rgb(0xF5F5F5))
                            .p_8()
                            .gap_8()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(400.))
                                    .gap_4()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .rounded(px(8.))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .mb_2()
                                            .child("Pre-selected Combobox")
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .child(self.combobox.clone())
                                    )
                            )
                    )
            )
    }
}

