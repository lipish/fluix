//! # Selection Components
//!
//! Components for selecting AI models, providers, and configuration options.

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::{Combobox, ComboboxEvent, SelectOption, SelectOptionGroup, Icon, IconName, IconSize, ComponentSize, DropdownDirection, DropdownWidth, DropdownAlignment};
use super::{ModelInfo, ModelCapability, ProviderInfo};

/// Events emitted by ModelSelector
#[derive(Clone, Debug)]
pub enum ModelSelectorEvent {
    /// Model selection changed
    ModelChanged(String),
    /// Provider filter changed
    ProviderChanged(String),
    /// Capability filter changed
    CapabilityChanged(ModelCapability),
    /// Show pricing toggled
    ShowPricingToggled(bool),
}

impl EventEmitter<ModelSelectorEvent> for ModelSelector {}

/// Configuration for ModelSelector
#[derive(Clone, Debug)]
pub struct ModelSelectorConfig {
    /// Group models by provider
    pub group_by_provider: bool,
    /// Show pricing information
    pub show_pricing: bool,
    /// Show model descriptions
    pub show_descriptions: bool,
    /// Show capability badges
    pub show_capabilities: bool,
    /// Filter by specific capability
    pub filter_capability: Option<ModelCapability>,
    /// Filter by specific provider
    pub filter_provider: Option<String>,
    /// Component size
    pub size: ComponentSize,
    /// Compact display mode
    pub compact: bool,
    /// Maximum width
    pub max_width: Option<f32>,
    /// Dropdown direction
    pub dropdown_direction: DropdownDirection,
    /// Dropdown width control
    pub dropdown_width: DropdownWidth,
    /// Dropdown alignment
    pub dropdown_alignment: DropdownAlignment,
    /// Show only popular models (limit to 5)
    pub show_only_popular: bool,
    /// Recently used models (shown at top)
    pub recently_used: Vec<String>,
    /// Remove borders and shadows for clean look
    pub clean_style: bool,
}

impl Default for ModelSelectorConfig {
    fn default() -> Self {
        Self {
            group_by_provider: true,
            show_pricing: false,
            show_descriptions: false, // 默认不显示描述，只显示模型名称
            show_capabilities: true,
            filter_capability: None,
            filter_provider: None,
            size: ComponentSize::Medium,
            compact: false,
            max_width: None,
            dropdown_direction: DropdownDirection::Auto,
            dropdown_width: DropdownWidth::MaxWidth(px(400.0)), // 设置合理的最大宽度
            dropdown_alignment: DropdownAlignment::Right, // 默认右对齐
            show_only_popular: false, // 默认显示所有模型（changed from true）
            recently_used: Vec::new(),
            clean_style: true, // 默认使用清洁样式（无边框无阴影）
        }
    }
}

/// Model group for organizing models by provider
#[derive(Clone, Debug)]
pub struct ModelGroup {
    pub provider: String,
    pub models: Vec<ModelInfo>,
}

/// Popular AI models (top 5 most commonly used)
const POPULAR_MODELS: &[&str] = &[
    "gpt-4o",
    "gpt-4o-mini",
    "claude-3-5-sonnet-20241022",
    "gemini-1.5-pro",
    "llama-3.1-405b-instruct",
];

/// Enhanced model selector component for AI applications
pub struct ModelSelector {
    /// Available models
    models: Vec<ModelInfo>,
    /// Grouped models by provider
    model_groups: Vec<ModelGroup>,
    /// Currently selected model
    selected_model: Option<String>,
    /// Configuration
    config: ModelSelectorConfig,
    /// Internal combobox component
    combobox: Entity<Combobox>,
    /// Event subscriptions
    _subscriptions: Vec<Subscription>,
}

