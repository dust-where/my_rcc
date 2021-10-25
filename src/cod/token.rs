#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // C语言中有六类
    Keyword(Keyword),  // 关键字
    Identifier(String), // 标识符
    // 标识符的处理很简单 ，看见是什么就是什么
    /*
    0~9
    a~z
    A~Z
    */
    Constant(i32), // 常量
    Operator(Operator), // 操作符号
    Punctuator(Punctuator), // 标点符号
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Int,
    Break,
    Else,
    Return,
    Continue,
    For,
    Do,
    If,
    While,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punctuator {
    // [] () {} , : ;
    QuestionMark, // ?
    OpenParen, // (
    CloseParen, // )
    OpenBrace, // {
    CloseBrace, // }
    Comma, // ,
    Colon, // :
    Semicolon, // ;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,               // +
    Minus,              // -
    Multiplication,     // *
    Division,           // /
    Modulo,             // %
    
    BitwiseShiftLeft,   // <<
    BitwiseShiftRight,  // >>
    BitwiseAnd,         // &
    BitwiseOr,          // |
    BitwiseXor,         // ^

    LogicalNegation,    // !
    LogicalAnd,         // &&
    LogicalOr,          // ||

    Equal,              // ==
    NotEqual,           // !=
    
    LessThan,           // <
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    GreaterThanOrEqual, // >=

    Assignment,         // =
    AssignPlus,         // +=
    AssignMinus,        // -=
    AssignMult,         // *=
    AssignDiv,          // /=
    AssignMod,          // %=
}

impl Operator {
    pub fn is_unary(self) -> bool { // - ! ~
        match self {
            Operator::Minus |
            Operator::LogicalNegation => true,
            _ => false,
        }
    }
 
    pub fn is_bitwise_operators(self) -> bool { // << >> & | ^
        match self {
            Operator::BitwiseShiftLeft |
            Operator::BitwiseShiftRight |
            Operator::BitwiseAnd |
            Operator::BitwiseOr | 
            Operator::BitwiseXor => true,
            _ => false,
        }
    }

    pub fn is_assignment_operators(self) -> bool { // = += -= *= /= %=
        match self {
            Operator::Assignment
            | Operator::AssignPlus
            | Operator::AssignMinus
            | Operator::AssignMult
            | Operator::AssignDiv
            | Operator::AssignMod => true,
            _ => false,
        }
    }
    
}