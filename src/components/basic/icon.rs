use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconName {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Check,
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
}

impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            Self::ArrowLeft => "icons/arrow-left.svg",
            Self::ArrowRight => "icons/arrow-right.svg",
            Self::ArrowUp => "icons/arrow-up.svg",
            Self::ArrowDown => "icons/arrow-down.svg",
            Self::Check => "icons/check.svg",
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

pub struct Icon {
    path: SharedString,
    size: IconSize,
    color: Option<Rgba>,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            path: name.path().into(),
            size: IconSize::default(),
            color: None,
        }
    }

    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            size: IconSize::default(),
            color: None,
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
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = self.size.px();
        let color = self.color.unwrap_or(rgb(0x333333));

        svg()
            .path(self.path)
            .size(size)
            .text_color(color)
            .flex_none()
    }
}

impl IntoElement for Icon {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
