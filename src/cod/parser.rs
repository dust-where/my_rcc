use core::panic;
use std::collections::HashMap;
use std::slice::Iter;

use peek_nth::{IteratorExt, PeekableNth};

use crate::cod::token::Punctuator;

use super::token::*;
use super::ast::*;

type AstMap = HashMap<String, (usize, bool)>;

// 梯度下降
pub fn parser(tokens: &[Token]) -> Ast {
    let mut ast_map = AstMap::new();
    
    let ast = parser_functions(&mut tokens.iter().peekable_nth(), &mut ast_map);

    Ast::Ast(ast)
}

/*
 * 遍历所有内容，
 * 返回一个Vec<AstNode> 也就是Ast
*/
fn parser_functions(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &mut AstMap) -> Vec<AstNode> {
    let mut fun1 = Vec::new();

    while let Some(_) = tokens.peek() {
        let f = parser_function(tokens, ast_map);
        fun1.push(f);
    }
    
    fun1
}

/*
 * 先判断是不是一个函数
 * 如果是则返回AstNode
*/
fn parser_function(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &mut AstMap) -> AstNode {
    match tokens.next() {
        Some(Token::Keyword(Keyword::Int)) => match tokens.next() { // int 
            Some(Token::Identifier(id)) => match tokens.next() { // name main add ...
                Some(Token::Punctuator(Punctuator::OpenParen)) => { // (
                    let params = parser_function_parameters(tokens); // 去获取函数参数
                    let nparams = params.len(); // 有多少参数
                    let has_body = tokens.peek() == Some(&&Token::Punctuator(Punctuator::OpenBrace)); // 是否是 {
                    // 哈希表中是否有当前函数存储
                    if let Some(&(orig_nparams, orig_has_body)) = ast_map.get(id) {
                        if orig_nparams != nparams {
                            // 参数数量不同
                            panic!("Number of parameters in function conflicts with earlier declaration");
                        } else if orig_has_body && has_body {
                            // 是否是前大括号
                            panic!("Redefinition of function");
                        } else {
                            // 哈希表中有同名函数，但内容不同
                            ast_map.insert(id.clone(), (nparams, has_body));
                        }
                    } else {
                        // 哈希表中没有当前内容
                        ast_map.insert(id.clone(), (nparams, has_body));
                    }
                    // 解析函数内部内容
                    let body = match tokens.next() {
                        Some(Token::Punctuator(Punctuator::OpenBrace)) => { // {
                            Some(parser_items(tokens, ast_map))
                        }
                        Some(Token::Punctuator(Punctuator::Semicolon)) => None, // ;
                        // 开头错误
                        _ => panic!("Unexpected token after function declaration"),
                    };
                    // 返回内容：函数名， 函数参数列表， 函数内容的迭代器
                    AstNode::AstNode(id.clone(), params, body)
                }
                // 错误
                e => panic!("Expected opening parenthesis at {:?}", e),
            },
            // Token类型不是Identifier，有可能是用了关键字当名称
            _ => panic!("Expected name for function"),
        },
        // type不是int，而是其他的
        _ => panic!("Expected type for function"),
    }
}

/*
 * 获取函数参数
 * 返回值为以函数参数的名称的数组
*/
fn parser_function_parameters(tokens: &mut PeekableNth<Iter<Token>>) -> Vec<String> {
    let mut params = Vec::new();

    match tokens.peek() {
        // 如果没有参数
        Some(Token::Punctuator(Punctuator::CloseParen)) => { // )
            tokens.next();
            return params;
        },
        // 如果有参数
        Some(_) => {
            let param = parser_next_parameter(tokens);
            params.push(param);

            // 多个参数的处理
            loop {
                match tokens.next() {
                    // 没有多余的参数了
                    Some(Token::Punctuator(Punctuator::CloseParen)) => break,
                    // 有多余的参数
                    Some(Token::Punctuator(Punctuator::Comma)) => { // ,
                        let param = parser_next_parameter(tokens);
                        params.push(param);
                    }
                    // 其他错误情况
                    _ => panic!("Unexpected token in function parameter"),
                }
            }
        }
        // 如果没有token了
        None => panic!("Expected closing parenthesis"),
    }
    // 返回函数参数的字符串
    params
}

/*
 * 多个参数的处理
 * 返回值是函数参数名称
*/
fn parser_next_parameter(tokens: &mut PeekableNth<Iter<Token>>) -> String {
    match tokens.next() {
        Some(Token::Keyword(Keyword::Int)) => match tokens.next() {
            Some(Token::Identifier(id)) => id.clone(),
            // 如果函数int后面没有参数 或者不是参数的时候
            _ => panic!("Expected identifiter for function paramter"),
        },
        // 如果函数参数不是以int开头
        _ => panic!("Expected int keyword for function paramter"),
    }
}

