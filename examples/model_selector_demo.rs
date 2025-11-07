use fluix::prelude::*;
use fluix::ai::{ModelSelector, ModelSelectorEvent, ModelInfo, ModelCapability, PricingInfo, ModelSelectorConfig};
use fluix::components::form::select::{DropdownDirection, DropdownAlignment};

struct ModelSelectorDemo {
    model_selector: Entity<ModelSelector>,
    compact_model_selector: Entity<ModelSelector>,
    searchable_model_selector: Entity<ModelSelector>,
    selected_model: String,
    scroll_handle: ScrollHandle,
    alignment: DropdownAlignment,
    direction: DropdownDirection,
}

impl ModelSelectorDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Get models dynamically from llm-link
        let models = ModelInfo::default_models_from_llm_link();

        // If no models are available, fallback to sample models
        let models = if models.is_empty() {
            vec![
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
                    id: "gpt-4o-mini".to_string(),
                    name: "GPT-4o Mini".to_string(),
                    description: Some("Faster and cheaper GPT-4 variant".to_string()),
                    provider: "OpenAI".to_string(),
                    context_length: Some(128000),
                    capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                    pricing: Some(PricingInfo { input_price: 0.00015, output_price: 0.0006, currency: "USD".to_string() }),
                },
                ModelInfo {
                    id: "claude-3-5-sonnet-20241022".to_string(),
                    name: "Claude 3.5 Sonnet".to_string(),
                    description: Some("Latest Claude model with enhanced reasoning".to_string()),
                    provider: "Anthropic".to_string(),
                    context_length: Some(200000),
                    capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                    pricing: Some(PricingInfo { input_price: 0.003, output_price: 0.015, currency: "USD".to_string() }),
                },
                ModelInfo {
                    id: "gemini-1.5-pro".to_string(),
                    name: "Gemini 1.5 Pro".to_string(),
                    description: Some("Google's most capable AI model".to_string()),
                    provider: "Google".to_string(),
                    context_length: Some(1000000),
                    capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                    pricing: Some(PricingInfo { input_price: 0.00125, output_price: 0.005, currency: "USD".to_string() }),
                },
            ]
        } else {
            models
        };

        // Create model selector
        let model_selector = cx.new(|cx| {
            ModelSelector::new_with_models(cx, models.clone())
                .show_all_models()
                .clean_style(true)
                .dropdown_direction(DropdownDirection::Up)
                .dropdown_alignment(DropdownAlignment::Right)
        });

        // Create compact model selector without groups
        let mut compact_config = ModelSelectorConfig::default();
        compact_config.group_by_provider = false;
        compact_config.compact = true;
        compact_config.clean_style = true;
        compact_config.dropdown_direction = DropdownDirection::Up;
        compact_config.dropdown_alignment = DropdownAlignment::Right;
        
        let compact_model_selector = cx.new(|cx| {
            ModelSelector::new_with_config(cx, models.clone(), compact_config)
        });

        // Create searchable model selector with more models to demonstrate filtering
        let mut searchable_models = models.clone();
        // Add more sample models to demonstrate search functionality
        searchable_models.extend(vec![
            ModelInfo {
                id: "gpt-3.5-turbo".to_string(),
                name: "GPT-3.5 Turbo".to_string(),
                description: Some("Fast and efficient GPT-3.5 model".to_string()),
                provider: "OpenAI".to_string(),
                context_length: Some(16385),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: Some(PricingInfo { input_price: 0.0005, output_price: 0.0015, currency: "USD".to_string() }),
            },
            ModelInfo {
                id: "claude-3-opus-20240229".to_string(),
                name: "Claude 3 Opus".to_string(),
                description: Some("Most capable Claude 3 model".to_string()),
                provider: "Anthropic".to_string(),
                context_length: Some(200000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: Some(PricingInfo { input_price: 0.015, output_price: 0.075, currency: "USD".to_string() }),
            },
            ModelInfo {
                id: "claude-3-haiku-20240307".to_string(),
                name: "Claude 3 Haiku".to_string(),
                description: Some("Fastest Claude 3 model".to_string()),
                provider: "Anthropic".to_string(),
                context_length: Some(200000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: Some(PricingInfo { input_price: 0.00025, output_price: 0.00125, currency: "USD".to_string() }),
            },
            ModelInfo {
                id: "gemini-1.5-flash".to_string(),
                name: "Gemini 1.5 Flash".to_string(),
                description: Some("Fast and efficient Gemini model".to_string()),
                provider: "Google".to_string(),
                context_length: Some(1000000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: Some(PricingInfo { input_price: 0.000075, output_price: 0.0003, currency: "USD".to_string() }),
            },
            ModelInfo {
                id: "llama-3.1-405b-instruct".to_string(),
                name: "Llama 3.1 405B".to_string(),
                description: Some("Meta's largest open-source model".to_string()),
                provider: "Meta".to_string(),
                context_length: Some(128000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: None,
            },
            ModelInfo {
                id: "llama-3.1-70b-instruct".to_string(),
                name: "Llama 3.1 70B".to_string(),
                description: Some("Meta's efficient open-source model".to_string()),
                provider: "Meta".to_string(),
                context_length: Some(128000),
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::CodeGeneration],
                pricing: None,
            },
        ]);

        let mut searchable_config = ModelSelectorConfig::default();
        searchable_config.group_by_provider = false;
        searchable_config.compact = true;
        searchable_config.clean_style = true;
        searchable_config.dropdown_direction = DropdownDirection::Up;
        searchable_config.dropdown_alignment = DropdownAlignment::Right;
        
        let searchable_model_selector = cx.new(|cx| {
            ModelSelector::new_with_config(cx, searchable_models, searchable_config)
        });

        // Subscribe to model selector events
        cx.subscribe(&model_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
            match event {
                ModelSelectorEvent::ModelChanged(model_id) => {
                    this.selected_model = model_id.clone();
                    cx.notify();
                }
                _ => {}
            }
        });

        Self {
            model_selector,
            compact_model_selector,
            searchable_model_selector,
            selected_model: String::new(),
            scroll_handle: ScrollHandle::new(),
            alignment: DropdownAlignment::Right,
            direction: DropdownDirection::Up,
        }
    }
}

impl Render for ModelSelectorDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                            .child("Model Selector Demo")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("AI model selection component with searchable dropdown")
                                    )
                            )
                            .child(
                                // Model selector
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
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .mb_2()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x374151))
                                                    .child("Select AI Model:")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .gap_2()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x6B7280))
                                                            .child(format!("Direction: {:?}", self.direction))
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x6B7280))
                                                            .child(format!("Alignment: {:?}", self.alignment))
                                                    )
                                            )
                                    )
                                    .child(self.model_selector.clone())
                            )
                            .child(
                                // Compact model selector without groups
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .p_4()
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
                                            .child("Compact Model Selector (No Groups):")
                                    )
                                    .child(self.compact_model_selector.clone())
                            )
                            .child(
                                // Searchable model selector with input filtering
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .mb_2()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x374151))
                                                    .child("Searchable Model Selector:")
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("Type model name to filter (e.g., 'gpt', 'claude', 'gemini')")
                                            )
                                    )
                                    .child(self.searchable_model_selector.clone())
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

            let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Model Selector Demo".into()),
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
                    app_id: Some("model_selector_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| cx.new(|cx| ModelSelectorDemo::new(window, cx)),
            );
        });
}

