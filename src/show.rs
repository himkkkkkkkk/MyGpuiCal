use crate::calculator::Calculator;
use crate::style;
use gpui::*;

/// 显示组件 - 显示计算器的当前状态
pub struct Show {
    /// 计算器实体的引用
    calculator: Entity<Calculator>,
}

impl Show {
    /// 创建新的显示组件
    pub fn new(calculator: Entity<Calculator>, cx: &mut Context<Self>) -> Self {
        // 订阅计算器事件，当计算器状态变化时重新渲染显示组件
        cx.observe(
            &calculator,
            |_this: &mut Show, _calculator: Entity<Calculator>, cx| {
                // 当计算器状态变化时，触发显示组件的重新渲染
                cx.notify();
            },
        )
        .detach();

        Self { calculator }
    }

    // 获取当前显示文本
    // pub fn display_text(&self, cx: &App) -> String {
    //     self.calculator.read(cx).display_text()
    // }
}

impl Render for Show {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 从计算器获取当前显示文本
        let display_text = self.calculator.read(cx).display_text();

        div()
            .id("calculator-display")
            //.h(DefiniteLength::Fraction(0.2))
            .h_full()
            .px_4()
            .w_full()
            .flex()
            .items_center()
            .justify_end()
            .text_color(rgb(style::PRIMARY_COLOR))
            .bg(rgb(style::DISPLAY_COLOR))
            .text_3xl()
            .font_family("JetBrainsMono Nerd Font")
            // 确保文本不会被截断，允许滚动
            .overflow_x_scroll()
            .child(display_text)
    }
}