/*
 * 遍历块内元素
 * 遍历到 } 退出 
 * 返回每一条语句的集合
*/
fn parser_items(tokens:&mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Vec<Item> {
    let mut block = Vec::new();
    
    while tokens.peek() != Some(&&Token::Punctuator(Punctuator::CloseBrace)) { // 遍历到 }
        block.push(parser_items_item(tokens, ast_map));
    }

    match tokens.next() {
        Some(Token::Punctuator(Punctuator::CloseBrace)) => block, // }
        _ => panic!("Expected closing braces at end of block {:?}", tokens),
    }
}

/*
 * 在块内的某一段特定语句遍历元素
 * 确认是表达式还是声明
 * 返回这一段语句的item
*/
fn parser_items_item(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Item {
    let item: Item;
    
    match tokens.peek() {
        Some(Token::Keyword(Keyword::Int)) => {
            // 声明
            item = Item::Declaration(parser_declaration(tokens, ast_map));
        },
        Some(_) => {
            // 表达式
            item = Item::Statement(parser_statement(tokens, ast_map));
        },
        None => panic!("Expected block"),
    }

    item
}

/*
 * int ... = expression;
 * 解析声明
*/
fn parser_declaration(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Declaration {
    let declaration: Declaration;
    match tokens.next() {
        Some(Token::Keyword(Keyword::Int)) => match tokens.next() { // int
            Some(Token::Identifier(id)) => { // ...
                if let Some(&&Token::Operator(Operator::Assignment)) = tokens.peek() { // =
                    tokens.next();
                    declaration = Declaration::Declaration (id.clone(), Some(parser_expression(tokens, ast_map)), );
                } else {
                    // 声明不定义: int i;
                    declaration = Declaration::Declaration(id.clone(), None, );
                }
            }
            // 只有int，后面没有变量名
            _ => panic!("Expected identifier"),
        }
        // 我认为这个错误从来不会出现
        _ => panic!("Expected int keyword"),
    }

    match tokens.next() {
        Some(Token::Punctuator(Punctuator::Semicolon)) => declaration,
        _ => panic!("Expected semicolon at teh end of declatation: {:?}", tokens),
    }
}

/*
 * 解析表达式
*/
fn parser_statement(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Statement {
    let statement: Statement;

    match tokens.peek() {
        Some(Token::Keyword(Keyword::Return)) => { // return expersion;
            tokens.next();
            statement = Statement::Return(parser_expression(tokens, ast_map));
        }
        Some(Token::Keyword(Keyword::If)) => { // if 
            tokens.next();
            return parser_if_statement(tokens, ast_map);
        }
        Some(Token::Keyword(Keyword::For)) => { // for
            tokens.next();
            return parser_for_statement(tokens, ast_map);
        }
        Some(Token::Keyword(Keyword::While)) => { // while
            tokens.next();
            return parser_while_statement(tokens, ast_map);
        }
        Some(Token::Keyword(Keyword::Do)) =>{ // do while
            tokens.next();
            statement = parser_do_statement(tokens, ast_map);
        }
        Some(Token::Keyword(Keyword::Break)) => { // break;
            tokens.next();
            statement = Statement::Break;
        }
        Some(Token::Keyword(Keyword::Continue)) => { // continue;
            tokens.next();
            statement = Statement::Continue;
        }
        // 一个新的块
        Some(Token::Punctuator(Punctuator::OpenBrace)) => { // { 
            tokens.next();
            return Statement::Compound(parser_items(tokens, ast_map));
        }
        _ => {
            statement = Statement::Expression(parser_optional_expression(tokens, Punctuator::Semicolon, ast_map));
        }
    }
    
    match tokens.next() {
        Some(Token::Punctuator(Punctuator::Semicolon)) => statement, // ;
        // 没有结束符
        _ => panic!("Expected semicolon at teh end of statement: {:?}", tokens),
    }
}

/*
 * if (expression) {
 *      if_statement
 * } else {
 *      else_statement
 * }
*/
fn parser_if_statement(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Statement {
    match tokens.next() {
        Some(Token::Punctuator(Punctuator::OpenParen)) => { // (
            let expression = parser_expression(tokens, ast_map);
            match tokens.next() {
                Some(Token::Punctuator(Punctuator::CloseParen)) => { // )
                    // if 中的表达式
                    let if_statement = parser_statement(tokens, ast_map);
                    match tokens.peek() {
                        // 有else
                        Some(Token::Keyword(Keyword::Else)) => {
                            tokens.next();
                            let else_statement = parser_statement(tokens, ast_map);
                            Statement::If(expression, Box::new(if_statement), Some(Box::new(else_statement)))
                        }
                        // 无else
                        _ => Statement::If(expression, Box::new(if_statement), None)
                    }
                }
                // 语法错误
                _ => panic!("Expected cloding parenthesis"),
            }
        }
        // 语法错误
        _ => panic!("Expected opening parenthesis"),
    }
}

/*
 * for ( 声明/表达式 init ; 表达式 condition ; 表达式 modifier ) {
 *      body
 * }
 * 这个函数主要是区分第一个参数是声明还是表达式
*/
fn parser_for_statement(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Statement {
    match tokens.next() {
        Some(Token::Punctuator(Punctuator::OpenParen)) => match tokens.peek() { // (
            // 声明的for循环
            Some(Token::Keyword(Keyword::Int)) => {
                let init = parser_declaration(tokens, ast_map);
                let (condition, modifier, body) = parser_for_components(tokens, ast_map);
                Statement::ForDeclaration(init, condition, modifier, Box::new(body))
            }
            // 表达式的for循环
            _ => {
                let init = parser_optional_expression(tokens, Punctuator::Semicolon, ast_map);
                if let Some(Token::Punctuator(Punctuator::Semicolon)) = tokens.peek() {
                    tokens.next();
                } else {
                    // 括号中没有; 语法错误
                    panic!("Expected semicolon after initializer");
                }

                let (condition, modifier, body) = parser_for_components(tokens, ast_map);
                Statement::For(init, condition, modifier, Box::new(body))
            }
        },
        // 语法错误 不是以 ( 开头
        _ => panic!("Expected open parenthesis"),
    }
}

/*
 * 这个函数是处理for循环中的第二个参数，第三个参数和中间的表达式，
*/
fn parser_for_components(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> (Expression, Option<Expression>, Statement) {
    // 分析第二个参数， 有可能参数是以 ， 分隔的表达式
    let condition = match parser_optional_expression(tokens, Punctuator::Semicolon, ast_map) {
        Some(expr) => expr,
        None => Expression::Constant(1),
    };

    let modifier: Option<Expression>;
    let body: Statement;

    match tokens.next() {
        Some(Token::Punctuator(Punctuator::Semicolon)) => {
            // 分析第三个参数
            modifier = parser_optional_expression(tokens, Punctuator::Semicolon, ast_map);
            match tokens.next() {
                // 分析表达式
                Some(Token::Punctuator(Punctuator::CloseParen)) => {
                    body = parser_statement(tokens, ast_map);
                },
                // 语法错误  没有以 ） 结尾
                _ => panic!("Expected close parenthesis"),
            }
        }
        // 语法错误 第三个参数不是表达式等等
        _ => panic!("Expected semicolon after conditional expression"),
    }

    (condition, modifier, body)
}

/*
 * while (expression) {
 *      body
 * }
*/
fn parser_while_statement(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Statement {
    match tokens.next() {
        Some(Token::Punctuator(Punctuator::OpenParen)) => { // (
            let expression = parser_expression(tokens, ast_map);
            match tokens.next() {
                Some(Token::Punctuator(Punctuator::CloseParen)) => { // )
                    let body = parser_statement(tokens, ast_map);
                    Statement::While(expression, Box::new(body))
                }
                _ => panic!("Expected close parenthseis"),
            }
        }
        _ => panic!("Expected open parenthseis"),
    }
}

/*
 * do {
 *  body
 * } while (exxpression)
*/
fn parser_do_statement(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Statement {
    let body = parser_statement(tokens, ast_map);
    match tokens.next() {
        Some(Token::Keyword(Keyword::While)) => match tokens.next() { // while
            Some(Token::Punctuator(Punctuator::OpenParen)) => { // (
                let expression = parser_expression(tokens, ast_map);
                match tokens.next() {
                    Some(Token::Punctuator(Punctuator::CloseParen)) => { // )
                        Statement::DoWhile(expression, Box::new(body))
                    }
                    // 语法错误
                    _ => panic!("Expected close parenthesis"),
                }
            }
            // 语法错误
            _ => panic!("Expected open parenthseis"),
        }
        // 语法错误
        _ => panic!("Expected while keyword"),
    }
}

/*
 * 以expected为分割符返回表达式
*/
fn parser_optional_expression(tokens: &mut PeekableNth<Iter<Token>>, expected: Punctuator, ast_map: &AstMap) -> Option<Expression> {
    match tokens.peek() {
        Some(Token::Punctuator(t)) if t == &expected => None,
        _ => Some(parser_expression(tokens, ast_map)),
    }
}

/*
 * exception => exception + exception 
 * exception => (exception) + operator + (exception)  括号内内容不一定存在
 * 处理
 * expression _ expression
 * 如果中间的符号是 =操作符
 * 优先级14
*/
fn parser_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap, ) -> Expression {
    match tokens.peek() {
        Some(Token::Identifier(id)) => {
            match tokens.peek_nth(1) {
                // 有id 确定是二元运算符 => id  op  expression
               Some(Token::Operator(op)) if op.is_assignment_operators() => { // op 为 赋值操作符
                   tokens.next();
                   tokens.next();
                   Expression::AssignmentOperators(*op, id.clone(), Box::new(parser_expression(tokens, ast_map)))
               }
               // 其他情况
               _ => parser_conditional_expression(tokens, ast_map),
            }
        }
        // 其他情况
        _ => parser_conditional_expression(tokens, ast_map),
    }
}

/*
 * 处理
 * expression _ expression
 * 如果中间的符号是 ?:
 * 优先级13
*/
fn parser_conditional_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut expression = parser_logical_or_expression(tokens, ast_map);

    while let Some(Token::Punctuator(Punctuator::QuestionMark)) = tokens.peek() {
        tokens.next();
        let true_expression = parser_expression(tokens, ast_map);
        match tokens.next() {
            Some(Token::Punctuator(Punctuator::Colon)) => {
                let false_expression = parser_expression(tokens, ast_map);
                expression = Expression::TernaryOperators(Box::new(expression), Box::new(true_expression), Box::new(false_expression));
            }
            // 不符合三目运算符
            _ => panic!("Expected colon"),
        }
    }

    expression
}

/*
 * 处理
 * expression _ expression
 * 如果中间的符号是 ||
 * 优先级12
*/
fn parser_logical_or_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut expression = parser_logical_and_expression(tokens, ast_map);

    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::LogicalOr => { // ||
                tokens.next();
                let next_expression = parser_expression(tokens, ast_map);
                expression = Expression::BinaryOperators(*op, Box::new(expression), Box::new(next_expression))
            }
            _ => break,
        }
    }

    expression
}

/*
 * 处理
 * expression _ expression
 * 如果中间的符号是 &&
 * 优先级11
*/
fn parser_logical_and_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut expression = parser_equality_expression(tokens, ast_map);
    
    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::LogicalAnd => { // &&
                tokens.next();
                let next_expression = parser_expression(tokens, ast_map);
                expression = Expression::BinaryOperators(*op, Box::new(expression), Box::new(next_expression))
            }
            _ => break,
        }
    }

    expression
}

/*
 * 处理
 * expression _ expression
 * 如果中间的符号是 == != 
 * 优先级7
*/
fn parser_equality_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut term = parser_relational_expression(tokens, ast_map);

    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Equal || op == &Operator::NotEqual => { // == !=
                tokens.next();
                let next_trem = parser_expression(tokens, ast_map);
                term = Expression::BinaryOperators(*op, Box::new(term), Box::new(next_trem))
            }
            _ => break,
        }
    }

    term
}

