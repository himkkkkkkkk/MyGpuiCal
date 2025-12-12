use gpui::*;
mod calculator;
mod logic;
mod show;
mod style;
mod toucharea;

use calculator::Calculator;
use show::Show;
use toucharea::TouchArea;

struct Root {
    show: Entity<Show>,
    toucharea: Entity<TouchArea>,
}

impl Root {
    fn new(cx: &mut App) -> Self {
        let calculator = cx.new(|_| Calculator::new());
        let show = cx.new(|cx| Show::new(calculator.clone(), cx));
        let toucharea = cx.new(|_| TouchArea::new(calculator.clone()));
        Self { show, toucharea }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .font_family(SharedString::from("JetBrainsMono Nerd Font"))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(DefiniteLength::Fraction(0.2))
                    .child(self.show.clone()),
            )
            .child(div().flex_1().child(self.toucharea.clone()))
    }
}

fn main() {
    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(400.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| Root::new(cx)),
        )
        .unwrap();
    });
}