impl ModelSelector {
    /// Create a new ModelSelector with default models from llm-link
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Use default models from llm-link
        let default_models = ModelInfo::default_models_from_llm_link();
        Self::new_with_models(cx, default_models)
    }

    /// Create a new ModelSelector with models
    pub fn new_with_models(cx: &mut Context<Self>, models: Vec<ModelInfo>) -> Self {
        Self::new_with_models_and_config(cx, models, ModelSelectorConfig::default())
    }

    /// Create a new ModelSelector with models and specific dropdown direction
    pub fn new_with_models_and_direction(cx: &mut Context<Self>, models: Vec<ModelInfo>, direction: DropdownDirection) -> Self {
        let mut config = ModelSelectorConfig::default();
        config.dropdown_direction = direction;
        config.compact = true; // 默认启用紧凑模式，因为模型较多
        Self::new_with_models_and_config(cx, models, config)
    }

    /// Create a new ModelSelector with custom config
    pub fn new_with_config(cx: &mut Context<Self>, models: Vec<ModelInfo>, config: ModelSelectorConfig) -> Self {
        Self::new_with_models_and_config(cx, models, config)
    }

    /// Create a new ModelSelector with models and config
    fn new_with_models_and_config(cx: &mut Context<Self>, models: Vec<ModelInfo>, config: ModelSelectorConfig) -> Self {

        // Create model groups
        let mut model_groups = Vec::new();
        if config.group_by_provider {
            let mut groups: std::collections::HashMap<String, Vec<ModelInfo>> =
                std::collections::HashMap::new();
            // Track seen model IDs to prevent duplicates
            let mut seen_model_ids = std::collections::HashSet::new();

            for model in &models {
                // Skip if we've already seen this model ID
                if seen_model_ids.contains(&model.id) {
                    continue;
                }
                seen_model_ids.insert(model.id.clone());
                
                groups
                    .entry(model.provider.clone())
                    .or_insert_with(Vec::new)
                    .push(model.clone());
            }

            model_groups = groups
                .into_iter()
                .map(|(provider, models)| ModelGroup { provider, models })
                .collect();

            // Sort groups by provider name
            model_groups.sort_by(|a, b| a.provider.cmp(&b.provider));
        } else {
            if !models.is_empty() {
                // Deduplicate models by ID before creating single group
                let mut seen_model_ids = std::collections::HashSet::new();
                let unique_models: Vec<ModelInfo> = models
                    .iter()
                    .filter(|model| {
                        if seen_model_ids.contains(&model.id) {
                            false
                        } else {
                            seen_model_ids.insert(model.id.clone());
                            true
                        }
                    })
                    .cloned()
                    .collect();

                if !unique_models.is_empty() {
                    model_groups = vec![ModelGroup {
                        provider: "All Models".to_string(),
                        models: unique_models,
                    }];
                }
            }
        }

        // Create combobox with proper grouping
        let combobox = cx.new(|cx| {
            let mut combobox_builder = Combobox::new(cx)
                .size(ComponentSize::Medium)
                .placeholder("Select a model...")
                .dropdown_direction(config.dropdown_direction)
                .dropdown_width(config.dropdown_width)
                .dropdown_alignment(config.dropdown_alignment);

            // Enable compact mode for tighter spacing when there are many models
            if config.compact {
                combobox_builder = combobox_builder.compact();
            }

            // Apply clean styling (no borders, no shadows, transparent background)
            if config.clean_style {
                combobox_builder = combobox_builder.no_border().no_shadow().transparent_background();
            }

            if config.group_by_provider && model_groups.len() > 1 {
                // Use option groups for proper grouping
                let groups = Self::create_option_groups(&model_groups, &config);
                combobox_builder.option_groups(groups)
            } else {
                // Use flat options
                let options = Self::create_flat_options(&model_groups, &config);
                combobox_builder.options(options)
            }
        });

        let _subscriptions = vec![
            cx.subscribe(&combobox, |this, _combobox, event: &ComboboxEvent, cx| {
                this.handle_combobox_event(event, cx);
            }),
        ];

        Self {
            models,
            model_groups,
            selected_model: None,
            config,
            combobox,
            _subscriptions,
        }
    }

    /// Create with models (builder pattern)
    pub fn with_models(mut self, models: Vec<ModelInfo>) -> Self {
        // Note: This is a limitation of the current design.
        // In a real implementation, we would need to recreate the component
        // or use a different approach. For now, this is a placeholder.
        self.models = models;
        self.update_model_groups();
        self
    }

    /// Create with providers
    pub fn with_providers(mut self, providers: Vec<ProviderInfo>) -> Self {
        self.models = providers
            .into_iter()
            .flat_map(|provider| provider.models)
            .collect();
        self.update_model_groups();
        self
    }

    // Builder pattern methods

    /// Group models by provider
    pub fn group_by_provider(mut self, group: bool) -> Self {
        self.config.group_by_provider = group;
        self.update_model_groups();
        self
    }

    /// Show pricing information
    pub fn show_pricing(mut self, show: bool) -> Self {
        self.config.show_pricing = show;
        self
    }

    /// Show model descriptions
    pub fn show_descriptions(mut self, show: bool) -> Self {
        self.config.show_descriptions = show;
        self
    }

    /// Show capability badges
    pub fn show_capabilities(mut self, show: bool) -> Self {
        self.config.show_capabilities = show;
        self
    }

    /// Filter by capability
    pub fn filter_by_capability(mut self, capability: ModelCapability) -> Self {
        self.config.filter_capability = Some(capability);
        self.update_model_groups();
        self
    }

    /// Filter by provider
    pub fn filter_by_provider(mut self, provider: impl Into<String>) -> Self {
        self.config.filter_provider = Some(provider.into());
        self.update_model_groups();
        self
    }

    /// Set component size
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.config.size = size;
        self
    }

    /// Enable compact mode
    pub fn compact(mut self) -> Self {
        self.config.compact = true;
        self.config.size = ComponentSize::Small;
        self.config.show_descriptions = false;
        self.config.show_capabilities = false;
        // Note: compact mode will be applied to the Select component
        // when it's created, allowing tighter spacing in the dropdown
        self
    }

    /// Show only popular models (default: true)
    pub fn show_only_popular(mut self, show: bool) -> Self {
        self.config.show_only_popular = show;
        self.update_model_groups();
        self
    }

    /// Show all available models (disable popular filter)
    pub fn show_all_models(mut self) -> Self {
        self.config.show_only_popular = false;
        self.update_model_groups();
        self
    }

    /// Set recently used models
    pub fn with_recently_used(mut self, recently_used: Vec<String>) -> Self {
        self.config.recently_used = recently_used;
        self
    }

    /// Enable clean style (no borders, no shadows)
    pub fn clean_style(mut self, clean: bool) -> Self {
        self.config.clean_style = clean;
        self
    }

    /// Use default style (with borders and shadows)
    pub fn default_style(mut self) -> Self {
        self.config.clean_style = false;
        self
    }

    /// Set maximum width
    pub fn max_width(mut self, width: f32) -> Self {
        self.config.max_width = Some(width);
        self
    }

    /// Set dropdown direction
    pub fn dropdown_direction(mut self, direction: DropdownDirection) -> Self {
        self.config.dropdown_direction = direction;
        // Note: In a real implementation, we would update the select component here
        // For now, this will be applied when the component is recreated
        self
    }

    /// Set dropdown to expand upward
    pub fn dropdown_up(mut self) -> Self {
        self.config.dropdown_direction = DropdownDirection::Up;
        self
    }

    /// Set dropdown to expand downward
    pub fn dropdown_down(mut self) -> Self {
        self.config.dropdown_direction = DropdownDirection::Down;
        self
    }

    /// Set dropdown to auto-detect direction
    pub fn dropdown_auto(mut self) -> Self {
        self.config.dropdown_direction = DropdownDirection::Auto;
        self
    }

    /// Set dropdown width
    pub fn dropdown_width(mut self, width: DropdownWidth) -> Self {
        self.config.dropdown_width = width;
        self
    }

    /// Set dropdown alignment
    pub fn dropdown_alignment(mut self, alignment: DropdownAlignment) -> Self {
        self.config.dropdown_alignment = alignment;
        self
    }

    /// Align dropdown to right (convenience method)
    pub fn dropdown_right(mut self) -> Self {
        self.config.dropdown_alignment = DropdownAlignment::Right;
        self
    }

    /// Align dropdown to left (convenience method)
    pub fn dropdown_left(mut self) -> Self {
        self.config.dropdown_alignment = DropdownAlignment::Left;
        self
    }

    /// Set maximum dropdown width (convenience method)
    pub fn dropdown_max_width(mut self, width: f32) -> Self {
        self.config.dropdown_width = DropdownWidth::MaxWidth(px(width));
        self
    }

    /// Set fixed dropdown width (convenience method)
    pub fn dropdown_fixed_width(mut self, width: f32) -> Self {
        self.config.dropdown_width = DropdownWidth::Fixed(px(width));
        self
    }

    /// Set minimum dropdown width (convenience method)
    pub fn dropdown_min_width(mut self, width: f32) -> Self {
        self.config.dropdown_width = DropdownWidth::MinWidth(px(width));
        self
    }

    // State methods

    /// Get currently selected model
    pub fn selected_model(&self) -> Option<&String> {
        self.selected_model.as_ref()
    }

    /// Set selected model
    pub fn set_selected_model(&mut self, model_id: impl Into<String>, cx: &mut Context<Self>) {
        let model_id = model_id.into();
        self.selected_model = Some(model_id.clone());

        // Add to recently used models
        self.add_to_recently_used(model_id.clone());

        // Update internal combobox component
        self.combobox.update(cx, |combobox, _cx| {
            combobox.set_value(model_id.clone());
        });

        cx.emit(ModelSelectorEvent::ModelChanged(model_id));
        cx.notify();
    }

    /// Add model to recently used list
    fn add_to_recently_used(&mut self, model_id: String) {
        // Remove if already exists
        self.config.recently_used.retain(|id| id != &model_id);

        // Add to front
        self.config.recently_used.insert(0, model_id);

        // Keep only last 3 recently used
        self.config.recently_used.truncate(3);
    }

    /// Set recently used models
    pub fn set_recently_used(&mut self, recently_used: Vec<String>) {
        self.config.recently_used = recently_used;
    }

    /// Get recently used models
    pub fn get_recently_used(&self) -> &[String] {
        &self.config.recently_used
    }

    /// Get available models
    pub fn models(&self) -> &[ModelInfo] {
        &self.models
    }

    /// Get model by ID
    pub fn get_model(&self, model_id: &str) -> Option<&ModelInfo> {
        self.models.iter().find(|model| model.id == model_id)
    }

    /// Get filtered models
    pub fn filtered_models(&self) -> Vec<&ModelInfo> {
        self.models
            .iter()
            .filter(|model| self.matches_filters(model))
            .collect()
    }

    // Internal methods

    fn update_model_groups(&mut self) {
        if self.config.group_by_provider {
            let mut groups: std::collections::HashMap<String, Vec<ModelInfo>> = 
                std::collections::HashMap::new();
            // Track seen model IDs to prevent duplicates
            let mut seen_model_ids = std::collections::HashSet::new();

            for model in &self.models {
                if self.matches_filters(model) {
                    // Skip if we've already seen this model ID
                    if seen_model_ids.contains(&model.id) {
                        continue;
                    }
                    seen_model_ids.insert(model.id.clone());
                    
                    groups
                        .entry(model.provider.clone())
                        .or_insert_with(Vec::new)
                        .push(model.clone());
                }
            }

            self.model_groups = groups
                .into_iter()
                .map(|(provider, models)| ModelGroup { provider, models })
                .collect();

            // Sort groups by provider name
            self.model_groups.sort_by(|a, b| a.provider.cmp(&b.provider));
        } else {
            // Single group with all models
            // Deduplicate models by ID
            let mut seen_model_ids = std::collections::HashSet::new();
            let filtered_models: Vec<ModelInfo> = self.models
                .iter()
                .filter(|model| {
                    if !self.matches_filters(model) {
                        return false;
                    }
                    if seen_model_ids.contains(&model.id) {
                        return false;
                    }
                    seen_model_ids.insert(model.id.clone());
                    true
                })
                .cloned()
                .collect();

            if !filtered_models.is_empty() {
                self.model_groups = vec![ModelGroup {
                    provider: "All Models".to_string(),
                    models: filtered_models,
                }];
            } else {
                self.model_groups.clear();
            }
        }

        // Update select options
        self.update_select_options();
    }

    fn matches_filters(&self, model: &ModelInfo) -> bool {
        // Filter by capability
        if let Some(capability) = &self.config.filter_capability {
            if !model.capabilities.contains(capability) {
                return false;
            }
        }

        // Filter by provider
        if let Some(provider) = &self.config.filter_provider {
            if model.provider != *provider {
                return false;
            }
        }

        true
    }

    fn create_option_groups(model_groups: &[ModelGroup], config: &ModelSelectorConfig) -> Vec<SelectOptionGroup> {
        let filtered_groups = if config.show_only_popular {
            Self::filter_popular_models(model_groups, config)
        } else {
            model_groups.to_vec()
        };

        let groups: Vec<SelectOptionGroup> = filtered_groups
            .iter()
            .map(|group| {
                // Deduplicate models within each group by model ID
                let mut seen_ids = std::collections::HashSet::new();
                let unique_models: Vec<&ModelInfo> = group.models
                    .iter()
                    .filter(|model| {
                        if seen_ids.contains(&model.id) {
                            false // Skip duplicate
                        } else {
                            seen_ids.insert(model.id.clone());
                            true // Keep first occurrence
                        }
                    })
                    .collect();

                let options = unique_models
                    .iter()
                    .map(|model| {
                        let label = Self::format_model_label_static(model, config);
                        SelectOption::new(&model.id, &label)
                    })
                    .collect();

                SelectOptionGroup::new(&group.provider, options)
            })
            .collect();

        groups
    }

    fn create_flat_options(model_groups: &[ModelGroup], config: &ModelSelectorConfig) -> Vec<SelectOption> {
        let filtered_groups = if config.show_only_popular {
            Self::filter_popular_models(model_groups, config)
        } else {
            model_groups.to_vec()
        };

        // Collect all models and deduplicate by model ID
        let mut all_models: Vec<&ModelInfo> = filtered_groups
            .iter()
            .flat_map(|group| &group.models)
            .collect();

        // Remove duplicates by model ID (keep first occurrence)
        let mut seen_ids = std::collections::HashSet::new();
        all_models.retain(|model| {
            if seen_ids.contains(&model.id) {
                false // Skip duplicate
            } else {
                seen_ids.insert(model.id.clone());
                true // Keep first occurrence
            }
        });

        // Sort models: recently used first, then popular, then alphabetical
        all_models.sort_by(|a, b| {
            let a_recently_used = config.recently_used.contains(&a.id);
            let b_recently_used = config.recently_used.contains(&b.id);

            match (a_recently_used, b_recently_used) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                (true, true) => {
                    // Both recently used, sort by position in recently_used list
                    let a_pos = config.recently_used.iter().position(|id| id == &a.id).unwrap_or(usize::MAX);
                    let b_pos = config.recently_used.iter().position(|id| id == &b.id).unwrap_or(usize::MAX);
                    a_pos.cmp(&b_pos)
                }
                (false, false) => {
                    // Neither recently used, sort by popularity then name
                    let a_popular = POPULAR_MODELS.contains(&a.id.as_str());
                    let b_popular = POPULAR_MODELS.contains(&b.id.as_str());

                    match (a_popular, b_popular) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.cmp(&b.name),
                    }
                }
            }
        });

        all_models
            .iter()
            .map(|model| {
                let label = Self::format_model_label_static(model, config);
                SelectOption::new(&model.id, &label)
            })
            .collect()
    }

    fn format_model_label_static(model: &ModelInfo, _config: &ModelSelectorConfig) -> String {
        // 只显示模型名称，不显示描述
        model.name.clone()
    }

    /// Filter models to show only popular ones and recently used
    fn filter_popular_models(model_groups: &[ModelGroup], config: &ModelSelectorConfig) -> Vec<ModelGroup> {
        let mut filtered_groups = Vec::new();

        for group in model_groups {
            let mut filtered_models = Vec::new();

            // Add recently used models first
            for recent_id in &config.recently_used {
                if let Some(model) = group.models.iter().find(|m| &m.id == recent_id) {
                    filtered_models.push(model.clone());
                }
            }

            // Add popular models that aren't already in recently used
            for model in &group.models {
                if POPULAR_MODELS.contains(&model.id.as_str()) &&
                   !config.recently_used.contains(&model.id) &&
                   !filtered_models.iter().any(|m| m.id == model.id) {
                    filtered_models.push(model.clone());
                }
            }

            // Limit to 5 total models per group
            filtered_models.truncate(5);

            if !filtered_models.is_empty() {
                filtered_groups.push(ModelGroup {
                    provider: group.provider.clone(),
                    models: filtered_models,
                });
            }
        }

        filtered_groups
    }

    #[allow(dead_code)]
    fn create_select_options(&self) -> Vec<SelectOption> {
        Self::create_flat_options(&self.model_groups, &self.config)
    }

    fn update_select_options(&mut self) {
        // Note: Since Combobox doesn't have public methods to update options after creation,
        // we would need to recreate the combobox. However, this is complex because we need
        // a Context to create new entities. For now, this is a limitation.
        //
        // The proper solution would be to add public methods to Combobox like:
        // - set_options(&mut self, options: Vec<SelectOption>)
        // - set_option_groups(&mut self, groups: Vec<SelectOptionGroup>)
        //
        // For now, the options are set correctly during initial creation when using
        // the builder pattern methods.
    }

    #[allow(dead_code)]
    fn format_model_label(&self, model: &ModelInfo) -> String {
        if self.config.compact {
            model.name.clone()
        } else {
            let mut label = model.name.clone();
            
            if self.config.show_descriptions {
                if let Some(description) = &model.description {
                    label.push_str(&format!(" - {}", description));
                }
            }
            
            if self.config.show_pricing {
                if let Some(pricing) = &model.pricing {
                    label.push_str(&format!(
                        " (${:.3}/1K tokens)",
                        pricing.input_price
                    ));
                }
            }
            
            label
        }
    }

    fn handle_combobox_event(&mut self, event: &ComboboxEvent, cx: &mut Context<Self>) {
        match event {
            ComboboxEvent::Changed(value) => {
                if value.is_empty() {
                    // 空值表示清除选择
                    self.selected_model = None;
                    cx.emit(ModelSelectorEvent::ModelChanged("".to_string()));
                } else if !value.starts_with("---") {
                    // 非空值且不是分组标题
                    self.selected_model = Some(value.clone());
                    cx.emit(ModelSelectorEvent::ModelChanged(value.clone()));
                }
            }
            ComboboxEvent::InputChanged(_value) => {
                // 用户输入变化时，搜索功能会自动过滤选项
                // 这里可以添加额外的处理逻辑如果需要
            }
        }
    }
}