/*
 * 处理
 * expression _ expression
 * 如果中间的符号是 > >= < <=
 * 优先级6
*/
fn parser_relational_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut term = parser_bitwise_expression(tokens, ast_map);

    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::LessThan || op == &Operator::LessThanOrEqual || op == &Operator::GreaterThan || op == &Operator::GreaterThanOrEqual => { // 比较运算符
                tokens.next();
                let next_term = parser_expression(tokens, ast_map);
                term = Expression::BinaryOperators(*op, Box::new(term), Box::new(next_term));
            }
            _ => break,
        }
    }

    term
}

/*
 * 处理
 * expression _ expression
 * 如果中间的符号是 & | << >> ^
 * 优先级5
*/
fn parser_bitwise_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut term = parser_additive_expression(tokens, ast_map);

    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op.is_bitwise_operators() => { // 是位运算
                tokens.next();
                let next_term = parser_expression(tokens, ast_map);
                term = Expression::BinaryOperators(*op, Box::new(term), Box::new(next_term));
            }
            _ => break,
        }
    }

    term
}

/*
 * 处理
 * expression _ expression
 * 如果中间的符号是 + -
 * 优先级4
*/
fn parser_additive_expression(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut term = parser_term(tokens, ast_map);

    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Plus || op == &Operator::Minus => {
                tokens.next();
                let next_term = parser_term(tokens, ast_map);
                term = Expression::BinaryOperators(*op, Box::new(term), Box::new(next_term));
            }
            _ => break,
        }
    }

    term
}

