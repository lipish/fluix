use fluix::prelude::*;

struct DropdownDirectionDemo {
    select_down: Entity<Select>,
    select_up: Entity<Select>,
    select_auto: Entity<Select>,
    messages: Vec<String>,
}

impl DropdownDirectionDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Create sample options
        let options = vec![
            SelectOption::new("option1", "Option 1 - Short"),
            SelectOption::new("option2", "Option 2 - Medium length option"),
            SelectOption::new("option3", "Option 3 - Very long option with lots of text"),
            SelectOption::new("option4", "Option 4 - Another option"),
            SelectOption::new("option5", "Option 5 - Final option"),
        ];

        // Create selects with different dropdown directions
        let select_down = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Dropdown Down")
                .options(options.clone())
                .dropdown_direction(DropdownDirection::Down)
        });

        let select_up = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Dropdown Up")
                .options(options.clone())
                .dropdown_direction(DropdownDirection::Up)
        });

        let select_auto = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Dropdown Auto")
                .options(options.clone())
                .dropdown_direction(DropdownDirection::Auto)
        });

        // Subscribe to events
        let _subscription1 =
            cx.subscribe(&select_down, |this, _select, event: &SelectEvent, cx| {
                this.handle_select_event("Down", event, cx);
            });

        let _subscription2 = cx.subscribe(&select_up, |this, _select, event: &SelectEvent, cx| {
            this.handle_select_event("Up", event, cx);
        });

        let _subscription3 =
            cx.subscribe(&select_auto, |this, _select, event: &SelectEvent, cx| {
                this.handle_select_event("Auto", event, cx);
            });

        Self {
            select_down,
            select_up,
            select_auto,
            messages: vec![
                "Welcome to the Dropdown Direction Demo!".to_string(),
                "Test different dropdown directions:".to_string(),
                "- Down: Always expands downward".to_string(),
                "- Up: Always expands upward".to_string(),
                "- Auto: Chooses best direction based on space".to_string(),
            ],
        }
    }

    fn handle_select_event(
        &mut self,
        selector_type: &str,
        event: &SelectEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            SelectEvent::Changed(value) => {
                self.messages
                    .push(format!("{} selector: Selected {}", selector_type, value));
                cx.notify();
            }
            _ => {}
        }
    }
}

impl Render for DropdownDirectionDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xF5F5F5))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(60.))
                    .bg(rgb(0xFFFFFF))
                    .border_b_1()
                    .border_color(rgb(0xE0E0E0))
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0x1A1A1A))
                            .child("Select Dropdown Direction Demo"),
                    ),
            )
            .child(
                // Main content
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .gap_6()
                    .p_6()
                    .child(
                        // Left panel - Selects
                        div()
                            .flex()
                            .flex_col()
                            .w(px(400.))
                            .gap_8()
                            .child(
                                // Top area - Down dropdown
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .text_lg()
                                            .text_color(rgb(0x1A1A1A))
                                            .child("Dropdown Down"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Always expands downward"),
                                    )
                                    .child(self.select_down.clone()),
                            )
                            .child(
                                // Middle area - Auto dropdown
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .text_lg()
                                            .text_color(rgb(0x1A1A1A))
                                            .child("Dropdown Auto"),
                                    )
                                    .child(div().text_sm().text_color(rgb(0x666666)).child(
                                        "Auto-detects best direction based on available space",
                                    ))
                                    .child(self.select_auto.clone()),
                            )
                            .child(
                                // Bottom area - Up dropdown
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .text_lg()
                                            .text_color(rgb(0x1A1A1A))
                                            .child("Dropdown Up"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Always expands upward"),
                                    )
                                    .child(self.select_up.clone()),
                            ),
                    )
                    .child(
                        // Right panel - Event log
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap_3()
                            .child(div().text_lg().text_color(rgb(0x1A1A1A)).child("Event Log"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .p_4()
                                    .gap_2()
                                    .overflow_hidden()
                                    .children(self.messages.iter().rev().take(20).map(|message| {
                                        div()
                                            .p_2()
                                            .bg(rgb(0xF8F8F8))
                                            .rounded_md()
                                            .text_sm()
                                            .child(message.clone())
                                    })),
                            ),
                    ),
            )
            .child(
                // Footer with instructions
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(50.))
                    .bg(rgb(0xFFFFFF))
                    .border_t_1()
                    .border_color(rgb(0xE0E0E0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("Click on the selects to test different dropdown directions"),
                    ),
            )
    }
}

fn main() {
    Application::new().with_assets(fluix::Assets).run(|cx| {
        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(900.0), px(700.0)), cx);
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Select Dropdown Direction Demo".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_background: WindowBackgroundAppearance::Opaque,
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                display_id: None,
                window_min_size: Some(size(px(800.0), px(600.0))),
                app_id: Some("select_dropdown_direction_demo".into()),
                is_minimizable: true,
                is_resizable: true,
                window_decorations: Some(WindowDecorations::Server),
                tabbing_identifier: None,
            },
            |window, cx| cx.new(|cx| DropdownDirectionDemo::new(window, cx)),
        );
    });
}
