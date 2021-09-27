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
    StringLiteral(String), // 字符串常量
    Operator(Operator), // 操作符号
    Punctuator(Punctuator), // 标点符号
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    // 28个关键字
    Auto,
    Double,
    Int,
    Struct,
    Break,
    Else,
    Long,
    Switch,
    Char,
    Extern,
    Return,
    Union,
    Const,
    Float,
    Short,
    Unsigned,
    Continue,
    For,
    Signed,
    Void,
    Default,
    Goto,
    Sizeof,
    Voiatile,
    Do,
    If,
    Static,
    While,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punctuator {
    // [] () {} , : ;
    OpenParen, // (
    CloseParen, // )
    OpenBracket, // [
    CloseBracket, // ]
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
    BitwiseComplement,  // ~
    BitwiseShiftLeft,   // <<
    BitwiseShiftRight,  // >>
    BitwiseAnd,         // &
    BitwiseOr,          // |
    BitwiseXor,         // ^
    BitwiseNegation,    // !
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
    PlusPlus,           // ++
    MinusMiuns,         // --
}

mod text {
}