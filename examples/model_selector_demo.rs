use fluix::prelude::*;
use fluix::ai::{ModelSelector, ModelSelectorEvent, ModelInfo, ModelCapability, PricingInfo, ModelSelectorConfig};
use fluix::components::form::select::{DropdownDirection, DropdownAlignment, SelectOption, SelectOptionGroup};
use fluix::components::form::combobox::{Combobox, ComboboxEvent};

struct ModelSelectorDemo {
    model_selector: Entity<ModelSelector>,
    compact_model_selector: Entity<ModelSelector>,
    searchable_model_selector: Entity<ModelSelector>,
    recently_used_model_selector: Entity<ModelSelector>,
    right_aligned_model_selector: Entity<ModelSelector>,
    grouped_combobox: Entity<Combobox>,
    fixed_width_combobox: Entity<Combobox>,
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

        // Create model selector with same style as other demos
        let mut model_config = ModelSelectorConfig::default();
        model_config.group_by_provider = false; // Same as other demos
        model_config.compact = true; // Same as other demos
        model_config.clean_style = true;
        model_config.dropdown_direction = DropdownDirection::Down; // 下拉，不是上拉
        model_config.dropdown_alignment = DropdownAlignment::Left; // 改为左对齐，确保宽度足够
        model_config.right_align_text = false; // 禁用右对齐
        
        let model_selector = cx.new(|cx| {
            ModelSelector::new_with_config(cx, models.clone(), model_config)
        });

        // Create compact model selector without groups
        let mut compact_config = ModelSelectorConfig::default();
        compact_config.group_by_provider = false;
        compact_config.compact = true;
        compact_config.clean_style = true;
        compact_config.dropdown_direction = DropdownDirection::Up;
        compact_config.dropdown_alignment = DropdownAlignment::Right;
        compact_config.right_align_text = false; // 禁用右对齐
        
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
        searchable_config.right_align_text = false; // 禁用右对齐
        // You can customize dropdown width here:
        // searchable_config.dropdown_width = DropdownWidth::MaxWidth(px(350.0));
        
        let searchable_model_selector = cx.new(|cx| {
            ModelSelector::new_with_config(cx, searchable_models.clone(), searchable_config)
        });

        // Create recently used model selector example (compact mode)
        let mut recently_used_config = ModelSelectorConfig::default();
        recently_used_config.group_by_provider = true;
        recently_used_config.compact = true;
        recently_used_config.clean_style = true;
        recently_used_config.dropdown_direction = DropdownDirection::Down;
        recently_used_config.dropdown_alignment = DropdownAlignment::Left;
        recently_used_config.right_align_text = false; // 禁用右对齐
        recently_used_config.recently_used = vec![
            "gpt-4o".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            "gemini-1.5-pro".to_string(),
        ];

        let recently_used_model_selector = cx.new(|cx| {
            ModelSelector::new_with_config(cx, searchable_models, recently_used_config)
        });

        // Create right-aligned model selector example
        let mut right_aligned_config = ModelSelectorConfig::default();
        right_aligned_config.group_by_provider = false;
        right_aligned_config.compact = true;
        right_aligned_config.clean_style = true; // 使用清洁样式（无边框）
        right_aligned_config.dropdown_direction = DropdownDirection::Down;
        right_aligned_config.dropdown_alignment = DropdownAlignment::Right;
        right_aligned_config.right_align_text = true; // 启用右对齐
        
        let right_aligned_model_selector = cx.new(|cx| {
            ModelSelector::new_with_config(cx, models.clone(), right_aligned_config)
        });

