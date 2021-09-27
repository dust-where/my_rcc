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
            '[' => tokens.push(Token::Punctuator(Punctuator::OpenBracket)),
            ']' => tokens.push(Token::Punctuator(Punctuator::CloseBracket)),
            '{' => tokens.push(Token::Punctuator(Punctuator::OpenBrace)),
            '}' => tokens.push(Token::Punctuator(Punctuator::CloseBrace)),
            ',' => tokens.push(Token::Punctuator(Punctuator::Comma)),
            ':' => tokens.push(Token::Punctuator(Punctuator::Colon)),
            ';' => tokens.push(Token::Punctuator(Punctuator::Semicolon)),
            '^' => tokens.push(Token::Operator(Operator::BitwiseXor)),
            '~' => tokens.push(Token::Operator(Operator::BitwiseComplement)),
            // 不处理
            // ' ' | '\t' | '\n' | '\r' => {}


            // c语言中的多字符
            '+' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::AssignPlus));
                } else if let Some(&'+') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::PlusPlus));
                } else {
                    tokens.push(Token::Operator(Operator::Plus));
                }
            }
            '-' => {
                if let Some(&'=') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::AssignMinus));
                } else if let Some(&'-') = input.peek() {
                    input.next();
                    tokens.push(Token::Operator(Operator::MinusMiuns));
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
                    tokens.push(Token::Operator(Operator::BitwiseNegation));
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
            
        }
    }

    tokens
}