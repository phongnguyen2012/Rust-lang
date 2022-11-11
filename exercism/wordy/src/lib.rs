pub struct WordProblem;
#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Digit(i32),
}
pub fn answer(command: &str) -> Option<i32> {
    translate_to_math_expr(command)
        .and_then(parse_tokens)
        .and_then(operator_eq_precedence_arith)
        .ok()
}
fn parse_tokens(problem: String) -> Result<Vec<Token>, String> {
    problem
        .split(' ')
        .map(parse_token)
        .collect::<Result<Vec<Token>, String>>()
}
fn operator_eq_precedence_arith(tokens: Vec<Token>) -> Result<i32, String> {
    let tokens_len = tokens.len();
    let mut op_stack = vec![];
    let mut val_stack: Vec<i32> = vec![];
    for (i, token) in tokens.into_iter().enumerate() {
        //reject prefix notation
        if !op_stack.is_empty() && val_stack.is_empty() {
            return Err("Prefix notation not allowed".to_string());
        }
        //reject postfix notation
        if op_stack.is_empty() && val_stack.len() == 2 {
            return Err("Postfix notation not allowed".to_string());
        }
        match token {
            Token::Digit(b) if !op_stack.is_empty() || i == tokens_len => {
                let op = op_stack.pop().unwrap();
                let a = val_stack.pop().unwrap();
                match op {
                    Token::Add => val_stack.push(a + b),
                    Token::Subtract => val_stack.push(a - b),
                    Token::Multiply => val_stack.push(a * b),
                    Token::Divide => val_stack.push(a / b),
                    Token::Power => val_stack.push(a.pow(b as u32)),
                    _ => unreachable!(),
                }
            }
            Token::Digit(b) => val_stack.push(b),
            t => op_stack.push(t),
        }
    }
    if op_stack.is_empty() && val_stack.len() == 1 {
        val_stack
            .first()
            .cloned()
            .ok_or("Invalid expression".to_string())
    } else {
        Err("Invalid expression".into())
    }
}
fn translate_to_math_expr(command: &str) -> Result<String, String> {
    let expr = command
        .replace("What is", "")
        .trim()
        .replace("plus", "+")
        .replace("minus", "-")
        .replace("multiplied by", "*")
        .replace("divided by", "/")
        .replace('?', "");
    //translate single power expr
    if let Some(i) = expr.find("raised to the") {
        let (prefix, pow) = expr.split_at(i);
        let (pow, rest) = pow.split_once(" power").unwrap();
        let pow = pow
            .replace("raised to the", "^")
            .replace("th", "")
            .replace("nd", "");
        return Ok(format!("{}{}{}", prefix, pow, rest));
    }
    Ok(expr)
}
fn parse_token(x: &str) -> Result<Token, String> {
    let token = match x {
        "+" => Token::Add,
        "-" => Token::Subtract,
        "*" => Token::Multiply,
        "/" => Token::Divide,
        "^" => Token::Power,
        _ => Token::Digit(x.parse::<i32>().map_err(|e| e.to_string())?),
    };
    Ok(token)
}
