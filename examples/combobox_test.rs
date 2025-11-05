use gpui::*;
use fluix::{Combobox, SelectOption, ComboboxEvent};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(1000.), px(800.)),  // 更大的窗口
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Combobox Test - Fixed".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| ComboboxTest::new(window, cx))
        }).unwrap();
    });
}

struct ComboboxTest {
    combobox: Entity<Combobox>,
    selected_value: String,
}

impl ComboboxTest {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let combobox = cx.new(|cx| {
            Combobox::new(cx)
                .placeholder("Search or select a model...")
                .options(vec![
                    SelectOption::new("gpt-4", "GPT-4"),
                    SelectOption::new("gpt-3.5", "GPT-3.5 Turbo"),
                    SelectOption::new("claude-3-opus", "Claude 3 Opus"),
                    SelectOption::new("claude-3-sonnet", "Claude 3 Sonnet"),
                    SelectOption::new("claude-2", "Claude 2"),
                    SelectOption::new("gemini-pro", "Gemini Pro"),
                    SelectOption::new("llama-2", "Llama 2"),
                    SelectOption::new("mistral", "Mistral"),
                ])
        });

        cx.subscribe_in(&combobox, _window, |this: &mut Self, _combobox, event: &ComboboxEvent, _window, cx| {
            match event {
                ComboboxEvent::Changed(value) => {
                    this.selected_value = value.clone();
                    println!("✅ Selected: {}", value);
                    cx.notify();
                }
                ComboboxEvent::InputChanged(value) => {
                    println!("📝 Input changed: {}", value);
                }
            }
        }).detach();

        Self {
            combobox,
            selected_value: String::new(),
        }
    }
}

impl Render for ComboboxTest {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xF5F5F5))
            .p(px(60.))  // 更大的内边距
            .gap(px(30.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(10.))
                    .child(
                        div()
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1F2937))
                            .child("Combobox Component Test")
                    )
                    .child(
                        div()
                            .text_base()
                            .text_color(rgb(0x6B7280))
                            .child("Test the fixed combobox functionality")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(600.))
                    .p(px(40.))
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xE5E7EB))
                    .rounded(px(12.))
                    .shadow(vec![BoxShadow {
                        color: rgba(0x00000010).into(),
                        offset: point(px(0.), px(4.)),
                        blur_radius: px(20.),
                        spread_radius: px(0.),
                    }])
                    .gap(px(24.))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1F2937))
                            .child("Select AI Model:")
                    )
                    .child(
                        div()
                            .h(px(50.))  // 固定高度容器
                            .child(self.combobox.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x374151))
                                    .child("Selected Value:")
                            )
                            .child(
                                div()
                                    .text_base()
                                    .text_color(if self.selected_value.is_empty() { rgb(0x9CA3AF) } else { rgb(0x059669) })
                                    .child(if self.selected_value.is_empty() {
                                        "No selection".to_string()
                                    } else {
                                        self.selected_value.clone()
                                    })
                            )
                    )
            )
    }
}

