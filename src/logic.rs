use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(Operation),
    LeftParen,
    RightParen,
}

/// 将字符转换为对应的 Token
fn char_to_token(c: char) -> Option<Token> {
    match c {
        '+' => Some(Token::Operator(Operation::Plus)),
        '-' => Some(Token::Operator(Operation::Minus)),
        '*' | '×' => Some(Token::Operator(Operation::Times)),
        '/' | '÷' => Some(Token::Operator(Operation::Divide)),
        '(' => Some(Token::LeftParen),
        ')' => Some(Token::RightParen),
        _ => None,
    }
}

/// 执行二元运算
fn apply_operation(a: f64, b: f64, op: &Operation) -> f64 {
    match op {
        Operation::Plus => a + b,
        Operation::Minus => a - b,
        Operation::Times => a * b,
        Operation::Divide => a / b,
    }
}

/// 获取操作符优先级（值越大优先级越高）
fn operator_precedence(op: &Operation) -> u8 {
    match op {
        Operation::Plus | Operation::Minus => 1,
        Operation::Times | Operation::Divide => 2,
    }
}

/// 将表达式字符串解析为 Token 序列
fn parse_expression(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_number = String::new();
    let mut chars = expr.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() || c == '.' {
            // 数字或小数点
            current_number.push(c);
        } else {
            // 如果当前有数字，先将其解析为数字 Token
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    tokens.push(Token::Number(num));
                }
                current_number.clear();
            }

            // 处理操作符或括号
            if let Some(token) = char_to_token(c) {
                tokens.push(token);
            } else if !c.is_whitespace() {
                // 忽略空白字符，其他字符视为错误（实际应用中应该报错）
                eprintln!("警告：忽略未知字符 '{}'", c);
            }
        }
    }

    // 处理末尾可能存在的数字
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<f64>() {
            tokens.push(Token::Number(num));
        }
    }

    tokens
}

/// 使用 Shunting-yard 算法将中缀表达式转换为后缀表达式（逆波兰表示法）
fn infix_to_postfix(tokens: &[Token]) -> Vec<Token> {
    let mut output = VecDeque::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => {
                // 数字直接加入输出队列
                output.push_back(token.clone());
            }
            Token::LeftParen => {
                // 左括号直接压入操作符栈
                operator_stack.push(token.clone());
            }
            Token::RightParen => {
                // 遇到右括号，弹出操作符栈中的元素直到遇到左括号
                while let Some(top) = operator_stack.pop() {
                    if matches!(top, Token::LeftParen) {
                        break;
                    }
                    output.push_back(top);
                }
            }
            Token::Operator(op1) => {
                // 处理操作符优先级
                while let Some(top) = operator_stack.last() {
                    match top {
                        Token::LeftParen => break, // 左括号在栈中时停止弹出
                        Token::Operator(op2) => {
                            // 如果栈顶操作符优先级大于等于当前操作符，弹出栈顶
                            if operator_precedence(op2) >= operator_precedence(op1) {
                                output.push_back(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                operator_stack.push(token.clone());
            }
        }
    }

    // 将操作符栈中剩余的操作符全部弹出到输出队列
    while let Some(token) = operator_stack.pop() {
        output.push_back(token);
    }

    output.into_iter().collect()
}

/// 计算后缀表达式的结果
fn evaluate_postfix(tokens: &[Token]) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => {
                stack.push(*num);
            }
            Token::Operator(op) => {
                // 二元操作需要两个操作数
                if stack.len() < 2 {
                    return None;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = apply_operation(a, b, op);
                stack.push(result);
            }
            _ => {
                // 后缀表达式中不应该有括号
                return None;
            }
        }
    }

    // 最终栈中应该只有一个值
    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}

/// 主计算函数：计算表达式字符串的结果
pub fn calculate(expr: &str) -> f64 {
    // 移除空白字符
    let expr = expr.trim();

    // 空表达式返回0
    if expr.is_empty() {
        return 0.0;
    }

    // 1. 解析表达式
    let tokens = parse_expression(expr);

    // 如果解析失败，返回0
    if tokens.is_empty() {
        return 0.0;
    }

    // 2. 转换为后缀表达式
    let postfix_tokens = infix_to_postfix(&tokens);

    // 3. 计算后缀表达式
    if let Some(result) = evaluate_postfix(&postfix_tokens) {
        result
    } else {
        // 计算失败时返回0
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        assert_eq!(calculate("1+2"), 3.0);
        assert_eq!(calculate("5-3"), 2.0);
        assert_eq!(calculate("2*3"), 6.0);
        assert_eq!(calculate("6/2"), 3.0);
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(calculate("2+3*4"), 14.0); // 3*4=12, 2+12=14
        assert_eq!(calculate("3*4+2"), 14.0); // 3*4=12, 12+2=14
        assert_eq!(calculate("10-6/2"), 7.0); // 6/2=3, 10-3=7
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(calculate("(1+2)*3"), 9.0); // (1+2)=3, 3*3=9
        assert_eq!(calculate("1+(2*3)"), 7.0); // 2*3=6, 1+6=7
        assert_eq!(calculate("(1+2)*(3+4)"), 21.0); // (1+2)=3, (3+4)=7, 3*7=21
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(calculate("((1+2)*3)+4"), 13.0); // (1+2)=3, 3*3=9, 9+4=13
        assert_eq!(calculate("1+(2*(3+4))"), 15.0); // (3+4)=7, 2*7=14, 1+14=15
    }

    #[test]
    fn test_decimal_numbers() {
        assert_eq!(calculate("1.5+2.5"), 4.0);
        assert_eq!(calculate("3.14*2"), 6.28);
        assert_eq!(calculate("10.0/4.0"), 2.5);
    }

    #[test]
    fn test_complex_expressions() {
        assert_eq!(calculate("3+4*2/(1-5)"), 1.0); // 3+4*2/(-4)=3+8/(-4)=3-2=1
        assert_eq!(calculate("(3+4)*(2+3)"), 35.0); // 7*5=35
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(calculate(""), 0.0);
        assert_eq!(calculate("   "), 0.0);
        assert_eq!(calculate("((()))"), 0.0); // 只有括号，没有数字
        assert_eq!(calculate("2++3"), 0.0); // 语法错误，返回0
    }
}
