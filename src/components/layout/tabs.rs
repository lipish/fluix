use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the Tabs component
#[derive(Clone, Debug)]
pub enum TabsEvent {
    /// Tab was changed
    TabChanged { index: usize },
}

impl EventEmitter<TabsEvent> for Tabs {}

// ============================================================================
// Types
// ============================================================================

/// A single tab item
pub struct TabItem {
    /// Label text for the tab
    pub label: String,
    /// Content element (can be any IntoElement)
    pub content: Box<dyn Fn() -> AnyElement>,
}

impl TabItem {
    /// Create a new tab item with a label and content builder
    pub fn new<F, E>(label: impl Into<String>, content: F) -> Self
    where
        F: Fn() -> E + 'static,
        E: IntoElement + 'static,
    {
        Self {
            label: label.into(),
            content: Box::new(move || content().into_any_element()),
        }
    }
}

// ============================================================================
// Component
// ============================================================================

/// A tabs component that displays multiple tab items with content panels
/// 
/// # Example
/// 
/// ```rust,ignore
/// let tabs = cx.new(|cx| {
///     Tabs::new()
///         .tabs(vec![
///             TabItem::new("Tab 1", || div().child("Tab 1 content")),
///             TabItem::new("Tab 2", || div().child("Tab 2 content")),
///             TabItem::new("Tab 3", || div().child("Tab 3 content")),
///         ])
/// });
/// ```
pub struct Tabs {
    /// List of tab items
    tabs: Vec<TabItem>,
    /// Currently active tab index
    active_index: usize,
    /// Size of the tabs
    size: ComponentSize,
}

impl Tabs {
    /// Create a new Tabs component
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_index: 0,
            size: ComponentSize::Medium,
        }
    }
    
    /// Set the tabs
    pub fn tabs(mut self, tabs: Vec<TabItem>) -> Self {
        self.tabs = tabs;
        // Ensure active_index is within bounds
        if self.active_index >= self.tabs.len() {
            self.active_index = 0;
        }
        self
    }
    
    /// Set the active tab index
    pub fn active_index(mut self, index: usize) -> Self {
        if index < self.tabs.len() {
            self.active_index = index;
        }
        self
    }
    
    /// Set the size of the tabs
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
    
    /// Get the padding for tab items
    fn tab_padding(&self) -> (Pixels, Pixels) {
        let (py, px) = self.size.padding();
        (py, px)
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for Tabs {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        // Clone only the labels, we'll use the original tabs for content
        let tab_labels: Vec<String> = self.tabs.iter().map(|t| t.label.clone()).collect();
        let tabs_ref = &self.tabs;
        let active_index = self.active_index;
        let size = self.size;
        let (tab_py, tab_px) = self.tab_padding();
        
        // Tab bar background color (light gray)
        let tab_bar_bg = rgb(0xF5F5F5); // Light gray background
        let tab_bar_radius = px(BorderRadius::LG);
        
        // Shadow for tab bar
        let tab_bar_shadow = BoxShadow {
            color: rgba(0x0000000A).into(), // Subtle shadow
            offset: point(px(0.), px(1.)),
            blur_radius: px(2.),
            spread_radius: px(0.),
        };
        
        // Helper function to create active tab shadow
        let make_active_tab_shadow = || BoxShadow {
            color: rgba(0x0000000A).into(),
            offset: point(px(0.), px(1.)),
            blur_radius: px(2.),
            spread_radius: px(0.),
        };
        
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap_4()
            .child(
                // Tab bar container
                div()
                    .flex()
                    .flex_row()
                    .bg(tab_bar_bg)
                    .rounded(tab_bar_radius)
                    .shadow(vec![tab_bar_shadow])
                    .p_1() // Padding around tabs
                    .gap_1() // Gap between tabs
                    .children(tab_labels.iter().enumerate().map({
                        let active_index = active_index;
                        let theme = theme.clone();
                        let tab_py = tab_py;
                        let tab_px = tab_px;
                        move |(index, label)| {
                            let is_active = index == active_index;
                            let label = label.clone();
                            let tab_index = index;
                            
                            div()
                                .relative()
                                .flex()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .py(tab_py)
                                .px(tab_px)
                                .rounded(px(BorderRadius::MD))
                                .text_size(size.font_size())
                                .when(is_active, |this| {
                                    this.bg(rgb(0xFFFFFF)) // White background for active tab
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x000000)) // Black text for active tab
                                        .shadow(vec![make_active_tab_shadow()])
                                })
                                .when(!is_active, |this| {
                                    this.text_color(theme.colors.text_secondary) // Gray text for inactive tabs
                                        .cursor(CursorStyle::PointingHand)
                                })
                                .on_mouse_down(MouseButton::Left, cx.listener({
                                    let tab_index = tab_index;
                                    move |this, _event: &MouseDownEvent, _window, cx| {
                                        if this.active_index != tab_index {
                                            this.active_index = tab_index;
                                            cx.emit(TabsEvent::TabChanged { index: tab_index });
                                            cx.notify();
                                        }
                                    }
                                }))
                                .child(label)
                        }
                    }))
            )
            .child(
                // Content area
                {
                    let active_index = active_index;
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .items_center()
                        .text_color(theme.colors.text_secondary)
                        .when(active_index < tabs_ref.len(), |this| {
                            let tab = &tabs_ref[active_index];
                            let content = (tab.content)();
                            this.child(content)
                        })
                        .when(active_index >= tabs_ref.len(), |this| {
                            this.child("No content")
                        })
                }
            )
    }
}

