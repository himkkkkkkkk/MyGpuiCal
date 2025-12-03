use crate::button;
use crate::show::Show;
use crate::style;
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(IntoElement, Clone)]
pub struct TouchArea {
    children: Vec<Vec<button::Button>>,
    show_state: Entity<Show>,
}

impl TouchArea {
    pub fn new(show_state: &Entity<Show>) -> Self {
        let mut children = vec![];
        let mut temp = vec![];
        let icon = vec![
            "←", "AC", "%", "÷", "7", "8", "9", "×", "4", "5", "6", "-", "1", "2", "3", "+", ",",
            "0", ".", "=",
        ];
        for i in icon {
            temp.push(button::Button::new(SharedString::from(format!("{}", i))));

            if temp.len() == 4 {
                children.push(temp.clone());
                temp.clear();
            }
        }
        Self {
            children,
            show_state: show_state.clone(),
        }
    }
}

impl RenderOnce for TouchArea {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(style::PAD_COLOR))
            .h_full()
            .py(DefiniteLength::Fraction(0.02))
            .gap(DefiniteLength::Fraction(0.02))
            .justify_around()
            .children(self.children.into_iter().map(|child| {
                div()
                    .id("act")
                    .flex()
                    .gap_1()
                    .m_neg_0p5()
                    .children(child.into_iter().map(|child| {
                        let show_state = self.show_state.clone();
                        let l = child.get_label();
                        let lc = l.to_string().clone();
                        div()
                            .id(l)
                            .flex()
                            .text_color(rgb(style::WHITE_COLOR))
                            .hover(|this| this.bg(rgb(style::BUTTON_COLOR_HOVER)))
                            .when(!"1234567890".contains(&lc), |this| {
                                this.bg(rgb(style::BUTTON_COLOR))
                                    .text_color(rgb(style::PRIMARY_COLOR))
                            })
                            .h(DefiniteLength::Fraction(0.8))
                            .w_full()
                            .on_click(move |_, window, cx| {
                                show_state.update(cx, |show, cx| match lc.as_str() {
                                    "←" => show.delete_char(window, cx),
                                    "AC" => show.reset(window, cx),
                                    "=" => show.cal(),
                                    _ => show.add_str(window, cx, &lc),
                                })
                            })
                            .rounded_lg()
                            .text_lg()
                            .text_center()
                            .justify_center()
                            .items_center()
                            .child(child.get_label())
                    }))
            }))
    }
}