/*
 * 处理
 * expression _ expression 
 * 如果中间的符号是* / %
 * 优先级3
*/
fn parser_term(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression {
    let mut factor = parser_factor(tokens, ast_map);

    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Multiplication || op == &Operator::Division || op == &Operator::Modulo => { // * / % 同一优先级
                tokens.next();
                let next_factor = parser_factor(tokens, ast_map);
                factor = Expression::BinaryOperators(*op, Box::new(factor), Box::new(next_factor));
            }
            _ => break,
        }
    }

    factor
}

/*
 * 处理了函数调用，一元运算符和常量
 * 优先级2
*/
fn parser_factor(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Expression{
    match tokens.next() {
        Some(Token::Punctuator(Punctuator::OpenParen)) => { // (
            let expression = parser_expression(tokens, ast_map);
            if let Some(Token::Punctuator(Punctuator::CloseParen)) = tokens.next() { // )
                return expression;
            } else {
                // 语法错误，没有反括号
                panic!("Expected closing parnthseis");
            }
        }

        // -(expression) ... 例如 -（a - b）
        Some(Token::Operator(op)) if op.is_unary() => { // - ~ ! 
            let factor = parser_factor(tokens, ast_map);
            Expression::UnaryOperators(*op, Box::new(factor))
        }

        // 32
        Some(Token::Constant(c)) => Expression::Constant(*c),

        // 函数调用 + a；
        Some(Token::Identifier(id)) => match tokens.peek() {
            // 函数调用
            Some(Token::Punctuator(Punctuator::OpenParen)) => { // (
                tokens.next();
                let args = parser_function_call(tokens, ast_map);
                if let Some(&(expected_nargs, _)) = ast_map.get(id) {
                    if args.len() == expected_nargs {
                        // 函数参数数量和输入数量一样
                        Expression::FunctionCalls(id.clone(), args)
                    } else {
                        // 不一样
                        panic!("Wrong number of arguments");
                    }
                } else {
                    // 函数未定义
                    panic!("Undeclared function: {}", id);
                }
            }
            // a;
            _ => Expression::Variable(id.clone()),
        },
        //  穷举法没有发现的表达式
        _ => panic!("Unexpected token"),
    }
}

/*
 * 处理函数调用表达式
 * 优先级1 ： （）
*/
fn parser_function_call(tokens: &mut PeekableNth<Iter<Token>>, ast_map: &AstMap) -> Vec<Expression> {
    let mut args = Vec::new();
    match tokens.peek() {
        Some(Token::Punctuator(Punctuator::CloseParen)) => { // ) 表示使用无参数方法
            tokens.next();
            return args;
        }
        Some(_) => {
            let arg = parser_expression(tokens, ast_map); // 函数参数可能是新的表达式
            args.push(arg);

            loop {
                match tokens.next() {
                    Some(Token::Punctuator(Punctuator::CloseParen)) => break,
                    Some(Token::Punctuator(Punctuator::Comma)) => { //  ， 多个参数
                        let arg = parser_expression(tokens, ast_map);
                        args.push(arg);
                    }
                    _ => panic!("Unexpected token in function argument"),
                }
            }
        }
        None => panic!("Expected closing parenthesis"),
    }
    
    args
}


// 优先级	     运算符	                                                                        结合律
// 1   	        []    ()    ·    ->  	                                                       从左到右
// 2	        ++    --    !    ~    +    -    *    &    sizeof    (类型)                     从右到左
// 3	        *    /    %	                                                                   从左到右
// 4	        +    -	                                                                       从左到右
// 5	        <<    >>	 &  ^   !                                                          从左到右
// 6	        >   >=  <   <=	                                                               从左到右
// 7	        ==    !=	                                                                   从左到右
// 8	        &	                                                                           从左到右
// 9	        ^	                                                                           从左到右
// 10	        |	                                                                           从左到右
// 11	        &&	                                                                           从左到右
// 12	        ||	                                                                           从左到右
// 13	        ?:	                                                                           从右到左
// 14	        =         +=        -=       *=       /=      %=       &=       ^=      |=     <<=      >>=	
//                                                                                             从右到左
// 15	        ，	                                                                           从左到右