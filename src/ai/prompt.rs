use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::{TextArea, TextAreaEvent, Icon, IconName, IconSize, ComponentSize, DropdownDirection};
use super::super::{Attachment, AttachmentType, ModelInfo, ModelSelector, ModelSelectorEvent};

/// Events emitted by PromptInput
#[derive(Clone, Debug)]
pub enum PromptInputEvent {
    /// User submitted text (Enter key or send button)
    Submit(String),
    /// Text content changed
    TextChanged(String),
    /// File attachment added
    AttachFile(Attachment),
    /// Image attachment added
    AttachImage(Attachment),
    /// Model selection changed
    ModelChanged(String),
    /// Input cleared
    Clear,
    /// Focus gained
    Focus,
    /// Focus lost
    Blur,
}

impl EventEmitter<PromptInputEvent> for PromptInput {}

/// Configuration for PromptInput component
#[derive(Clone, Debug)]
pub struct PromptInputConfig {
    // Appearance
    pub placeholder: String,
    pub min_height: f32,
    pub max_height: f32,
    pub show_model_selector: bool,
    pub show_attachments: bool,
    pub show_send_button: bool,
    pub show_toolbar: bool,
    pub background_color: Option<Rgba>,
    
    // Functionality
    pub enable_multiline: bool,
    pub enable_file_upload: bool,
    pub enable_image_upload: bool,
    pub enable_code_input: bool,
    pub enable_auto_resize: bool,
    pub submit_on_enter: bool,
    
    // Style
    pub variant: PromptInputVariant,
    pub size: ComponentSize,
    pub border_radius: f32,
    
    // Models
    pub available_models: Vec<ModelInfo>,
    pub default_model: Option<String>,
}

impl Default for PromptInputConfig {
    fn default() -> Self {
        Self {
            placeholder: "Type your message...".to_string(),
            min_height: 60.0,
            max_height: 200.0,
            show_model_selector: false,
            show_attachments: true,
            show_send_button: true,
            show_toolbar: true,
            background_color: Some(rgb(0xF5F5F5)), // Default gray background
            enable_multiline: true,
            enable_file_upload: true,
            enable_image_upload: true,
            enable_code_input: false,
            enable_auto_resize: true,
            submit_on_enter: true,
            variant: PromptInputVariant::Default,
            size: ComponentSize::Medium,
            border_radius: 12.0,
            available_models: Vec::new(),
            default_model: None,
        }
    }
}

/// Visual variants for PromptInput
#[derive(Clone, Debug, PartialEq)]
pub enum PromptInputVariant {
    /// Standard appearance with border and background
    Default,
    /// Compact appearance with reduced padding
    Compact,
    /// Floating appearance with shadow
    Floating,
    /// Embedded appearance without border
    Embedded,
    /// Minimal appearance
    Minimal,
}

/// Main PromptInput component for AI interactions
pub struct PromptInput {
    // Core components
    textarea: Entity<TextArea>,
    model_selector: Option<Entity<ModelSelector>>,
    
    // Configuration
    config: PromptInputConfig,
    
    // State
    current_text: String,
    current_model: Option<String>,
    attachments: Vec<Attachment>,
    is_focused: bool,
    
    // Event subscriptions
    _subscriptions: Vec<Subscription>,
}

