use fluix::prelude::*;
use fluix::ai::{PromptInput, PromptInputEvent, PromptInputConfig, PromptInputVariant, ModelInfo, ModelCapability, PricingInfo};

struct PromptDemo {
    default_prompt: Entity<PromptInput>,
    compact_prompt: Entity<PromptInput>,
    floating_prompt: Entity<PromptInput>,
    with_model_selector_prompt: Entity<PromptInput>,
    scroll_handle: ScrollHandle,
}

impl PromptDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Sample models
        let models = vec![
            ModelInfo {
                id: "gpt-4o".to_string(),
                name: "GPT-4o".to_string(),
                description: Some("Most advanced GPT-4 model".to_string()),
                provider: "OpenAI".to_string(),
                context_length: Some(128000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: Some(PricingInfo { input_price: 0.005, output_price: 0.015, currency: "USD".to_string() }),
            },
            ModelInfo {
                id: "claude-3-5-sonnet-20241022".to_string(),
                name: "Claude 3.5 Sonnet".to_string(),
                description: Some("Latest Claude model".to_string()),
                provider: "Anthropic".to_string(),
                context_length: Some(200000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: Some(PricingInfo { input_price: 0.003, output_price: 0.015, currency: "USD".to_string() }),
            },
            ModelInfo {
                id: "gemini-1.5-pro".to_string(),
                name: "Gemini 1.5 Pro".to_string(),
                description: Some("Google's advanced model".to_string()),
                provider: "Google".to_string(),
                context_length: Some(1000000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: None,
            },
        ];

        // Default prompt input
        let default_prompt = cx.new(|cx| {
            PromptInput::new(cx)
        });

        // Compact prompt input
        let mut compact_config = PromptInputConfig::default();
        compact_config.variant = PromptInputVariant::Compact;
        compact_config.min_height = 40.0;
        compact_config.max_height = 40.0;  // Fixed height - no auto resize
        compact_config.enable_auto_resize = false;  // Disable auto resize

        let compact_prompt = cx.new(|cx| {
            PromptInput::with_config(cx, compact_config)
        });

        // Floating prompt input
        let mut floating_config = PromptInputConfig::default();
        floating_config.variant = PromptInputVariant::Floating;
        floating_config.placeholder = "Ask me anything...".to_string();
        
        let floating_prompt = cx.new(|cx| {
            PromptInput::with_config(cx, floating_config)
        });

        // Prompt with model selector
        let mut model_selector_config = PromptInputConfig::default();
        model_selector_config.show_model_selector = true;
        model_selector_config.available_models = models;
        model_selector_config.placeholder = "Type your message and select a model...".to_string();
        
        let with_model_selector_prompt = cx.new(|cx| {
            PromptInput::with_config(cx, model_selector_config)
        });

        // Subscribe to events
        let _default_subscription = cx.subscribe(&default_prompt, |_this, _prompt, event: &PromptInputEvent, _cx| {
            match event {
                PromptInputEvent::Submit(text) => {
                    println!("Default prompt submitted: {}", text);
                }
                PromptInputEvent::TextChanged(text) => {
                    println!("Default prompt text changed: {}", text);
                }
                _ => {}
            }
        });

        Self {
            default_prompt,
            compact_prompt,
            floating_prompt,
            with_model_selector_prompt,
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for PromptDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()
            .bg(rgb(0xF5F5F5))
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
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x333333))
                                            .child("Prompt Input Demo")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("AI prompt input component with multiple variants")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x374151))
                                            .mb_2()
                                            .child("Default Prompt Input:")
                                    )
                                    .child(self.default_prompt.clone())
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x374151))
                                            .mb_2()
                                            .child("Compact Prompt Input:")
                                    )
                                    .child(self.compact_prompt.clone())
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x374151))
                                            .mb_2()
                                            .child("Floating Prompt Input:")
                                    )
                                    .child(self.floating_prompt.clone())
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x374151))
                                            .mb_2()
                                            .child("Prompt Input with Model Selector:")
                                    )
                                    .child(self.with_model_selector_prompt.clone())
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

            // Position window on the left side of the screen
            let bounds = Bounds {
                origin: Point { x: px(0.0), y: px(0.0) },
                size: size(px(600.0), px(900.0)),
            };
            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Prompt Input Demo".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_background: WindowBackgroundAppearance::Opaque,
                    focus: true,
                    show: true,
                    kind: WindowKind::Normal,
                    is_movable: true,
                    display_id: None,
                    window_min_size: Some(size(px(300.0), px(400.0))),
                    app_id: Some("prompt_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| cx.new(|cx| PromptDemo::new(window, cx)),
            );
        });
}

