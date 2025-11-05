use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::{Icon, IconName, IconSize};
use super::super::{Message, MessageRole, MessageContent, MessageId};

/// Events emitted by MessageBubble
#[derive(Clone, Debug)]
pub enum MessageBubbleEvent {
    /// Message was clicked
    Click(MessageId),
    /// Copy button was clicked
    Copy(String),
}

impl EventEmitter<MessageBubbleEvent> for MessageBubble {}

/// Visual variants for message bubbles
#[derive(Clone, Debug, PartialEq)]
pub enum MessageBubbleVariant {
    /// Standard bubble with background and padding
    Standard,
    /// Minimal appearance with less visual weight
    Minimal,
    /// Card-like appearance with shadow
    Card,
    /// Compact appearance with reduced spacing
    Compact,
}

/// Configuration for MessageBubble appearance and behavior
#[derive(Clone, Debug)]
pub struct MessageBubbleConfig {
    /// Visual variant
    pub variant: MessageBubbleVariant,
    /// Show timestamp
    pub show_timestamp: bool,
    /// Show avatar/role indicator
    pub show_avatar: bool,
    /// Show action buttons on hover
    pub show_actions: bool,
    /// Maximum width as percentage of container
    pub max_width_percent: f32,
}

impl Default for MessageBubbleConfig {
    fn default() -> Self {
        Self {
            variant: MessageBubbleVariant::Standard,
            show_timestamp: true,
            show_avatar: true,
            show_actions: true,
            max_width_percent: 75.0,
        }
    }
}

/// Message bubble component for displaying AI conversation messages
pub struct MessageBubble {
    /// The message to display
    message: Message,
    /// Configuration
    config: MessageBubbleConfig,
}

impl MessageBubble {
    /// Create a new MessageBubble
    pub fn new(message: Message) -> Self {
        Self {
            message,
            config: MessageBubbleConfig::default(),
        }
    }
    
    /// Set visual variant
    pub fn variant(mut self, variant: MessageBubbleVariant) -> Self {
        self.config.variant = variant;
        self
    }
    
    /// Show/hide timestamp
    pub fn show_timestamp(mut self, show: bool) -> Self {
        self.config.show_timestamp = show;
        self
    }
    
    /// Show/hide avatar
    pub fn show_avatar(mut self, show: bool) -> Self {
        self.config.show_avatar = show;
        self
    }
    
    /// Show/hide action buttons
    pub fn show_actions(mut self, show: bool) -> Self {
        self.config.show_actions = show;
        self
    }
    
    /// Set maximum width percentage
    pub fn max_width_percent(mut self, percent: f32) -> Self {
        self.config.max_width_percent = percent.clamp(10.0, 100.0);
        self
    }
    
    /// Enable syntax highlighting (placeholder for future implementation)
    pub fn syntax_highlighting(self, _enable: bool) -> Self {
        // TODO: Implement when syntax highlighting is available
        self
    }
    
    /// Enable minimal variant
    pub fn minimal(mut self) -> Self {
        self.config.variant = MessageBubbleVariant::Minimal;
        self.config.show_avatar = false;
        self.config.show_actions = false;
        self
    }
    
    /// Enable compact variant
    pub fn compact(mut self) -> Self {
        self.config.variant = MessageBubbleVariant::Compact;
        self.config.max_width_percent = 90.0;
        self
    }
    
    /// Enable card variant
    pub fn card(mut self) -> Self {
        self.config.variant = MessageBubbleVariant::Card;
        self
    }
}

impl IntoElement for MessageBubble {
    type Element = Div;
    
    fn into_element(self) -> Self::Element {
        let is_user = self.message.role == MessageRole::User;
        
        div()
            .flex()
            .flex_col()
            .w_full()
            .when(is_user, |this| this.items_end())
            .when(!is_user, |this| this.items_start())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .max_w(relative(self.config.max_width_percent / 100.0))
                    .when(is_user, |this| this.flex_row_reverse())
                    .when(self.config.show_avatar, |this| {
                        this.child(self.render_avatar())
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .flex_1()
                            .child(self.render_message_content())
                            .when(self.config.show_timestamp, |this| {
                                this.child(self.render_timestamp())
                            })
                    )
            )
    }
}

impl MessageBubble {
    fn render_avatar(&self) -> Div {
        let (bg_color, icon, icon_color) = match self.message.role {
            MessageRole::User => (rgb(0x696FC7), IconName::User, rgb(0xFFFFFF)),
            MessageRole::Assistant => (rgb(0x4CAF50), IconName::Settings, rgb(0xFFFFFF)), // Using Settings as placeholder for Bot
            MessageRole::System => (rgb(0xFF9800), IconName::Settings, rgb(0xFFFFFF)),
            MessageRole::Tool => (rgb(0x9C27B0), IconName::Settings, rgb(0xFFFFFF)), // Using Settings as placeholder for Tool
        };
        
        div()
            .size(px(32.))
            .flex()
            .items_center()
            .justify_center()
            .bg(bg_color)
            .rounded_full()
            .child(
                Icon::new(icon)
                    .size(IconSize::Small)
                    .color(icon_color)
            )
    }
    
