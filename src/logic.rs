#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Minus,
    Times,
    Frac,
}

#[derive(Debug)]
enum Token {
    Num(f64),
    Op(Operation),
}

fn c2op(c: char) -> Option<Operation> {
    match c {
        '+' => Some(Operation::Plus),
        '-' => Some(Operation::Minus),
        '*' => Some(Operation::Times),
        '/' => Some(Operation::Frac),
        '÷' => Some(Operation::Frac),
        '×' => Some(Operation::Times),
        _ => None,
    }
}

fn cal2(a: f64, b: f64, op: Operation) -> f64 {
    match op {
        Operation::Plus => a + b,
        Operation::Minus => a - b,
        Operation::Times => a * b,
        Operation::Frac => a / b,
    }
}

fn procedure(a: &Operation) -> i8 {
    match a {
        Operation::Plus => 1,
        Operation::Minus => 1,
        Operation::Times => 2,
        Operation::Frac => 2,
    }
}

pub fn calculate(s: &str) -> f64 {
    let s = s.trim();
    let mut temp = String::new();
    let mut token = Vec::new();
    for c in s.chars() {
        if c.is_ascii_digit() || c == '.' {
            temp.push(c);
        } else {
            if !temp.is_empty() {
                token.push(Token::Num(temp.parse().unwrap()));
                temp.clear();
            }
            token.push(Token::Op(c2op(c).unwrap()));
        }
    }
    if !temp.is_empty() {
        token.push(Token::Num(temp.parse().unwrap()));
    }
    //dbg!(&token);

    let mut op_stack = Vec::new();
    let mut output = Vec::new();

    for t in token {
        match t {
            Token::Num(_) => output.push(t),
            Token::Op(op1) => {
                while let Some(Token::Op(op2)) = op_stack.last() {
                    if procedure(&op1) <= procedure(&op2) {
                        output.push(Token::Op(op2.clone()));
                        op_stack.pop();
                    } else {
                        break;
                    }
                }
                op_stack.push(Token::Op(op1));
            }
        }
    }
    while let Some(Token::Op(op2)) = op_stack.last() {
        output.push(Token::Op(op2.clone()));
        op_stack.pop();
    }
    // dbg!(&output);
    // dbg!(&op_stack);

    let mut res = Vec::new();
    for t in output {
        match t {
            Token::Num(num) => res.push(num),
            Token::Op(op) => {
                let b = res.pop().unwrap();
                let a = res.pop().unwrap();
                res.push(cal2(a, b, op));
            }
        }
    }

    res.pop().unwrap()
}
