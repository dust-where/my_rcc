use super::token::*;


/*
识别模块
实现了将源代码变成了一个类似字符串的东西
*/
pub fn lex(input: &str) -> Vec<Token> {
    // 
    let mut input = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(c) = input.next() {
        match c {
            // c语言中的单字符
            '(' => tokens.push(Token::Punctuator(Punctuator::OpenParen)),
            ')' => tokens.push(Token::Punctuator(Punctuator::CloseParen)),
            '{' => tokens.push(Token::Punctuator(Punctuator::OpenBrace)),
            '}' => tokens.push(Token::Punctuator(Punctuator::CloseBrace)),
            ',' => tokens.push(Token::Punctuator(Punctuator::Comma)),
            ':' => tokens.push(Token::Punctuator(Punctuator::Colon)),
            ';' => tokens.push(Token::Punctuator(Punctuator::Semicolon)),
            '^' => tokens.push(Token::Operator(Operator::BitwiseXor)),
            // 不处理
            // ' ' | '\t' | '\n' | '\r' => {}


            // c语言中的多字符
            '+' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::AssignPlus));
                } else {
                    tokens.push(Token::Operator(Operator::Plus));
                }
            }
            '-' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::AssignMinus));
                } else {
                    tokens.push(Token::Operator(Operator::Minus));
                }
            }
            '*' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::AssignMult));
                } else {
                    tokens.push(Token::Operator(Operator::Multiplication));
                }
            }
            '/' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::AssignDiv));
                } else {
                    tokens.push(Token::Operator(Operator::Division));
                }
            }
            '%' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::AssignMod));
                } else {
                    tokens.push(Token::Operator(Operator::Modulo));
                }
            }
            '!' => {
                if let Some(&'!') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::NotEqual));
                } else {
                    tokens.push(Token::Operator(Operator::LogicalNegation));
                }
            }
            '&' => {
                if let Some(&'&') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::LogicalAnd));
                } else {
                    tokens.push(Token::Operator(Operator::BitwiseAnd));
                }
            }
            '|' => {
                if let Some(&'|') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::LogicalOr));
                } else {
                    tokens.push(Token::Operator(Operator::BitwiseOr));
                }
            }
            '=' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::Equal));
                } else {
                    tokens.push(Token::Operator(Operator::Assignment));
                }
            }
            '<' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::LessThanOrEqual));
                } else if let Some(&'<') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::BitwiseShiftLeft));
                } else {
                    tokens.push(Token::Operator(Operator::LessThan));
                }
            }
            '>' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::GreaterThanOrEqual));
                } else if let Some(&'>') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::BitwiseShiftRight));
                } else {
                    tokens.push(Token::Operator(Operator::GreaterThan));
                }
            }
            // keyword identifier constant stringliteral
            _ => {
                if c.is_alphabetic() {
                    let mut s = c.to_string();

                    loop {
                        match input.peek() {
                            Some(&a) if a.is_alphanumeric() || a == '_' => s.push(a),
                            _ => break,
                        }
                        input.next();
                    }

                    match &s[..] {
                        "int" => tokens.push(Token::Keyword(Keyword::Int)),
                        "break" => tokens.push(Token::Keyword(Keyword::Break)),
                        "else" => tokens.push(Token::Keyword(Keyword::Else)),
                        "return" => tokens.push(Token::Keyword(Keyword::Return)),
                        "continue" => tokens.push(Token::Keyword(Keyword::Continue)),
                        "for" => tokens.push(Token::Keyword(Keyword::For)),
                        "do" => tokens.push(Token::Keyword(Keyword::Do)),
                        "if" => tokens.push(Token::Keyword(Keyword::If)),
                        "while" => tokens.push(Token::Keyword(Keyword::While)),
                        _ => tokens.push(Token::Identifier(s)),
                    }
                } else if c.is_digit(10) {
                    let mut n = c.to_string();

                    loop {
                        match input.peek() {
                            Some(c) if c.is_digit(10) => n.push(*c),
                            _ => break,
                        }
                        input.next();
                    }

                    let n = n.parse::<i32>().unwrap();
                    tokens.push(Token::Constant(n));
                }
            }
        }
    }

    tokens
}