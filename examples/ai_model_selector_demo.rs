use fluix::prelude::*;
use fluix::ai::selection::{ModelSelector, ModelSelectorEvent};
use fluix::ai::{ModelInfo, ModelCapability, PricingInfo};
use fluix::components::form::select::DropdownDirection;
use gpui::prelude::FluentBuilder;

struct ModelSelectorDemo {
    scroll_handle: ScrollHandle,
    model_selector: Entity<ModelSelector>,
    compact_selector: Entity<ModelSelector>,
    detailed_selector: Entity<ModelSelector>,
    selected_model: Option<String>,
    messages: Vec<String>,
}

impl ModelSelectorDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
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
            ModelInfo {
                id: "claude-3-opus".to_string(),
                name: "Claude 3 Opus".to_string(),
                description: Some("Anthropic's most capable model".to_string()),
                provider: "Anthropic".to_string(),
                context_length: Some(200000),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::ImageAnalysis,
                    ModelCapability::DocumentAnalysis,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.075,
                    output_price: 0.225,
                    currency: "USD".to_string(),
                }),
            },
            ModelInfo {
                id: "gemini-pro".to_string(),
                name: "Gemini Pro".to_string(),
                description: Some("Google's advanced multimodal model".to_string()),
                provider: "Google".to_string(),
                context_length: Some(32768),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::ImageAnalysis,
                    ModelCapability::FunctionCalling,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.0005,
                    output_price: 0.0015,
                    currency: "USD".to_string(),
                }),
            },
        ];

        // Create different model selectors
        let model_selector = cx.new(|cx| {
            ModelSelector::new_with_models_and_direction(cx, models.clone(), DropdownDirection::Auto)
                .group_by_provider(true)
                .show_pricing(true)
                .show_descriptions(true)
                .show_capabilities(true)
                .dropdown_max_width(450.0) // 设置最大宽度防止超出边界
        });

        let compact_selector = cx.new(|cx| {
            ModelSelector::new_with_models_and_direction(cx, models.clone(), DropdownDirection::Down)
                .compact()
                .max_width(200.0)
                .dropdown_max_width(250.0) // 紧凑模式下的小宽度
        });

        let detailed_selector = cx.new(|cx| {
            ModelSelector::new_with_models_and_direction(cx, models.clone(), DropdownDirection::Up)
                .group_by_provider(false)
                .show_pricing(true)
                .show_descriptions(true)
                .show_capabilities(true)
                .filter_by_capability(ModelCapability::CodeGeneration)
                .dropdown_max_width(400.0) // 筛选模式下的中等宽度
        });

        // Subscribe to model selector events
        let _subscription1 = cx.subscribe(&model_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            this.handle_model_event("Main", event, cx);
        });

        let _subscription2 = cx.subscribe(&compact_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            this.handle_model_event("Compact", event, cx);
        });

        let _subscription3 = cx.subscribe(&detailed_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            this.handle_model_event("Detailed", event, cx);
        });

        Self {
            scroll_handle: ScrollHandle::new(),
            model_selector,
            compact_selector,
            detailed_selector,
            selected_model: None,
            messages: vec![
                "Welcome to the Model Selector Demo!".to_string(),
                "Try selecting different models to see the events.".to_string(),
                "Each selector has different configurations.".to_string(),
            ],
        }
    }

    fn handle_model_event(&mut self, selector_name: &str, event: &ModelSelectorEvent, cx: &mut Context<Self>) {
        match event {
            ModelSelectorEvent::ModelChanged(model_id) => {
                self.selected_model = Some(model_id.clone());
                self.messages.push(format!(
                    "{} selector: Model changed to {}",
                    selector_name, model_id
                ));
                cx.notify();
            }
            ModelSelectorEvent::ProviderChanged(provider) => {
                self.messages.push(format!(
                    "{} selector: Provider filter changed to {}",
                    selector_name, provider
                ));
                cx.notify();
            }
            ModelSelectorEvent::CapabilityChanged(capability) => {
                self.messages.push(format!(
                    "{} selector: Capability filter changed to {:?}",
                    selector_name, capability
                ));
                cx.notify();
            }
            ModelSelectorEvent::ShowPricingToggled(show) => {
                self.messages.push(format!(
                    "{} selector: Show pricing toggled to {}",
                    selector_name, show
                ));
                cx.notify();
            }
        }
    }
}

impl Render for ModelSelectorDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                                            .child("AI Model Selector Demo")
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
                        // Left panel - Model selectors
                        div()
                            .flex()
                            .flex_col()
                            .w(px(400.))
                            .gap_6()
                            .child(
                                // Main selector
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
                                            .child("Full Featured Selector")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Grouped by provider, with width limit (450px)")
                                    )
                                    .child(self.model_selector.clone())
                            )
                            .child(
                                // Compact selector
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
                                            .child("Compact Selector")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Compact design with 250px width limit")
                                    )
                                    .child(self.compact_selector.clone())
                            )
                            .child(
                                // Detailed selector
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
                                            .child("Filtered Selector")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Code generation filter with 400px width limit")
                                    )
                                    .child(self.detailed_selector.clone())
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
                            .when(self.selected_model.is_some(), |this| {
                                this.child(
                                    div()
                                        .p_3()
                                        .bg(rgb(0xE7F3FF))
                                        .rounded_lg()
                                        .border_1()
                                        .border_color(rgb(0xBFDBFE))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(0x1E40AF))
                                                .child(format!(
                                                    "Currently selected: {}",
                                                    self.selected_model.as_ref().unwrap()
                                                ))
                                        )
                                )
                            })
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
                        title: Some("AI Model Selector Demo".into()),
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
                    app_id: Some("ai_model_selector_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| cx.new(|cx| ModelSelectorDemo::new(window, cx)),
            );
        });
}
