use gpui::*;
use fluix::{Select, SelectOption, SelectOptionGroup};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(600.), px(800.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Select Grouped Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectGroupedDemo::new(window, cx))
        }).unwrap();
    });
}

struct SelectGroupedDemo {
    scroll_handle: ScrollHandle,
    select: Entity<Select>,
}

impl SelectGroupedDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Select country...")
                .option_groups(vec![
                    SelectOptionGroup::new("North America", vec![
                        SelectOption::new("us", "United States"),
                        SelectOption::new("ca", "Canada"),
                        SelectOption::new("mx", "Mexico"),
                    ]),
                    SelectOptionGroup::new("Europe", vec![
                        SelectOption::new("uk", "United Kingdom"),
                        SelectOption::new("de", "Germany"),
                        SelectOption::new("fr", "France"),
                    ]),
                    SelectOptionGroup::new("Asia", vec![
                        SelectOption::new("cn", "China"),
                        SelectOption::new("jp", "Japan"),
                    ]),
                ])
        });

        Self {
            scroll_handle,
            select,
        }
    }
}

impl Render for SelectGroupedDemo {
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
                            .pb(px(400.))
                            .gap_8()
                            .items_start()
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
                                            .child("Grouped Select")
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .child(self.select.clone())
                                    )
                            )
                    )
            )
    }
}

