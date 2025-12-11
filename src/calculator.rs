use gpui::{Context, EventEmitter};
use std::fmt;

/// 计算器事件类型
#[derive(Debug, Clone)]
pub enum CalculatorEvent {
    /// 数字按键事件，包含数字字符
    NumberPressed(char),
    /// 操作符按键事件，包含操作符字符
    OperatorPressed(char),
    /// 小数点按键事件
    DecimalPressed,
    /// 括号按键事件，包含括号字符
    ParenthesisPressed(char),
    /// 百分比按键事件
    PercentagePressed,
    /// 清除按键事件
    ClearPressed,
    /// 删除按键事件
    DeletePressed,
    /// 等于按键事件，触发计算
    EqualsPressed,
    /// 表达式发生变化，包含新的表达式字符串
    ExpressionChanged(String),
    /// 计算结果，包含计算结果值
    ResultCalculated(f64),
    /// 计算器状态重置
    StateReset,
}

/// 计算器状态
#[derive(Debug, Clone)]
pub struct CalculatorState {
    /// 当前表达式
    pub expression: String,
    /// 上一个计算结果（如果有）
    pub last_result: Option<f64>,
    /// 是否刚执行过计算
    pub just_calculated: bool,
    /// 是否表达式为空
    pub is_empty: bool,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            expression: "0".to_string(),
            last_result: None,
            just_calculated: false,
            is_empty: false,
        }
    }
}

impl fmt::Display for CalculatorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.just_calculated && self.last_result.is_some() {
            write!(f, "{}", self.last_result.unwrap())
        } else {
            write!(f, "{}", self.expression)
        }
    }
}

/// 计算器实体 - 管理计算器状态和事件
pub struct Calculator {
    state: CalculatorState,
}

impl Calculator {
    /// 创建新的计算器实例
    pub fn new() -> Self {
        Self {
            state: CalculatorState::default(),
        }
    }

    /// 处理数字输入
    pub fn input_number(&mut self, number: char, cx: &mut Context<Self>) {
        if !number.is_ascii_digit() {
            return;
        }

        if self.state.just_calculated {
            self.state.expression.clear();
            self.state.just_calculated = false;
        }

        if self.state.expression == "0" {
            self.state.expression = number.to_string();
        } else {
            self.state.expression.push(number);
        }

        self.state.is_empty = self.state.expression.is_empty();

        cx.emit(CalculatorEvent::NumberPressed(number));
        cx.emit(CalculatorEvent::ExpressionChanged(
            self.state.expression.clone(),
        ));
        cx.notify();
    }

    /// 处理操作符输入
    pub fn input_operator(&mut self, operator: char, cx: &mut Context<Self>) {
        // 百分比由单独的 input_percentage 方法处理
        if operator == '%' {
            self.input_percentage(cx);
            return;
        }

        // 标准化操作符
        let normalized_op = match operator {
            '×' => '*',
            '÷' => '/',
            _ => operator,
        };

        // 检查标准化后的操作符是否有效
        let valid_operators = ['+', '-', '*', '/'];
        if !valid_operators.contains(&normalized_op) {
            return;
        }

        if self.state.just_calculated {
            if let Some(result) = self.state.last_result {
                self.state.expression = format!("{}", result);
            }
            self.state.just_calculated = false;
        }

        // 确保表达式不以操作符结尾
        if let Some(last_char) = self.state.expression.chars().last() {
            let all_operators = ['+', '-', '*', '/', '×', '÷'];
            if all_operators.contains(&last_char) {
                // 替换最后一个操作符
                self.state.expression.pop();
            }
        }

        self.state.expression.push(normalized_op);

        cx.emit(CalculatorEvent::OperatorPressed(operator));
        cx.emit(CalculatorEvent::ExpressionChanged(
            self.state.expression.clone(),
        ));
        cx.notify();
    }

    /// 处理百分比输入
    pub fn input_percentage(&mut self, cx: &mut Context<Self>) {
        if self.state.just_calculated {
            if let Some(result) = self.state.last_result {
                self.state.expression = format!("{}", result);
            }
            self.state.just_calculated = false;
        }

        // 添加百分号到表达式
        self.state.expression.push('%');

        cx.emit(CalculatorEvent::PercentagePressed);
        cx.emit(CalculatorEvent::ExpressionChanged(
            self.state.expression.clone(),
        ));
        cx.notify();
    }

    /// 处理括号输入
    pub fn input_parenthesis(&mut self, parenthesis: char, cx: &mut Context<Self>) {
        if !['(', ')'].contains(&parenthesis) {
            return;
        }

        if self.state.just_calculated {
            self.state.expression = "0".to_string();
            self.state.just_calculated = false;
        }

        // 检查括号是否与表达式匹配
        if parenthesis == ')' {
            // 统计现有的左右括号数量，确保不会添加不匹配的右括号
            let left_count = self.state.expression.chars().filter(|&c| c == '(').count();
            let right_count = self.state.expression.chars().filter(|&c| c == ')').count();
            if left_count <= right_count {
                // 右括号多于左括号，不添加
                return;
            }
        }

        // 对于左括号，如果前面是数字，可能需要添加乘号
        if parenthesis == '(' {
            if let Some(last_char) = self.state.expression.chars().last() {
                if last_char.is_ascii_digit() || last_char == ')' {
                    // 数字后面直接跟左括号，需要添加乘号
                    self.state.expression.push('*');
                }
            }
        }

        // 对于右括号，如果后面是数字，可能需要添加乘号
        if parenthesis == ')' {
            // 这里主要是为了显示，实际计算由解析器处理
            // 右括号后面跟随数字时，会在计算时由解析器处理
        }

        self.state.expression.push(parenthesis);

        cx.emit(CalculatorEvent::ParenthesisPressed(parenthesis));
        cx.emit(CalculatorEvent::ExpressionChanged(
            self.state.expression.clone(),
        ));
        cx.notify();
    }

