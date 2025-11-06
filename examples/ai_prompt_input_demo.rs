use fluix::prelude::*;
use fluix::ai::input::{PromptInput, PromptInputEvent};
use fluix::ai::{ModelInfo, ModelCapability, PricingInfo};

struct PromptInputDemo {
    prompt_input: Entity<PromptInput>,
    messages: Vec<String>,
}

impl PromptInputDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Get models dynamically from llm-link
        let models = ModelInfo::default_models_from_llm_link();
        
        // If no models are available, fallback to sample models
        let models = if models.is_empty() {
            vec![
                ModelInfo {
                    id: "gpt-4".to_string(),
                    name: "GPT-4".to_string(),
                    description: Some("Most capable GPT model".to_string()),
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
                    id: "claude-3".to_string(),
                    name: "Claude 3 Sonnet".to_string(),
                    description: Some("Anthropic's latest model".to_string()),
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
            ]
        } else {
            models
        };

        // Create prompt input with models
        let prompt_input = cx.new(|cx| {
            PromptInput::new(cx)
                .placeholder("Ask me anything...")
                .with_models(models.clone())
                .default_model(models.first().map(|m| m.id.clone()).unwrap_or_default())
                .enable_attachments(true)
                .variant(fluix::ai::input::PromptInputVariant::Embedded)
                .height(80.0, 300.0)
                .bg_color(rgb(0xF5F5F5))
        });

        // Subscribe to prompt input events
        let _subscription = cx.subscribe(&prompt_input, |this, _input, event: &PromptInputEvent, cx| {
            match event {
                PromptInputEvent::Submit(text) => {
                    this.messages.push(format!("User: {}", text));
                    this.messages.push(format!("Assistant: I received your message: \"{}\"", text));
                    cx.notify();
                }
                PromptInputEvent::ModelChanged(model) => {
                    this.messages.push(format!("System: Model changed to {}", model));
                    cx.notify();
                }
                PromptInputEvent::AttachFile(attachment) => {
                    this.messages.push(format!("System: File attached: {}", attachment.name));
                    cx.notify();
                }
                PromptInputEvent::AttachImage(attachment) => {
                    this.messages.push(format!("System: Image attached: {}", attachment.name));
                    cx.notify();
                }
                _ => {}
            }
        });

        Self {
            prompt_input,
            messages: vec![
                "Welcome to the AI Prompt Input Demo!".to_string(),
                "Try typing a message and pressing Enter.".to_string(),
                "You can also change models and attach files.".to_string(),
            ],
        }
    }
}

impl Render for PromptInputDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF)) // White background since PromptInput has gray background
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
                            .child("AI Prompt Input Demo")
                    )
            )
            .child(
                // Main content
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_6()
                    .gap_4()
                    .child(
                        // Messages area
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
                            .children(
                                self.messages.iter().map(|message| {
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
                        // Prompt input
                        div()
                            .w_full()
                            .child(self.prompt_input.clone())
                    )
            )
    }
}

fn main() {
    Application::new()
        .with_assets(fluix::Assets)
        .run(|cx| {
            cx.activate(true);

            let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
            let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("AI Prompt Input Demo".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_background: WindowBackgroundAppearance::Opaque,
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                display_id: None,
                window_min_size: Some(size(px(600.0), px(400.0))),
                app_id: Some("ai_prompt_input_demo".into()),
                is_minimizable: true,
                is_resizable: true,
                window_decorations: Some(WindowDecorations::Server),
                tabbing_identifier: None,
            },
            |window, cx| cx.new(|cx| PromptInputDemo::new(window, cx)),
        );
        });
}