        // Create grouped combobox example
        let grouped_combobox = cx.new(|cx| {
            Combobox::new(cx)
                .placeholder("Select a programming language...")
                .option_groups(vec![
                    SelectOptionGroup::new("Frontend", vec![
                        SelectOption::new("javascript", "JavaScript"),
                        SelectOption::new("typescript", "TypeScript"),
                        SelectOption::new("react", "React"),
                        SelectOption::new("vue", "Vue"),
                    ]),
                    SelectOptionGroup::new("Backend", vec![
                        SelectOption::new("rust", "Rust"),
                        SelectOption::new("go", "Go"),
                        SelectOption::new("python", "Python"),
                        SelectOption::new("nodejs", "Node.js"),
                    ]),
                    SelectOptionGroup::new("Mobile", vec![
                        SelectOption::new("swift", "Swift"),
                        SelectOption::new("kotlin", "Kotlin"),
                        SelectOption::new("flutter", "Flutter"),
                    ]),
                ])
                .dropdown_direction(DropdownDirection::Down)
                .dropdown_alignment(DropdownAlignment::Left) // 改回左对齐
                .dropdown_max_width(px(300.0))
                .fixed_width(true) // Use fixed width with border - text and button stay in place
        });

        // Subscribe to grouped combobox events
        let _grouped_subscription = cx.subscribe(&grouped_combobox, |_this, _combobox, event: &ComboboxEvent, _cx| {
            match event {
                ComboboxEvent::Changed(value) => {
                    println!("Grouped combobox selected: {}", value);
                }
                ComboboxEvent::InputChanged(value) => {
                    println!("Grouped combobox input: {}", value);
                }
            }
        });

        // Create fixed width combobox example
        let fixed_width_combobox = cx.new(|cx| {
            Combobox::new(cx)
                .placeholder("Select...")
                .options(vec![
                    SelectOption::new("short", "Short"),
                    SelectOption::new("medium", "Medium Option"),
                    SelectOption::new("very_long", "Very Long Option Text Here"),
                ])
                .dropdown_max_width(px(350.0))
                .fixed_width(true) // Icon stays at right edge with border
        });

        // Subscribe to fixed width combobox events
        let _fixed_width_subscription = cx.subscribe(&fixed_width_combobox, |_this, _combobox, event: &ComboboxEvent, _cx| {
            match event {
                ComboboxEvent::Changed(value) => {
                    println!("Fixed width combobox selected: {}", value);
                }
                ComboboxEvent::InputChanged(value) => {
                    println!("Fixed width combobox input: {}", value);
                }
            }
        });

        // Subscribe to model selector events
        let _model_selector_subscription = cx.subscribe(&model_selector, |this, _selector, event: &ModelSelectorEvent, cx| {
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
            recently_used_model_selector,
            right_aligned_model_selector,
            grouped_combobox,
            fixed_width_combobox,
            selected_model: String::new(),
            scroll_handle: ScrollHandle::new(),
            alignment: DropdownAlignment::Right,
            direction: DropdownDirection::Up,
        }
    }
}

impl Render for ModelSelectorDemo {
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
                            .child(
                                // Recently used model selector
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
                                                    .child("Recently Used Model Selector:")
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("Shows recently used models (GPT-4o, Claude 3.5 Sonnet, Gemini 1.5 Pro) at the top")
                                            )
                                    )
                                    .child(self.recently_used_model_selector.clone())
                            )
                            .child(
                                // Right-aligned model selector
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
                                                    .child("Right-Aligned Model Selector:")
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("Text aligns right, expand button stays fixed (right_align_text: true)")
                                            )
                                    )
                                    .child(self.right_aligned_model_selector.clone())
                            .child(
                                // 添加更多空白间距方便截图
                                div().h(px(150.)) // 150px 的空白高度
                            )
                            )
                            .child(
                                // Grouped combobox example
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .min_h(px(400.)) // Add minimum height for screenshot space
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .mb_4()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x1F2937))
                                                    .child("Grouped Combobox")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("Select from grouped programming languages (Frontend, Backend, Mobile)")
                                            )
                                    )
                                    .child(self.grouped_combobox.clone())
                            )
                            .child(
                                // Fixed width combobox example
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .min_h(px(400.)) // Add minimum height for screenshot space
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .mb_4()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x1F2937))
                                                    .child("Fixed Width Combobox")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("Icon stays at right edge with border (fixed_width: true)")
                                            )
                                    )
                                    .child(self.fixed_width_combobox.clone())
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

            // 设置窗口在屏幕左边
            let bounds = Bounds::new(
                point(px(100.0), px(100.0)), // 左上角位置
                size(px(800.0), px(900.0)),   // 窗口大小
            );
            
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