    /// 处理小数点输入
    pub fn input_decimal(&mut self, cx: &mut Context<Self>) {
        if self.state.just_calculated {
            self.state.expression = "0".to_string();
            self.state.just_calculated = false;
        }

        // 检查当前数字是否已经包含小数点
        let parts: Vec<&str> = self.state.expression.split(['+', '-', '*', '/']).collect();
        if let Some(current_number) = parts.last() {
            if !current_number.contains('.') {
                self.state.expression.push('.');

                cx.emit(CalculatorEvent::DecimalPressed);
                cx.emit(CalculatorEvent::ExpressionChanged(
                    self.state.expression.clone(),
                ));
                cx.notify();
            }
        } else {
            self.state.expression.push('.');

            cx.emit(CalculatorEvent::DecimalPressed);
            cx.emit(CalculatorEvent::ExpressionChanged(
                self.state.expression.clone(),
            ));
            cx.notify();
        }
    }

    /// 清除计算器状态
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.state = CalculatorState::default();

        cx.emit(CalculatorEvent::ClearPressed);
        cx.emit(CalculatorEvent::StateReset);
        cx.notify();
    }

    /// 删除最后一个字符
    pub fn delete(&mut self, cx: &mut Context<Self>) {
        if self.state.just_calculated {
            self.clear(cx);
            return;
        }

        self.state.expression.pop();

        if self.state.expression.is_empty() {
            self.state.expression = "0".to_string();
        }

        self.state.is_empty = self.state.expression.is_empty();

        cx.emit(CalculatorEvent::DeletePressed);
        cx.emit(CalculatorEvent::ExpressionChanged(
            self.state.expression.clone(),
        ));
        cx.notify();
    }

    /// 执行计算
    pub fn calculate(&mut self, cx: &mut Context<Self>) -> Option<f64> {
        use crate::logic;

        if self.state.expression.is_empty() || self.state.expression == "0" {
            return None;
        }

        // 标准化表达式字符串（将×和÷替换为*和/）
        let mut normalized_expr = self.state.expression.replace('×', "*").replace('÷', "/");

        // 处理百分比表达式
        normalized_expr = self.process_percentage_expression(&normalized_expr);

        match logic::calculate(&normalized_expr) {
            result => {
                self.state.last_result = Some(result);
                self.state.just_calculated = true;

                cx.emit(CalculatorEvent::EqualsPressed);
                cx.emit(CalculatorEvent::ResultCalculated(result));
                cx.notify();

                Some(result)
            }
        }
    }

    /// 处理百分比表达式，将 % 转换为 /100.0 或根据上下文处理
    fn process_percentage_expression(&self, expr: &str) -> String {
        let mut result = String::new();
        let mut chars = expr.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                // 处理百分比：转换为 /100.0
                // 如果是紧跟在数字后面，比如 "50%" 变成 "50/100.0"
                // 如果是 "50+10%" 需要根据上下文，这里简单处理为除以100
                result.push_str("/100.0");
            } else {
                result.push(c);
            }
        }

        result
    }

    /// 获取当前表达式
    pub fn expression(&self) -> &str {
        &self.state.expression
    }

    /// 获取当前显示内容
    pub fn display_text(&self) -> String {
        if self.state.just_calculated {
            if let Some(result) = self.state.last_result {
                // 格式化结果，移除不必要的尾随零
                let result_str = format!("{}", result);
                if result_str.ends_with(".0") {
                    result_str.trim_end_matches(".0").to_string()
                } else {
                    result_str
                }
            } else {
                "0".to_string()
            }
        } else if self.state.expression.is_empty() {
            "0".to_string()
        } else {
            // 将*和/转换回×和÷以便显示
            self.state.expression.replace('*', "×").replace('/', "÷")
        }
    }

    /// 获取计算器状态
    pub fn state(&self) -> &CalculatorState {
        &self.state
    }

    /// 从字符串设置表达式（用于测试或恢复状态）
    pub fn set_expression(&mut self, expr: &str, cx: &mut Context<Self>) {
        self.state.expression = expr.to_string();
        self.state.just_calculated = false;
        self.state.is_empty = expr.is_empty();

        cx.emit(CalculatorEvent::ExpressionChanged(
            self.state.expression.clone(),
        ));
        cx.notify();
    }
}

/// 为计算器实现事件发射器
impl EventEmitter<CalculatorEvent> for Calculator {}

impl Clone for Calculator {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
