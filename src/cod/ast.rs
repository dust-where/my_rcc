use super::token::*;
// 抽象语法树

// 源代码中只有函数
/*
 * 用Vec来存储所有的函数，每个函数占有一个node
*/
#[derive(Debug)]
pub enum Ast { // 直接将语法树变成dag(有向无环图)
    Ast(Vec<AstNode>)
}

/*
 * 函数名
 * 参数
 * 函数内元素
*/
#[derive(Debug)]
pub enum AstNode {
    AstNode(String, Vec<String>, Option<Vec<Item>>),
}

/*
 * 声明
 * 表达式
*/
#[derive(Debug)]
pub enum Item {
    Declaration(Declaration),
    Statement(Statement),
}

/*
 * 名称
 * 值
*/
#[derive(Debug)]
pub enum Declaration {
    Declaration(String, Option<Expression>)
}

/*
 * 表达式语句
*/
#[derive(Debug)]
pub enum Statement {
    Expression(Option<Expression>), // 表达式语句可能不存在
    Return(Expression), // return exp
    If(Expression, Box<Statement>, Option<Box<Statement>>), // if
    Compound(Vec<Item>), // += ...
    For(Option<Expression>, Expression, Option<Expression>, Box<Statement>), // for
    // 这里如果for没有第一个参数，默认按照第一个处理
    ForDeclaration(Declaration, Expression, Option<Expression>, Box<Statement>), // for 
    While(Expression, Box<Statement>), // while
    DoWhile(Expression, Box<Statement>), // do while
    Break,
    Continue,
}

/*
 * 
*/
#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    Variable(String),
    UnaryOperators(Operator, Box<Expression>), // |a
    AssignmentOperators(Operator, String, Box<Expression>), // ___  __ ___ a = b
    BinaryOperators(Operator, Box<Expression>, Box<Expression>), // __ __ __ a + b
    TernaryOperators(Box<Expression>, Box<Expression>, Box<Expression>), // ?:
    FunctionCalls(String, Vec<Expression>), // 函数调用
}