use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconName {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Check,
    ChevronUpDown,
    Close,
    Plus,
    Minus,
    Search,
    Settings,
    Home,
    User,
    Bell,
    Star,
    Heart,
    Menu,
    Info,
    Warning,
    Error,
    Success,
    UnfoldMore,
    Send,
}

impl IconName {
    /// Get the SVG file path for the icon
    /// Following Zed's pattern: IconName::path() returns the asset path
    pub fn path(self) -> &'static str {
        match self {
            Self::ArrowLeft => "icons/arrow-left.svg",
            Self::ArrowRight => "icons/arrow-right.svg",
            Self::ArrowUp => "icons/arrow-up.svg",
            Self::ArrowDown => "icons/arrow-down.svg",
            Self::Check => "icons/check.svg",
            Self::ChevronUpDown => "icons/chevron-up-down.svg",
            Self::Close => "icons/close.svg",
            Self::Plus => "icons/plus.svg",
            Self::Minus => "icons/minus.svg",
            Self::Search => "icons/search.svg",
            Self::Settings => "icons/settings.svg",
            Self::Home => "icons/home.svg",
            Self::User => "icons/user.svg",
            Self::Bell => "icons/bell.svg",
            Self::Star => "icons/star.svg",
            Self::Heart => "icons/heart.svg",
            Self::Menu => "icons/menu.svg",
            Self::Info => "icons/info.svg",
            Self::Warning => "icons/warning.svg",
            Self::Error => "icons/error.svg",
            Self::Success => "icons/success.svg",
            Self::UnfoldMore => "icons/unfold-more.svg",
            Self::Send => "icons/send.svg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconSize {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    Custom(f32),
}

impl IconSize {
    pub fn px(&self) -> Pixels {
        match self {
            Self::XSmall => px(12.0),
            Self::Small => px(16.0),
            Self::Medium => px(20.0),
            Self::Large => px(24.0),
            Self::XLarge => px(32.0),
            Self::Custom(size) => px(*size),
        }
    }
}

impl Default for IconSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Background shape for the icon
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconBackground {
    /// No background
    None,
    /// Square background (equal width and height)
    Square,
    /// Rectangular background (custom width and height)
    Rectangle { width: Pixels, height: Pixels },
}

pub struct Icon {
    name: IconName,
    size: IconSize,
    color: Option<Rgba>,
    background: IconBackground,
    background_color: Option<Rgba>,
    border_radius: Option<Pixels>,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: IconSize::default(),
            color: None,
            background: IconBackground::None,
            background_color: None,
            border_radius: None,
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Rgba) -> Self {
        self.color = Some(color);
        self
    }

    pub fn xsmall(mut self) -> Self {
        self.size = IconSize::XSmall;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = IconSize::Small;
        self
    }

    pub fn medium(mut self) -> Self {
        self.size = IconSize::Medium;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = IconSize::Large;
        self
    }

    pub fn xlarge(mut self) -> Self {
        self.size = IconSize::XLarge;
        self
    }

    /// Add a square background
    pub fn with_square_bg(mut self, color: Rgba) -> Self {
        self.background = IconBackground::Square;
        self.background_color = Some(color);
        self
    }

    /// Add a rectangular background
    pub fn with_rect_bg(mut self, width: Pixels, height: Pixels, color: Rgba) -> Self {
        self.background = IconBackground::Rectangle { width, height };
        self.background_color = Some(color);
        self
    }

    /// Set background color (requires background to be set first)
    pub fn bg_color(mut self, color: Rgba) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set border radius for background
    pub fn rounded(mut self, radius: Pixels) -> Self {
        self.border_radius = Some(radius);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = self.size.px();
        let color = self.color.unwrap_or(rgb(0x333333));
        let path = self.name.path();

        let icon = svg()
            .path(path)
            .size(size)
            .text_color(color)
            .flex_none();

        // Wrap in background if specified
        match self.background {
            IconBackground::None => icon.into_any_element(),
            IconBackground::Square => {
                let bg_color = self.background_color.unwrap_or(rgb(0xF3F4F6));
                let padding = size * 0.25; // 25% padding
                let total_size = size + padding * 2.0;

                let mut container = div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(total_size)
                    .bg(bg_color);

                if let Some(radius) = self.border_radius {
                    container = container.rounded(radius);
                }

                container.child(icon).into_any_element()
            }
            IconBackground::Rectangle { width, height } => {
                let bg_color = self.background_color.unwrap_or(rgb(0xF3F4F6));

                let mut container = div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(width)
                    .h(height)
                    .bg(bg_color);

                if let Some(radius) = self.border_radius {
                    container = container.rounded(radius);
                }

                container.child(icon).into_any_element()
            }
        }
    }
}

impl IntoElement for Icon {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