impl PromptInput {
    /// Create a new PromptInput with default configuration
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self::with_config(cx, PromptInputConfig::default())
    }
    
    /// Create a new PromptInput with custom configuration
    pub fn with_config(cx: &mut Context<Self>, config: PromptInputConfig) -> Self {
        // Create textarea
        let textarea = cx.new(|cx| {
            let mut textarea = TextArea::new(cx)
                .placeholder(&config.placeholder)
                .min_height(config.min_height)
                .max_height(config.max_height)
                .no_border(); // Remove border from TextArea
            
            // Set background color if provided
            if let Some(bg_color) = config.background_color {
                textarea = textarea.bg_color(bg_color);
            }
            
            textarea
        });
        
        // Create model selector if enabled
        // Note: We'll create it lazily in render if config changes via builder methods
        let model_selector = if config.show_model_selector && !config.available_models.is_empty() {
            Some(cx.new(|cx| {
                ModelSelector::new_with_models_and_direction(cx, config.available_models.clone(), DropdownDirection::Up)
                    .size(config.size)
                    .show_all_models()  // Show all models, not just popular ones
                    .clean_style(true) // Remove borders and shadows for clean look
                    // compact 模式已在 new_with_models_and_direction 中默认启用
            }))
        } else {
            None
        };
        
        // Subscribe to events
        let mut subscriptions = vec![
            cx.subscribe(&textarea, {
                move |this, _textarea, event: &TextAreaEvent, cx| {
                    this.handle_textarea_event(event, cx);
                }
            }),
        ];

        // Subscribe to model selector events if present
        if let Some(selector) = &model_selector {
            subscriptions.push(
                cx.subscribe(selector, {
                    move |this, _selector, event: &ModelSelectorEvent, cx| {
                        this.handle_model_selector_event(event, cx);
                    }
                })
            );
        }

        let _subscriptions = subscriptions;
        
        let current_model = config.default_model.clone();
        
        Self {
            textarea,
            model_selector,
            config,
            current_text: String::new(),
            current_model,
            attachments: Vec::new(),
            is_focused: false,
            _subscriptions,
        }
    }
    
    // Builder pattern methods
    
    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.config.placeholder = text.into();
        self
    }
    
    /// Set height constraints
    pub fn height(mut self, min: f32, max: f32) -> Self {
        self.config.min_height = min;
        self.config.max_height = max;
        self
    }
    
    /// Set visual variant
    pub fn variant(mut self, variant: PromptInputVariant) -> Self {
        self.config.variant = variant;
        self
    }
    
    /// Enable/disable model selector
    /// Note: This sets the configuration, but model selector Entity must be created
    /// during initialization. If you need to add models after creation, ensure
    /// show_model_selector is set to true before calling new().
    pub fn with_models(mut self, models: Vec<ModelInfo>) -> Self {
        self.config.available_models = models.clone();
        self.config.show_model_selector = !models.is_empty();
        // Note: We can't create Entity here as builder methods don't have cx
        // The model selector should be created in with_config if show_model_selector is true
        self
    }
    
    /// Set default model
    pub fn default_model(mut self, model_id: impl Into<String>) -> Self {
        self.config.default_model = Some(model_id.into());
        self
    }
    
    /// Enable compact variant
    pub fn compact(mut self) -> Self {
        self.config.variant = PromptInputVariant::Compact;
        self.config.size = ComponentSize::Small;
        self
    }
    
    /// Enable floating variant
    pub fn floating(mut self) -> Self {
        self.config.variant = PromptInputVariant::Floating;
        self
    }
    
    /// Enable/disable attachments
    pub fn enable_attachments(mut self, enable: bool) -> Self {
        self.config.show_attachments = enable;
        self.config.enable_file_upload = enable;
        self.config.enable_image_upload = enable;
        self
    }
    
    /// Enable/disable multiline input
    pub fn multiline(mut self, enable: bool) -> Self {
        self.config.enable_multiline = enable;
        self
    }
    
    /// Enable/disable send button
    pub fn show_send_button(mut self, show: bool) -> Self {
        self.config.show_send_button = show;
        self
    }
    
    /// Enable/disable toolbar
    pub fn show_toolbar(mut self, show: bool) -> Self {
        self.config.show_toolbar = show;
        self
    }
    
    /// Set background color
    pub fn background_color(mut self, color: Rgba) -> Self {
        self.config.background_color = Some(color);
        self
    }
    
    /// Set background color from RGB values
    pub fn bg_color(mut self, color: Rgba) -> Self {
        self.config.background_color = Some(color);
        self
    }
    
    // State methods
    
    /// Get current text content
    pub fn get_text(&self, cx: &Context<Self>) -> String {
        self.textarea.read(cx).get_value().to_string()
    }
    
    /// Set text content
    pub fn set_text(&mut self, text: impl Into<String>, cx: &mut Context<Self>) {
        let text = text.into();
        self.current_text = text.clone();
        self.textarea.update(cx, |textarea, cx| {
            textarea.set_value(text.clone(), cx);
        });
        cx.emit(PromptInputEvent::TextChanged(text));
    }
    
    /// Clear input
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.current_text.clear();
        self.attachments.clear();
        self.textarea.update(cx, |textarea, cx| {
            textarea.clear(cx);
        });
        cx.emit(PromptInputEvent::Clear);
        cx.notify();
    }
    
    /// Focus the input
    pub fn focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.textarea.update(cx, |textarea, _cx| {
            textarea.focus(window);
        });
    }
    
    /// Get current model
    pub fn current_model(&self) -> Option<&String> {
        self.current_model.as_ref()
    }
    
    /// Set current model
    pub fn set_model(&mut self, model_id: impl Into<String>, cx: &mut Context<Self>) {
        let model_id = model_id.into();
        self.current_model = Some(model_id.clone());
        
        // Note: Select component doesn't have set_value method yet
        // This would be implemented when the Select component is enhanced
        
        cx.emit(PromptInputEvent::ModelChanged(model_id));
    }
    
    /// Get attachments
    pub fn attachments(&self) -> &[Attachment] {
        &self.attachments
    }
    
    /// Add attachment
    pub fn add_attachment(&mut self, attachment: Attachment, cx: &mut Context<Self>) {
        match attachment.attachment_type {
            AttachmentType::Image => {
                cx.emit(PromptInputEvent::AttachImage(attachment.clone()));
            }
            _ => {
                cx.emit(PromptInputEvent::AttachFile(attachment.clone()));
            }
        }
        
        self.attachments.push(attachment);
        cx.notify();
    }
    
    /// Remove attachment
    pub fn remove_attachment(&mut self, attachment_id: &str, cx: &mut Context<Self>) {
        self.attachments.retain(|a| a.id != attachment_id);
        cx.notify();
    }
    
    // Event handlers
    
    fn handle_textarea_event(&mut self, event: &TextAreaEvent, cx: &mut Context<Self>) {
        match event {
            TextAreaEvent::Submit(content) => {
                if self.config.submit_on_enter && !content.trim().is_empty() {
                    cx.emit(PromptInputEvent::Submit(content.clone()));
                    if self.config.enable_auto_resize {
                        self.clear(cx);
                    }
                }
            }
            TextAreaEvent::Change(content) => {
                self.current_text = content.clone();
                cx.emit(PromptInputEvent::TextChanged(content.clone()));
            }
            TextAreaEvent::Focus => {
                self.is_focused = true;
                cx.emit(PromptInputEvent::Focus);
                cx.notify();
            }
            TextAreaEvent::Blur => {
                self.is_focused = false;
                cx.emit(PromptInputEvent::Blur);
                cx.notify();
            }
            _ => {}
        }
    }

    fn handle_model_selector_event(&mut self, event: &ModelSelectorEvent, cx: &mut Context<Self>) {
        match event {
            ModelSelectorEvent::ModelChanged(model_id) => {
                self.current_model = Some(model_id.clone());
                cx.emit(PromptInputEvent::ModelChanged(model_id.clone()));
            }
            _ => {}
        }
    }
    
    #[allow(dead_code)]
    fn handle_send_click(&mut self, cx: &mut Context<Self>) {
        let text = self.get_text(cx);
        if !text.trim().is_empty() {
            cx.emit(PromptInputEvent::Submit(text));
            self.clear(cx);
        }
    }
    
    #[allow(dead_code)]
    fn handle_attach_file(&mut self, cx: &mut Context<Self>) {
        // TODO: Implement file picker
        // For now, emit event for parent to handle
        cx.emit(PromptInputEvent::AttachFile(Attachment::new(
            "example.txt",
            "/path/to/file",
            1024,
            "text/plain",
        )));
    }
    
    #[allow(dead_code)]
    fn handle_attach_image(&mut self, cx: &mut Context<Self>) {
        // TODO: Implement image picker
        // For now, emit event for parent to handle
        cx.emit(PromptInputEvent::AttachImage(Attachment::new(
            "example.png",
            "/path/to/image",
            2048,
            "image/png",
        )));
    }
}

