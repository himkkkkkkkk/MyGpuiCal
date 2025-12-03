use gpui::*;
// use std::io;
mod button;
mod logic;
mod show;
mod style;
mod toucharea;

//use button::Button;
use show::Show;
use toucharea::TouchArea;

struct Root {
    show: Entity<Show>,
    toucharea: Entity<TouchArea>,
}

impl Root {
    fn new(cx: &mut App) -> Self {
        let show = cx.new(|_| Show::new());
        let toucharea = cx.new(|_| TouchArea::new(&show));
        Self { show, toucharea }
    }
}
impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .font_family(SharedString::from("JetBrainsMono Nerd Font"))
            .child(div().child(self.show.read(cx).clone()))
            .child(div().child(self.toucharea.read(cx).clone()))
    }
}

fn main() {
    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(300.0), px(180.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| Root::new(cx)),
        )
        .unwrap();
    });
    todo!("重构代码,完成剩余的功能");
}
