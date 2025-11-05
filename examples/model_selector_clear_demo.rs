use fluix::prelude::*;
use fluix::ai::selection::{ModelSelector, ModelSelectorEvent};
use fluix::ai::{ModelInfo, ModelCapability, PricingInfo};
use fluix::components::form::select::DropdownDirection;

struct ModelSelectorClearDemo {
    scroll_handle: ScrollHandle,
    model_selector: Entity<ModelSelector>,
    clear_button: Entity<Button>,
    messages: Vec<String>,
}

impl ModelSelectorClearDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Create sample models
        let models = vec![
            ModelInfo {
                id: "gpt-4".to_string(),
                name: "GPT-4".to_string(),
                description: Some("Most capable GPT model for complex tasks".to_string()),
                provider: "OpenAI".to_string(),
                context_length: Some(8192),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::FunctionCalling,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.03,
                    output_price: 0.06,
                    currency: "USD".to_string(),
                }),
            },
            ModelInfo {
                id: "gpt-3.5-turbo".to_string(),
                name: "GPT-3.5 Turbo".to_string(),
                description: Some("Fast and efficient model for most tasks".to_string()),
                provider: "OpenAI".to_string(),
                context_length: Some(4096),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.001,
                    output_price: 0.002,
                    currency: "USD".to_string(),
                }),
            },
            ModelInfo {
                id: "claude-3-sonnet".to_string(),
                name: "Claude 3 Sonnet".to_string(),
                description: Some("Anthropic's balanced model for various tasks".to_string()),
                provider: "Anthropic".to_string(),
                context_length: Some(200000),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::ImageAnalysis,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.015,
                    output_price: 0.075,
                    currency: "USD".to_string(),
                }),
            },
        ];

        // Create model selector with clear functionality
        let model_selector = cx.new(|cx| {
            ModelSelector::new_with_models_and_direction(cx, models, DropdownDirection::Auto)
                .show_pricing(true)
                .show_descriptions(true)
                .dropdown_max_width(400.0)
        });

        let _subscription1 = cx.subscribe(&model_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            this.handle_model_event(event, cx);
        });

        // Create clear button
        let clear_button = cx.new(|_| {
            Button::new("Clear")
                .variant(ButtonVariant::Secondary)
        });

        let _subscription2 = cx.subscribe_in(&clear_button, window, |this, _button, event: &ButtonEvent, _window, cx| {
            match event {
                ButtonEvent::Click => {
                    this.clear_selection(cx);
                }
            }
        });

        Self {
            scroll_handle: ScrollHandle::new(),
            model_selector,
            clear_button,
            messages: vec![
                "Welcome to the Model Selector Clear Demo!".to_string(),
                "Select a model, then click 'Clear Selection' to unselect it.".to_string(),
                "Notice how the selection state changes.".to_string(),
            ],
        }
    }

    fn handle_model_event(&mut self, event: &ModelSelectorEvent, cx: &mut Context<Self>) {
        match event {
            ModelSelectorEvent::ModelChanged(model_id) => {
                if model_id.is_empty() {
                    self.messages.push("Selection cleared - no model selected".to_string());
                } else {
                    self.messages.push(format!("Selected model: {}", model_id));
                }
                cx.notify();
            }
            _ => {}
        }
    }

    fn clear_selection(&mut self, cx: &mut Context<Self>) {
        // Manually clear the selection
        self.model_selector.update(cx, |selector, cx| {
            selector.clear_selection(cx);
        });
        self.messages.push("Manual clear triggered".to_string());
        cx.notify();
    }
}

impl Render for ModelSelectorClearDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden() // Layer 1: Outer container
            .child(
                div()
                    .id("scroll-container")
                    .size_full()
                    .overflow_y_scroll() // Layer 2: Scroll container
                    .track_scroll(&self.scroll_handle)
                    .child(
                        div() // Layer 3: Content container
                            .flex()
                            .flex_col()
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
                                            .child("Model Selector Clear Demo")
                                    )
                            )
                            .child(
                                // Main content
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap_6()
                                    .p_6()
                                    .child(
                                        // Left panel - Model selector and controls
                                        div()
                                            .flex()
                                            .flex_col()
                                            .w(px(400.))
                                            .gap_6()
                                            .child(
                                                // Model selector
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
                                                            .child("Model Selector")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0x666666))
                                                            .child("Select a model from the dropdown")
                                                    )
                                                    .child(
                                                        div()
                                                            .w_full()
                                                            .max_w(px(350.)) // 限制最大宽度
                                                            .child(self.model_selector.clone())
                                                    )
                                            )
                                            .child(
                                                // Clear button
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
                                                            .child("Clear Selection")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0x666666))
                                                            .child("Click to clear the current selection")
                                                    )
                                                    .child(self.clear_button.clone())
                                            )
                                            .child(
                                                // Instructions
                                                div()
                                                    .p_4()
                                                    .bg(rgb(0xFFF7ED))
                                                    .rounded_lg()
                                                    .border_1()
                                                    .border_color(rgb(0xFED7AA))
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0x9A3412))
                                                            .child("💡 Try selecting a model, then click 'Clear' to see the unselect functionality. You can also select 'No model selected' from the dropdown to clear the selection.")
                                                    )
                                            )
                                    )
                                    .child(
                                        // Right panel - Event log
                                        div()
                                            .flex()
                                            .flex_col()
                                            .flex_1()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .text_color(rgb(0x1A1A1A))
                                                    .child("Event Log")
                                            )
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
                                                    .children(
                                                        self.messages.iter().rev().take(20).map(|message| {
                                                            div()
                                                                .p_2()
                                                                .bg(rgb(0xF8F8F8))
                                                                .rounded_md()
                                                                .text_sm()
                                                                .child(message.clone())
                                                        })
                                                    )
                                            )
                                            .child(
                                                // Current selection status
                                                div()
                                                    .p_4()
                                                    .bg(rgb(0xE7F3FF))
                                                    .rounded_lg()
                                                    .border_1()
                                                    .border_color(rgb(0xBFDBFE))
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0x1E40AF))
                                                            .child(
                                                                self.model_selector.read(cx).get_selected_model()
                                                                    .map(|model| format!("Current selection: {}", model))
                                                                    .unwrap_or_else(|| "No model selected".to_string())
                                                            )
                                                    )
                                            )
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new()
        .with_assets(fluix::Assets)
        .run(|cx| {
            cx.activate(true);
            
            let bounds = Bounds::centered(None, size(px(1000.0), px(700.0)), cx);
            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Model Selector Clear Demo".into()),
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
                    app_id: Some("model_selector_clear_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| cx.new(|cx| ModelSelectorClearDemo::new(window, cx)),
            );
        });
}