impl Render for ModelSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let container = div()
            .flex()
            .flex_col()
            .gap_2();

        let container = if let Some(max_width) = self.config.max_width {
            container.max_w(px(max_width))
        } else {
            container.w_full()
        };

        container
            .when(!self.config.compact, |this| {
                this.child(self.render_header())
            })
            .child(self.render_selector(cx))
            .when(self.selected_model.is_some() && !self.config.compact, |this| {
                this.child(self.render_model_details())
            })
    }
}

impl ModelSelector {
    fn render_header(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x374151))
                    .child("AI Model")
            )
            .when(self.config.show_pricing, |this| {
                this.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x6B7280))
                        .child(format!("{} models available", self.filtered_models().len()))
                )
            })
    }

    fn render_selector(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex_1()
                    .min_w(px(200.)) // 设置最小宽度，避免输入时宽度变化
                    .max_w(px(400.)) // 设置最大宽度
                    .child(self.combobox.clone())
            )
    }

    fn render_refresh_button(&self) -> impl IntoElement {
        div()
            .p_1()
            .rounded_md()
            .cursor_pointer()
            .hover(|this| this.bg(rgb(0xF3F4F6)))
            .child(
                Icon::new(IconName::Settings) // Using Settings as placeholder for refresh
                    .size(IconSize::Small)
                    .color(rgb(0x6B7280))
            )
    }

    fn render_capability_filter(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_1()
            .flex_wrap()
            .children(
                [
                    ModelCapability::TextGeneration,
                    ModelCapability::CodeGeneration,
                    ModelCapability::ImageGeneration,
                    ModelCapability::ImageAnalysis,
                    ModelCapability::FunctionCalling,
                ]
                .iter()
                .map(|capability| self.render_capability_badge(capability))
            )
    }

    fn render_capability_badge(&self, capability: &ModelCapability) -> impl IntoElement {
        let is_active = self.config.filter_capability.as_ref() == Some(capability);
        let label = match capability {
            ModelCapability::TextGeneration => "Text",
            ModelCapability::CodeGeneration => "Code",
            ModelCapability::ImageGeneration => "Image Gen",
            ModelCapability::ImageAnalysis => "Image Analysis",
            ModelCapability::FunctionCalling => "Functions",
            ModelCapability::DocumentAnalysis => "Documents",
            ModelCapability::WebSearch => "Web Search",
        };

        div()
            .px_2()
            .py_1()
            .text_xs()
            .rounded_md()
            .cursor_pointer()
            .when(is_active, |this| {
                this.bg(rgb(0x3B82F6))
                    .text_color(rgb(0xFFFFFF))
            })
            .when(!is_active, |this| {
                this.bg(rgb(0xF3F4F6))
                    .text_color(rgb(0x6B7280))
                    .hover(|this| this.bg(rgb(0xE5E7EB)))
            })
            .child(label)
    }

    fn render_model_details(&self) -> impl IntoElement {
        if let Some(model_id) = &self.selected_model {
            if let Some(model) = self.get_model(model_id) {
                let mut details = div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_3();

                // Apply styling based on clean_style setting
                if self.config.clean_style {
                    details = details.bg(rgba(0x00000000)); // Transparent background
                } else {
                    details = details
                        .bg(rgb(0xF9FAFB))
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(0xE5E7EB));
                }

                return details
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x111827))
                                    .child(model.name.clone())
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x6B7280))
                                    .child(model.provider.clone())
                            )
                    )
                    .when(model.description.is_some(), |this| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x6B7280))
                                .child(model.description.as_ref().unwrap().clone())
                        )
                    })
                    .when(model.context_length.is_some(), |this| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x6B7280))
                                .child(format!(
                                    "Context: {} tokens",
                                    model.context_length.unwrap()
                                ))
                        )
                    })
                    .when(self.config.show_pricing && model.pricing.is_some(), |this| {
                        let pricing = model.pricing.as_ref().unwrap();
                        this.child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_4()
                                .text_xs()
                                .text_color(rgb(0x6B7280))
                                .child(format!("Input: ${:.3}/1K", pricing.input_price))
                                .child(format!("Output: ${:.3}/1K", pricing.output_price))
                        )
                    })
                    .when(!model.capabilities.is_empty(), |this| {
                        this.child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_1()
                                .flex_wrap()
                                .children(
                                    model.capabilities.iter().map(|cap| {
                                        div()
                                            .px_2()
                                            .py_1()
                                            .text_xs()
                                            .bg(rgb(0xDCFCE7))
                                            .text_color(rgb(0x166534))
                                            .rounded_md()
                                            .child(format!("{:?}", cap))
                                    })
                                )
                        )
                    });
            }
        }

        div() // Empty div if no model selected
    }

    /// Clear the current selection
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selected_model = None;

        // Also clear the underlying Combobox component
        self.combobox.update(cx, |combobox, _cx| {
            // Clear by setting empty values
            combobox.set_value("");
            combobox.set_input_value("");
        });

        cx.emit(ModelSelectorEvent::ModelChanged("".to_string()));
        cx.notify();
    }

    /// Get the currently selected model ID
    pub fn get_selected_model(&self) -> Option<&String> {
        self.selected_model.as_ref()
    }
}
