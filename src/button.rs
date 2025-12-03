use gpui::{IntoElement, ParentElement, Rems, RenderOnce, SharedString, Styled, div};

#[derive(IntoElement, Clone)]
pub struct Button {
    label: SharedString,
}

impl Button {
    pub fn new(s: SharedString) -> Self {
        Self { label: s }
    }
    pub fn get_label(&self) -> SharedString {
        self.label.clone()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        div()
            .size_24()
            .text_size(Rems(4.0))
            .text_center()
            .child(self.label)
    }
}