impl Render for PromptInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // If model selector should be shown but doesn't exist, create it lazily
        if self.config.show_model_selector 
            && !self.config.available_models.is_empty() 
            && self.model_selector.is_none() {
            // Create model selector lazily
            let selector = cx.new(|cx| {
                ModelSelector::new_with_models_and_direction(cx, self.config.available_models.clone(), DropdownDirection::Up)
                    .size(self.config.size)
                    .show_all_models()  // Show all models, not just popular ones
                    .clean_style(true) // Remove borders and shadows for clean look
                    // compact 模式已在 new_with_models_and_direction 中默认启用
            });
            
            // Subscribe to events
            let _sub = cx.subscribe(&selector, {
                move |this, _selector, event: &ModelSelectorEvent, cx| {
                    this.handle_model_selector_event(event, cx);
                }
            });
            
            self.model_selector = Some(selector);
        }
        
        let container_style = match self.config.variant {
            PromptInputVariant::Default => self.render_default_container(),
            PromptInputVariant::Compact => self.render_compact_container(),
            PromptInputVariant::Floating => self.render_floating_container(),
            PromptInputVariant::Embedded => self.render_embedded_container(),
            PromptInputVariant::Minimal => self.render_minimal_container(),
        };
        
        container_style
            .child(self.render_input_area(cx))
            .when(self.config.show_toolbar, |this| {
                this.child(self.render_toolbar(cx))
            })
            .when(!self.attachments.is_empty(), |this| {
                this.child(self.render_attachments())
            })
    }
}

impl PromptInput {
    fn render_default_container(&self) -> Div {
        let bg_color = self.config.background_color.unwrap_or(rgb(0xFFFFFF));
        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(bg_color)
            .border_1()
            .border_color(if self.is_focused { rgb(0x696FC7) } else { rgb(0xE0E0E0) })
            .rounded(px(self.config.border_radius))
            .p_3()
            .shadow_sm()
    }
    
