use gpui::*;
use fluix::{Tabs, TabItem, TabsEvent};

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(600.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Tabs Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| TabsDemo::new(window, cx))
        }).unwrap();
    });
}

struct TabsDemo {
    tabs: Entity<Tabs>,
}

impl TabsDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tabs = cx.new(|_cx| {
            Tabs::new()
                .tabs(vec![
                    TabItem::new("Tab 1", || {
                        div()
                            .text_color(rgb(0x666666))
                            .child("Tab 1 content")
                    }),
                    TabItem::new("Tab 2", || {
                        div()
                            .text_color(rgb(0x666666))
                            .child("Tab 2 content")
                    }),
                    TabItem::new("Tab 3", || {
                        div()
                            .text_color(rgb(0x666666))
                            .child("Tab 3 content")
                    }),
                ])
                .active_index(1) // Start with Tab 2 active
        });

        // Subscribe to tab change events
        let _sub = cx.subscribe_in(&tabs, window, Self::on_tab_changed);

        Self {
            tabs,
        }
    }

    fn on_tab_changed(
        &mut self,
        _tabs: &Entity<Tabs>,
        event: &TabsEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        match event {
            TabsEvent::TabChanged { index } => {
                println!("Tab changed to index: {}", index);
            }
        }
    }
}

impl Render for TabsDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p_8()
            .child(self.tabs.clone())
    }
}

