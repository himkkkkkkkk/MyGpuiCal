use crate::logic;
use crate::style;
use gpui::*;

#[derive(IntoElement, Clone)]
pub struct Show {
    expr: SharedString,
    have_eq: bool,
}

impl Show {
    pub fn new() -> Self {
        Self {
            expr: SharedString::new("0.0"),
            have_eq: false,
        }
    }
    pub fn reset(&mut self, _window: &mut Window, _cx: &mut App) {
        self.expr = SharedString::new("0.0");
        self.have_eq = false;
    }

    pub fn add_str(&mut self, _window: &mut Window, _cx: &mut App, c: &str) {
        if self.have_eq && "1234567890".contains(c) {
            self.expr = SharedString::from("");
            self.have_eq = false;
        } else if self.have_eq {
            self.have_eq = false;
        }
        if self.expr.to_string() == "0.0" {
            self.expr = SharedString::from(format!("{}", c))
        } else {
            self.expr = SharedString::from(format!("{}{}", self.expr, c))
        }
    }

    pub fn delete_char(&mut self, window: &mut Window, cx: &mut App) {
        if self.have_eq {
            self.expr = SharedString::from("");
            self.have_eq = false;
        }
        let mut s = self.expr.to_string();
        s.pop();
        self.expr = SharedString::from(s);
        if self.expr.is_empty() {
            self.reset(window, cx);
        }
    }
    pub fn cal(&mut self) {
        self.expr = SharedString::from(format!("{}", logic::calculate(self.expr.as_str())));
        self.have_eq = true;
    }
}

impl RenderOnce for Show {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .h(DefiniteLength::Fraction(0.2))
            .px_4()
            .w_full()
            .flex()
            .items_center()
            .justify_end()
            .text_color(rgb(style::PRIMARY_COLOR))
            .bg(rgb(style::DISPLAY_COLOR))
            .child(self.expr)
    }
}
