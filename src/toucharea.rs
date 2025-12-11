use crate::calculator::Calculator;
use crate::style;
use gpui::*;

/// 按钮类型枚举
enum ButtonType {
    Number(char),
    Operator(char),
    Decimal,
    Clear,
    Delete,
    Equals,
}

/// 触摸区域组件 - 包含计算器按钮网格
pub struct TouchArea {
    /// 计算器实体的引用
    calculator: Entity<Calculator>,
}

impl TouchArea {
    /// 创建新的触摸区域组件
    pub fn new(calculator: Entity<Calculator>) -> Self {
        Self { calculator }
    }

    /// 获取按钮网格配置
    fn button_grid() -> Vec<Vec<(SharedString, ButtonType)>> {
        vec![
            vec![
                ("←".into(), ButtonType::Delete),
                ("AC".into(), ButtonType::Clear),
                ("%".into(), ButtonType::Operator('%')),
                ("÷".into(), ButtonType::Operator('÷')),
            ],
            vec![
                ("7".into(), ButtonType::Number('7')),
                ("8".into(), ButtonType::Number('8')),
                ("9".into(), ButtonType::Number('9')),
                ("×".into(), ButtonType::Operator('×')),
            ],
            vec![
                ("4".into(), ButtonType::Number('4')),
                ("5".into(), ButtonType::Number('5')),
                ("6".into(), ButtonType::Number('6')),
                ("-".into(), ButtonType::Operator('-')),
            ],
            vec![
                ("1".into(), ButtonType::Number('1')),
                ("2".into(), ButtonType::Number('2')),
                ("3".into(), ButtonType::Number('3')),
                ("+".into(), ButtonType::Operator('+')),
            ],
            vec![
                ("，".into(), ButtonType::Operator('，')),
                ("0".into(), ButtonType::Number('0')),
                (".".into(), ButtonType::Decimal),
                ("=".into(), ButtonType::Equals),
            ],
        ]
    }
}

impl Render for TouchArea {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let calculator = self.calculator.clone();
        let button_grid = Self::button_grid();

        div()
            .id("calculator-keypad")
            .flex()
            .flex_col()
            .bg(rgb(style::PAD_COLOR))
            .size_full()
            .p_1()
            .gap_1()
            .children(button_grid.into_iter().map(|row| {
                div().flex().flex_1().gap_1().children(row.into_iter().map(
                    |(label, button_type)| {
                        let calculator = calculator.clone();

                        // 确定按钮的样式类
                        let is_number = matches!(button_type, ButtonType::Number(_));
                        let is_operator = matches!(button_type, ButtonType::Operator(_));
                        let is_special = matches!(
                            button_type,
                            ButtonType::Clear | ButtonType::Delete | ButtonType::Equals
                        );

                        let base_style = div()
                            .id(label.clone())
                            .flex()
                            .flex_1()
                            .rounded_md()
                            .text_lg()
                            .text_center()
                            .justify_center()
                            .items_center()
                            .child(label.clone());

                        // 应用样式
                        let styled_button = if is_number {
                            base_style
                                .bg(rgb(style::BUTTON_COLOR))
                                .text_color(rgb(style::WHITE_COLOR))
                                .hover(|this| this.bg(rgb(style::BUTTON_COLOR_HOVER)))
                        } else if is_operator || is_special {
                            base_style
                                .bg(rgb(style::BUTTON_COLOR))
                                .text_color(rgb(style::PRIMARY_COLOR))
                                .hover(|this| this.bg(rgb(0x707070))) // 操作符悬停色
                        } else {
                            // 小数点按钮
                            base_style
                                .bg(rgb(style::BUTTON_COLOR))
                                .text_color(rgb(style::WHITE_COLOR))
                                .hover(|this| this.bg(rgb(style::BUTTON_COLOR_HOVER)))
                        };

                        // 添加点击事件处理
                        styled_button.on_click(move |_event, _window, cx| {
                            calculator.update(cx, |calculator, cx| match button_type {
                                ButtonType::Number(num) => {
                                    calculator.input_number(num, cx);
                                }
                                ButtonType::Operator(op) => {
                                    if op == '(' || op == ')' {
                                        calculator.input_parenthesis(op, cx);
                                    } else {
                                        calculator.input_operator(op, cx);
                                    }
                                }
                                ButtonType::Decimal => {
                                    calculator.input_decimal(cx);
                                }
                                ButtonType::Clear => {
                                    calculator.clear(cx);
                                }
                                ButtonType::Delete => {
                                    calculator.delete(cx);
                                }
                                ButtonType::Equals => {
                                    calculator.calculate(cx);
                                }
                            });
                        })
                    },
                ))
            }))
    }
}