    fn render_compact_container(&self) -> Div {
        let bg_color = self.config.background_color.unwrap_or(rgb(0xF8F8F8));
        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(bg_color)
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_lg()
            .p_2()
    }
    
    fn render_floating_container(&self) -> Div {
        let bg_color = self.config.background_color.unwrap_or(rgb(0xFFFFFF));
        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(bg_color)
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_xl()
            .p_4()
            .shadow_lg()
    }
    
    fn render_embedded_container(&self) -> Div {
        let bg_color = self.config.background_color.unwrap_or(rgb(0xF8F8F8));
        // Border color darker than gray background
        // If background is gray (around 0xF5F5F5), use darker gray border
        let border_color = if bg_color.r < 0.95 && bg_color.g < 0.95 && bg_color.b < 0.95 {
            rgb(0xD0D0D0) // Darker gray for gray backgrounds
        } else {
            rgb(0xE0E0E0) // Standard light gray for white backgrounds
        };
        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded_lg()
            .p_2()
            // Don't set overflow_hidden to allow dropdown to overflow
    }
    
    fn render_minimal_container(&self) -> Div {
        let bg_color = self.config.background_color.unwrap_or(rgb(0xFFFFFF));
        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(bg_color)
            .p_1()
    }
    
    fn render_input_area(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .child(self.textarea.clone())
    }
    
    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_between()
            .pt_2()
            // Don't set overflow_hidden to allow dropdown to overflow
            .child(self.render_left_actions(cx))
            .child(self.render_right_actions(cx))
    }
    
    fn render_left_actions(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_1()
            .when(self.config.enable_file_upload, |this| {
                this.child(self.render_simple_button(IconName::Attachment))
            })
            .when(self.config.enable_image_upload, |this| {
                this.child(self.render_simple_button(IconName::Image))
            })
            // Settings button removed - not needed for prompt input
            // .when(self.config.enable_code_input, |this| {
            //     this.child(self.render_simple_button(IconName::Settings))
            // })
    }
    
    fn render_right_actions(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_2()
            .items_center()
            // Don't set overflow_hidden to allow dropdown to overflow
            .when(self.config.show_model_selector && !self.config.available_models.is_empty(), |this| {
                // Show model selector if available (should exist after lazy creation in render)
                if let Some(selector) = &self.model_selector {
                    this.child(
                        div()
                            .relative() // Ensure proper positioning context for dropdown
                            // Don't set overflow_hidden to allow dropdown to overflow
                            .child(selector.clone())
                    )
                } else {
                    this
                }
            })
            .when(self.config.show_send_button, |this| {
                this.child(self.render_send_button(cx))
            })
    }
    
    fn render_simple_button(&self, icon: IconName) -> impl IntoElement {
        div()
            .p_1()
            .rounded_md()
            .cursor_pointer()
            .hover(|this| this.bg(rgb(0xF0F0F0)))
            .child(
                Icon::new(icon)
                    .size(IconSize::Small)
                    .color(rgb(0x666666))
            )
    }
    
    fn render_send_button(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.current_text.trim().is_empty();

        div()
            .size(px(32.))
            .flex()
            .items_center()
            .justify_center()
            .bg(if is_empty { rgb(0xCCCCCC) } else { rgb(0x696FC7) })
            .rounded_lg()
            .cursor_pointer()
            .when(!is_empty, |this| {
                this.hover(|this| this.bg(rgb(0xA7AAE1)))
            })
            .child(
                Icon::new(IconName::Send)
                    .size(IconSize::Small)
                    .color(rgb(0xFFFFFF))
            )
    }
    
    fn render_attachments(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_2()
            .pt_2()
            .children(
                self.attachments.iter().map(|attachment| {
                    self.render_attachment_chip(attachment)
                })
            )
    }
    
    fn render_attachment_chip(&self, attachment: &Attachment) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .bg(rgb(0xF0F0F0))
            .rounded_md()
            .child(
                Icon::new(match attachment.attachment_type {
                    AttachmentType::Image => IconName::Image,
                    AttachmentType::Document => IconName::Attachment,
                    AttachmentType::Code => IconName::Settings,
                    _ => IconName::Attachment,
                })
                .size(IconSize::XSmall)
                .color(rgb(0x666666))
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x666666))
                    .child(attachment.name.clone())
            )
            .child(
                div()
                    .p_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .hover(|this| this.bg(rgb(0xE0E0E0)))
                    .child(
                        Icon::new(IconName::Close)
                            .size(IconSize::XSmall)
                            .color(rgb(0x999999))
                    )
            )
    }
}
