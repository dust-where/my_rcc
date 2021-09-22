#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // C语言中有六类
    Keyword(Keyword),  // 关键字
    Identifier(String), // 标识符
    Constant(i32), // 常量
    String_literal(String), // 字符串常量
    Operator(Operator), // 操作符和
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
    // [] () {} * , : = ;
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    
}

mod text {
    use token;

    
}