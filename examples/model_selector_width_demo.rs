use fluix::prelude::*;
use fluix::ai::selection::{ModelSelector, ModelSelectorEvent};
use fluix::ai::{ModelInfo, ModelCapability, PricingInfo};
use fluix::components::form::select::DropdownDirection;

struct ModelSelectorWidthDemo {
    scroll_handle: ScrollHandle,
    unlimited_selector: Entity<ModelSelector>,
    limited_selector: Entity<ModelSelector>,
    fixed_selector: Entity<ModelSelector>,
    messages: Vec<String>,
}

impl ModelSelectorWidthDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Create models with very long descriptions to test width limits
        let models = vec![
            ModelInfo {
                id: "gpt-4-turbo-preview".to_string(),
                name: "GPT-4 Turbo Preview".to_string(),
                description: Some("The latest GPT-4 model with improved instruction following, JSON mode, reproducible outputs, parallel function calling, and more. This model has a very long description to test width limits and text wrapping behavior in dropdown menus.".to_string()),
                provider: "OpenAI".to_string(),
                context_length: Some(128000),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::FunctionCalling,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.01,
                    output_price: 0.03,
                    currency: "USD".to_string(),
                }),
            },
            ModelInfo {
                id: "claude-3-opus-20240229".to_string(),
                name: "Claude 3 Opus (20240229)".to_string(),
                description: Some("Anthropic's most powerful model with exceptional performance on highly complex tasks, superior performance on math, coding, reasoning, and multilingual understanding. Features extremely long context window and advanced reasoning capabilities that make it suitable for the most demanding applications.".to_string()),
                provider: "Anthropic".to_string(),
                context_length: Some(200000),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::ImageAnalysis,
                    ModelCapability::DocumentAnalysis,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.015,
                    output_price: 0.075,
                    currency: "USD".to_string(),
                }),
            },
            ModelInfo {
                id: "gemini-1.5-pro-latest".to_string(),
                name: "Gemini 1.5 Pro (Latest)".to_string(),
                description: Some("Google's most advanced multimodal model with breakthrough long context understanding, advanced reasoning, and multimodal capabilities including text, images, audio, video, and code. Optimized for complex reasoning tasks and creative applications with industry-leading context length.".to_string()),
                provider: "Google".to_string(),
                context_length: Some(2000000),
                capabilities: vec![
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::ImageGeneration,
                    ModelCapability::ImageAnalysis,
                    ModelCapability::FunctionCalling,
                ],
                pricing: Some(PricingInfo {
                    input_price: 0.0035,
                    output_price: 0.0105,
                    currency: "USD".to_string(),
                }),
            },
        ];

        // Create selectors with different width controls
        let unlimited_selector = cx.new(|cx| {
            ModelSelector::new_with_models_and_direction(cx, models.clone(), DropdownDirection::Down)
                .show_pricing(true)
                .show_descriptions(true)
                // No width limit - will expand as needed
        });

        let limited_selector = cx.new(|cx| {
            ModelSelector::new_with_models_and_direction(cx, models.clone(), DropdownDirection::Down)
                .show_pricing(true)
                .show_descriptions(true)
                .dropdown_max_width(300.0) // Limited to 300px for better wrapping demo
        });

        let fixed_selector = cx.new(|cx| {
            ModelSelector::new_with_models_and_direction(cx, models.clone(), DropdownDirection::Down)
                .show_pricing(true)
                .show_descriptions(true)
                .dropdown_fixed_width(250.0) // Fixed at 250px for better wrapping demo
        });

        // Subscribe to events
        let _subscription1 = cx.subscribe(&unlimited_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            this.handle_model_event("Unlimited", event, cx);
        });

        let _subscription2 = cx.subscribe(&limited_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            this.handle_model_event("Limited", event, cx);
        });

        let _subscription3 = cx.subscribe(&fixed_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            this.handle_model_event("Fixed", event, cx);
        });

        Self {
            scroll_handle: ScrollHandle::new(),
            unlimited_selector,
            limited_selector,
            fixed_selector,
            messages: vec![
                "Welcome to the Model Selector Width Demo!".to_string(),
                "Test different width controls:".to_string(),
                "- Unlimited: Expands to fit content".to_string(),
                "- Limited: Max width of 300px with text wrapping".to_string(),
                "- Fixed: Fixed width of 250px with text wrapping".to_string(),
                "Notice how long descriptions are handled differently.".to_string(),
            ],
        }
    }

    fn handle_model_event(&mut self, selector_type: &str, event: &ModelSelectorEvent, cx: &mut Context<Self>) {
        match event {
            ModelSelectorEvent::ModelChanged(model_id) => {
                self.messages.push(format!(
                    "{} selector: Selected {}",
                    selector_type, model_id
                ));
                cx.notify();
            }
            _ => {}
        }
    }
}

impl Render for ModelSelectorWidthDemo {
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
                                            .child("Model Selector Width Control Demo")
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
                        // Left panel - Selectors
                        div()
                            .flex()
                            .flex_col()
                            .w(px(500.))
                            .gap_6()
                            .child(
                                // Unlimited width selector
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
                                            .child("Unlimited Width")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Dropdown expands to fit content (may exceed screen)")
                                    )
                                    .child(self.unlimited_selector.clone())
                            )
                            .child(
                                // Limited width selector
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
                                            .child("Limited Width (300px)")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Dropdown limited to 300px max width with text wrapping")
                                    )
                                    .child(self.limited_selector.clone())
                            )
                            .child(
                                // Fixed width selector
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
                                            .child("Fixed Width (250px)")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Dropdown always 250px wide with text wrapping")
                                    )
                                    .child(self.fixed_selector.clone())
                            )
                    )
                    .child(
                        // Right panel - Event log and info
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
                                // Info panel
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
                                            .child("💡 Notice how the dropdown menus handle long content differently based on width settings. The unlimited version may extend beyond screen boundaries, while limited and fixed versions provide better control.")
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
            
            let bounds = Bounds::centered(None, size(px(1100.0), px(700.0)), cx);
            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Model Selector Width Demo".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_background: WindowBackgroundAppearance::Opaque,
                    focus: true,
                    show: true,
                    kind: WindowKind::Normal,
                    is_movable: true,
                    display_id: None,
                    window_min_size: Some(size(px(900.0), px(600.0))),
                    app_id: Some("model_selector_width_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| cx.new(|cx| ModelSelectorWidthDemo::new(window, cx)),
            );
        });
}
