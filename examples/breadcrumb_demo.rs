use gpui::prelude::FluentBuilder;
use gpui::*;
use fluix::{Breadcrumb, BreadcrumbItem, BreadcrumbEvent, BreadcrumbSeparator, ComponentSize};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(700.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Breadcrumb Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| BreadcrumbDemo::new(window, cx))
        }).unwrap();
    });
}

struct BreadcrumbDemo {
    scroll_handle: ScrollHandle,
    basic_breadcrumb: Entity<Breadcrumb>,
    slash_breadcrumb: Entity<Breadcrumb>,
    arrow_breadcrumb: Entity<Breadcrumb>,
    custom_breadcrumb: Entity<Breadcrumb>,
    clicked_index: Option<usize>,
}

impl BreadcrumbDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        let basic_breadcrumb = cx.new(|cx| {
            Breadcrumb::new(cx)
                .items(vec![
                    BreadcrumbItem::new("Home"),
                    BreadcrumbItem::new("Products"),
                    BreadcrumbItem::new("Electronics"),
                    BreadcrumbItem::new("Laptop"),
                ])
        });

        let slash_breadcrumb = cx.new(|cx| {
            Breadcrumb::new(cx)
                .items(vec![
                    BreadcrumbItem::new("Home"),
                    BreadcrumbItem::new("Documents"),
                    BreadcrumbItem::new("Projects"),
                    BreadcrumbItem::new("fluix"),
                ])
                .separator(BreadcrumbSeparator::Slash)
        });

        let arrow_breadcrumb = cx.new(|cx| {
            Breadcrumb::new(cx)
                .items(vec![
                    BreadcrumbItem::new("Home"),
                    BreadcrumbItem::new("Settings"),
                    BreadcrumbItem::new("Account"),
                ])
                .separator(BreadcrumbSeparator::Arrow)
        });

        let custom_breadcrumb = cx.new(|cx| {
            Breadcrumb::new(cx)
                .items(vec![
                    BreadcrumbItem::new("Home"),
                    BreadcrumbItem::new("Blog"),
                    BreadcrumbItem::new("2024"),
                    BreadcrumbItem::new("December"),
                ])
                .separator(BreadcrumbSeparator::Custom('>'))
        });

        cx.subscribe_in(&basic_breadcrumb, _window, |this: &mut Self, _breadcrumb, event: &BreadcrumbEvent, _window, cx| {
            let BreadcrumbEvent::ItemClicked { index } = event;
            this.clicked_index = Some(*index);
            println!("Breadcrumb item clicked: {}", index);
            cx.notify();
        }).detach();

        Self {
            scroll_handle,
            basic_breadcrumb,
            slash_breadcrumb,
            arrow_breadcrumb,
            custom_breadcrumb,
            clicked_index: None,
        }
    }
}

impl Render for BreadcrumbDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .bg(rgb(0xF5F5F5))
                            .p_8()
                            .gap_8()
                            .child(
                                // Header
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x333333))
                                            .child("Fluix Breadcrumb Component")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Navigation breadcrumb component with various separators")
                                    )
                            )
                            .child(
                                // Basic breadcrumb section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .rounded(px(8.))
                                    .gap_6()
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
                                                    .child("Basic Breadcrumb")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Default chevron separator")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(self.basic_breadcrumb.clone())
                                            .when_some(self.clicked_index, |this, idx| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Last clicked item index: {}", idx))
                                                )
                                            })
                                    )
                            )
                            .child(
                                // Slash separator section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .rounded(px(8.))
                                    .gap_6()
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
                                                    .child("Slash Separator")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Using slash (/) as separator")
                                            )
                                    )
                                    .child(self.slash_breadcrumb.clone())
                            )
                            .child(
                                // Arrow separator section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .rounded(px(8.))
                                    .gap_6()
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
                                                    .child("Arrow Separator")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Using arrow (→) as separator")
                                            )
                                    )
                                    .child(self.arrow_breadcrumb.clone())
                            )
                            .child(
                                // Custom separator section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .rounded(px(8.))
                                    .gap_6()
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
                                                    .child("Custom Separator")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Using custom character (>) as separator")
                                            )
                                    )
                                    .child(self.custom_breadcrumb.clone())
                            )
                            .child(
                                // Sizes section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .rounded(px(8.))
                                    .gap_6()
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
                                                    .child("Different Sizes")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Breadcrumb with different sizes")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(
                                                cx.new(|cx| {
                                                    Breadcrumb::new(cx)
                                                        .items(vec![
                                                            BreadcrumbItem::new("Home"),
                                                            BreadcrumbItem::new("Small"),
                                                        ])
                                                        .size(ComponentSize::Small)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Breadcrumb::new(cx)
                                                        .items(vec![
                                                            BreadcrumbItem::new("Home"),
                                                            BreadcrumbItem::new("Medium"),
                                                        ])
                                                        .size(ComponentSize::Medium)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Breadcrumb::new(cx)
                                                        .items(vec![
                                                            BreadcrumbItem::new("Home"),
                                                            BreadcrumbItem::new("Large"),
                                                        ])
                                                        .size(ComponentSize::Large)
                                                }).clone()
                                            )
                                    )
                            )
                    )
            )
    }
}