    fn render_message_content(&self) -> Div {
        let container_style = self.get_container_style();
        
        container_style
            .children(
                self.message.content.iter().map(|content| {
                    self.render_content_block(content)
                })
            )
            .when(self.message.is_streaming, |this| {
                this.child(self.render_typing_indicator())
            })
    }
    
    fn get_container_style(&self) -> Div {
        let is_user = self.message.role == MessageRole::User;
        let is_system = self.message.role == MessageRole::System;
        
        let base_style = div()
            .flex()
            .flex_col()
            .gap_2();
            
        match self.config.variant {
            MessageBubbleVariant::Standard => {
                let (bg_color, text_color) = if is_user {
                    (rgb(0x696FC7), rgb(0xFFFFFF))
                } else if is_system {
                    (rgb(0xFFF4E6), rgb(0x8B6914))
                } else {
                    (rgb(0xF0F0F0), rgb(0x1A1A1A))
                };
                
                base_style
                    .bg(bg_color)
                    .text_color(text_color)
                    .px_4()
                    .py_3()
                    .rounded_xl()
            }
            MessageBubbleVariant::Minimal => {
                base_style
                    .text_color(rgb(0x1A1A1A))
                    .px_2()
                    .py_1()
            }
            MessageBubbleVariant::Card => {
                base_style
                    .bg(rgb(0xFFFFFF))
                    .text_color(rgb(0x1A1A1A))
                    .px_4()
                    .py_3()
                    .rounded_xl()
                    .border_1()
                    .border_color(rgb(0xE0E0E0))
                    .shadow_sm()
            }
            MessageBubbleVariant::Compact => {
                let bg_color = if is_user {
                    rgb(0x696FC7)
                } else if is_system {
                    rgb(0xFFF4E6)
                } else {
                    rgb(0xF5F5F5)
                };
                
                base_style
                    .bg(bg_color)
                    .text_color(if is_user { rgb(0xFFFFFF) } else { rgb(0x1A1A1A) })
                    .px_3()
                    .py_2()
                    .rounded_lg()
            }
        }
    }
    
    fn render_content_block(&self, content: &MessageContent) -> Div {
        match content {
            MessageContent::Text(text) => self.render_text(text),
            MessageContent::Code { language, code } => self.render_code_block(language, code),
            MessageContent::Error(text) => self.render_error(text),
            MessageContent::Thinking(text) => self.render_thinking(text),
            _ => self.render_text(&format!("{:?}", content)), // Fallback for other content types
        }
    }
    
    fn render_text(&self, text: &str) -> Div {
        div()
            .text_sm()
            .child(text.to_string())
    }
    
    fn render_code_block(&self, language: &str, code: &str) -> Div {
        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0x1E1E1E))
            .rounded_lg()
            .overflow_hidden()
            .child(
                // Header with language
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(rgb(0x2D2D2D))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xCCCCCC))
                            .child(language.to_string())
                    )
            )
            .child(
                // Code content
                div()
                    .px_3()
                    .py_3()
                    .text_sm()
                    .text_color(rgb(0xCCCCCC))
                    .child(code.to_string())
            )
    }
    
    fn render_thinking(&self, text: &str) -> Div {
        div()
            .flex()
            .flex_row()
            .items_start()
            .gap_2()
            .px_3()
            .py_2()
            .bg(rgb(0xF8F9FA))
            .rounded_lg()
            .border_l_4()
            .border_color(rgb(0x6C757D))
            .child(
                Icon::new(IconName::Settings) // Placeholder for Brain icon
                    .size(IconSize::Small)
                    .color(rgb(0x6C757D))
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6C757D))
                    .child(text.to_string())
            )
    }
    
    fn render_error(&self, text: &str) -> Div {
        div()
            .flex()
            .flex_row()
            .items_start()
            .gap_2()
            .px_3()
            .py_2()
            .bg(rgb(0xFFF5F5))
            .rounded_lg()
            .border_l_4()
            .border_color(rgb(0xF56565))
            .child(
                Icon::new(IconName::AlertTriangle)
                    .size(IconSize::Small)
                    .color(rgb(0xF56565))
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xC53030))
                    .child(text.to_string())
            )
    }
    
    fn render_typing_indicator(&self) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(
                div()
                    .size(px(4.))
                    .bg(rgb(0x999999))
                    .rounded_full()
            )
            .child(
                div()
                    .size(px(4.))
                    .bg(rgb(0x999999))
                    .rounded_full()
            )
            .child(
                div()
                    .size(px(4.))
                    .bg(rgb(0x999999))
                    .rounded_full()
            )
    }
    
    fn render_timestamp(&self) -> Div {
        let time_str = self.message.timestamp.format("%I:%M %p").to_string();
        
        div()
            .text_xs()
            .text_color(rgb(0x999999))
            .child(time_str)
    }
}
