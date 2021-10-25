use super::ast::*;
use super::context::Context;
use super::token::*;

static mut COUNTER: u32 = 0;

/*
 * 创建唯一数
*/
fn generate_suffix() -> String {
    let n: u32; 
    unsafe {
        n = COUNTER;
        COUNTER += 1;
    }
    n.to_string()
}

/*
 * 创建带_的唯一后缀
*/
pub fn unique_suffix() -> String {
    let count: String = generate_suffix();
    let mut suffix = "_".to_string();
    suffix.push_str(&count);
    suffix
}

/*
 * 给word添加唯一后缀
*/
pub fn add_suffix(word: &str, suffix: &str) -> String {
    let mut word = word.to_string();
    word.push_str(suffix);
    word
}

/*
 * 层级遍历 
 * Ast->AstNodes
*/
pub fn generate(ast: &Ast) {
    println!(".inter_syntax noprefix");

    match ast {
        Ast::Ast(asts) => {
            generate_astnodes(asts);
        }
    }
}

/*
 * 层级遍历 
 * AstNodes->AstNode
*/
fn generate_astnodes(asts: &Vec<AstNode>) {
    asts.iter().for_each(|AstNode::AstNode(name,params, body) | {
        generate_astnode(name, params, body)
    });
}

/*
 * 层级遍历 
 * AstNode->AstNode
*/
fn generate_astnode(name: &str, params: &[String], body: &Option<Vec<Item>>) {
    if let Some(item) = body {
        println!(".global {}", name);
        println!("{}", name);

        println!("  push rbp");
        println!("  mov rbp,rsp");

        let context = Context::new(params);

        generate_item(item, &context);

        // 函数结束
        println!("  mov rax,0");
        generate_function_end();

    }
}

/*
 * 层级遍历 
 * Item->Declaration + statement
*/
fn generate_item(item: &[Item], context: &Context) {
    let mut context = context.clone();

    for each_item in item {
        match each_item {
            // 声明
            Item::Declaration(declaration) => {
                generate_declaration(declaration, &mut context);
            }
            // 表达式
            Item::Statement(statement) => {
                generate_statement(statement, &mut context);
            }
        }
    }
}

/*
 * 处理声明
*/
fn generate_declaration(decleration: &Declaration, context: &mut Context) {
    match decleration {
        Declaration::Declaration(name, expressione) => {
            if context.current_scope.contains(name) { // 查看变量集中是否有这个变量
                // 语法错误 变量出现了两次
                panic!("Variable {} declared twice in same scope", name);
            }
            
            if let Some(expr) = expressione {
                // expression 存在，有返回值，处理expresssion
                generate_expression(expr, &context);
                println!("  push rax");
            } else {
                // expression 不存在，也就是没有返回值
                println!("  push 0");
            }

            context.var_map.insert(name.clone(), context.stack_index);
            context.current_scope.insert(name.clone());
            context.stack_index -= 8;
        },
    }
}

/*
 * 表达式的处理
*/
fn generate_statement(statement: &Statement, context: &Context) {
    let context = context.reset_scope();
    match statement {
        Statement::Expression(expr) => {
            // 非空
            if let Some(e) = expr {
                generate_expression(e, &context);
            }
        },

        Statement::Return(expr) => {
            generate_expression(expr, &context);
            // 函数结束
            generate_function_end();
        },

        // 这里进行了修改
        Statement::If(expr, if_body, else_body) => {
            let suffix = unique_suffix();
            let post_if_label = add_suffix("post_if", &suffix);
            let post_else_label = add_suffix("post_else", &suffix);

            generate_expression(expr, &context);
            println!("  cmp rax,0");
            println!("je {}", post_if_label);
            

            // 有没有else
            if let Some(else_statement) = else_body {
                println!("  jmp {}", post_else_label);
                println!("{}:", post_if_label);
                generate_statement(if_body, &context);
                println!("{}:", post_else_label);
                generate_statement(else_statement, &context);
            } else {
                println!("{}:", post_if_label);
                generate_statement(if_body, &context);
            }

            println!("{}:", post_if_label);
            generate_statement(if_body, &context);
            
        },

        Statement::Compound(item) => {
            generate_item(item, &context);
        },

        Statement::For(init, condition, post_expression, body) => {
            // 有没有表达式
            if let Some(expr) = init {
                generate_expression(expr, &context);
            }

            // 
            for_loop(condition, post_expression, body, &context);
            println!("  pop rax");
        },
        
        Statement::ForDeclaration(decl, condition, post_expression, body) => {
            // 处理声明
            let mut context = context.clone();
            generate_declaration(decl, &mut context);

            // 
            for_loop(condition, post_expression, body, &context);
            println!("  pop rax");
        },

        Statement::While(condition, body) => {
            for_loop(condition, &None, body, &context);
        },

        Statement::DoWhile(condition, body) => {
            let suffix = unique_suffix();
            let loop_label = add_suffix("loop", &suffix);
            let break_label = add_suffix("post_loop", &suffix);
            let continue_label = add_suffix("continue_do_while", &suffix);

            println!("{}:", loop_label);

            let body_context = Context {break_label: Some(break_label.clone()), continue_label: Some(continue_label.clone()), ..context.clone()};
            
            generate_statement(body, &body_context);
            
            generate_expression(condition, &context);
            
            println!("  cmp rax,0");
            println!("  jne {}", loop_label);
            println!("{}:", break_label);
            
        },

        Statement::Break => match context.break_label {
            // 跳转到原来位置
            Some(label) => println!("   jmp {}", label),
            // 语法错误，没有break
            None => panic!("Break statement not in loop"),
        },

        Statement::Continue => match context.continue_label {
            // 跳转到原来位置
            Some(label) => println!("   jmp {}", label),
            // 语法错误，没有continue
            None => panic!("Continue statement not in loop"),
        },
    }
}

fn generate_expression(expression: &Expression, context: &Context) {
    match expression {
        Expression::Constant(n) => {
            println!("  mov rax,{}", n);
        },

        Expression::Variable(name) => {
            if !context.var_map.contains_key(name) {
                panic!("Variable undeclared");
            } else {
                let offset: isize = *context.var_map.get(name).expect("Missing offset");
                println!("  mov rax, [rbp{:+}]", offset);
            }
        },

        Expression::UnaryOperators(op, expr) => {
            generate_expression(expr, context);

            match op {
                Operator::Minus => { // 非
                    println!("  neg rax");
                }
                // TODO:我认为他写的有问题
                Operator::LogicalNegation => { // ~ 二进制取反
                    // println!("  mov rdi,0");
                    // println!("  cmp rdi,rax");
                    // println!("  sete al");
                    // println!("  movzb rax,al");
                }
                _ => panic!("Unexprected unary operator"),
            }
        },

        Expression::AssignmentOperators(op, name, expr) => {
            generate_expression(expr, context);

            // 是否存储这个变量
            if !context.var_map.contains_key(name) {
                panic!("Variable undeclared");
            } else {
                let offset: isize = *context.var_map.get(name).expect("Missing offset");
                match op {
                    // rax 是expr的返回值

                    // 计算+存储
                    Operator::Assignment => {
                        println!("  mov [rbp{:+}], rax", offset);
                    },
                    Operator::AssignPlus => {
                        println!("  add [rbp{:+}], rax", offset);
                        println!("  mov [rbp{:+}], rax", offset);
                    },
                    Operator::AssignMinus => {
                        println!("  sub [rbp{:+}], rax", offset);
                        println!("  mov [rbp{:+}], rax", offset);
                    },
                    Operator::AssignMult => {
                        println!("  mov rdi, rax");
                        println!("  mov rax, [rbp{:+}]", offset);
                        println!("  mul rdi");
                        println!("  mov [rbp{:+}], rax", offset);
                    }
                    Operator::AssignDiv => {
                        println!("  mov rdi, rax");
                        println!("  mov rax, [rbp{:+}]", offset);
                        println!("  mov rdx, 0");
                        println!("  div rdi");
                        println!("  mov [rbp{:+}], rax", offset);
                    }
                    Operator::AssignMod => {
                        println!("  mov rdi, rax");
                        println!("  mov rax, [rbp{:+}]", offset);
                        println!("  mov rdx, 0");
                        println!("  div rdi");
                        println!("  mov [rbp{:+}], rdx", offset);
                    }


                    _ => panic!("Unexpected assignment operator"),
                }
            }
        },

        Expression::BinaryOperators(op, lhs, rhs) => {
            // 这里仅仅只需要计算

            generate_expression(rhs, context);
            println!("  push rax");
            generate_expression(lhs, context);
            println!("  pop rdi");
            // rax是lhs rdi是rhs

            match op {
                Operator::Plus => {
                    println!("  add rax,rdi");
                },
                Operator::Minus => {
                    println!("  sub rax,rdi");
                },
                Operator::Multiplication => {
                    println!("  mul rdi");
                },
                Operator::Division => {
                    println!("  mov rdx,0");
                    println!("  div rdi");
                },
                Operator::Modulo => {
                    println!("  mov rdx,0");
                    println!("  div rdi");
                    println!("  mov rax,rdx");
                },
                Operator::Equal => {
                    println!("  sete al");
                },
                Operator::NotEqual => {
                    println!("  setne al");
                },
                Operator::LessThan => {
                    println!("  setl al");
                },
                Operator::LessThanOrEqual => {
                    println!("  setle al");
                }
                Operator::GreaterThan => {
                    println!("  setg al");
                },
                Operator::GreaterThanOrEqual => {
                    println!("  setge al");
                },
                // 位运算要补充位数
                Operator::LogicalOr => {
                    println!("  or rdi,rax");
                    println!("  setne al");
                    println!("  movzb rax,al");
                },
                Operator::LogicalAnd => {
                    println!("  cmp rdi,0");
                    println!("  setne dil");
                    println!("  cmp rax,0");
                    println!("  setne al");
                    println!("  movzb rax,al");
                    println!("  movzb rax,al");
                    println!("  and al,dil");
                },

                Operator::BitwiseAnd => {
                    println!("  and rax,rdi");
                },
                Operator::BitwiseOr => {
                    println!("  or rax,rdi");
                },
                Operator::BitwiseXor => {
                    println!("  xor rax,rdi");
                },
                Operator::BitwiseShiftLeft => {
                    println!("  mov rcx,rdi");
                    println!("  shl rax,cl");
                },
                Operator::BitwiseShiftRight => {
                    println!("  mov rcx,rdi");
                    println!("  shr rax,cl");
                }
                _ => panic!("Unexprected binary operator"),
            }
        },

        Expression::TernaryOperators(e1, e2, e3) => {
            generate_expression(e1, context);
            println!("  cmp rax, 0");

            let suffix = unique_suffix();
            let e_label = add_suffix("e", &suffix);
            let e_conditional_label = add_suffix("e_conditional", &suffix);

            println!("  je {}", e_label); // 跳转e
            generate_expression(e2, context);

            println!("  jmp {}", e_conditional_label); // 跳转e_conditional

            println!("{}:", e_label);
            generate_expression(e3, context);

            println!("{}:", e_conditional_label);
        },
        
        Expression::FunctionCalls(id, args) => {
            let arg_len = args.len();

            args.iter().rev().for_each(|e| {
                generate_expression(e, context);
                println!("  push rax");
            });

            println!("  call {}", id);
            println!("  add rsp, {}", arg_len * 8); // 分配空间
        },
    }
}

fn for_loop(condition: &Expression, post_expression: &Option<Expression>, body: &Statement, context: &Context) {
    let suffix = unique_suffix();
    let loop_label = add_suffix("loop", &suffix);
    let post_loop_label = add_suffix("post_loop", &suffix);
    let continue_label = add_suffix("loop_continue", &suffix);

    println!("{}:", loop_label);
    generate_expression(condition, context);
    println!("  cmp rax,0");
    println!("  je {}", post_loop_label);

    let body_context = Context {
        break_label: Some(post_loop_label.clone()),
        continue_label: Some(continue_label.clone()),
        ..context.clone()
    };

    generate_statement(body, &body_context);

    println!("{}:", continue_label);

    if let Some(expr) = post_expression {
        generate_expression(expr, &context);
    }

    println!("  jmp {}", loop_label);
    println!("{}:", post_loop_label);
}

/*
 * 结束添加
*/
fn generate_function_end() {
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

// 本解析器基于inter语法的x86_64
// 用到的汇编代码解析
// .inter_syntax noprefix 代表inter语法的x86
// .global 声明变量是全局可见的
// push 压入栈
// pop 出栈
// mov a,b 将b中内容存入a
// ret 返回call之前的地址
// rbp,rsp 寄存器指针
// rsp 是堆栈指针寄存器，通常会指向栈顶位置,push与pop都是通过改变rsp来实现栈指针移动
// rbp 是栈帧指针，指示栈的初始位置
// rax 是函数的返回值
// je 相等跳转
// jne 不相等跳转
// neg 非
// sete 相等时设置 setne setl setg setle setge
// add + sub - 
// movzb 拷贝的时候会补充0或1
// shl shr 左移右移
// call 调用


/*
 * 某些特定的结构：
 * 
 * if-else
 *      cmp ...         判断
 *      je L1           相等
 *      jmp L2          不相等
 * L1:  something
 * L2:  something
 * 
 * while
 *      mov eax,val1
 * beginwhile:
 *      cmp eax,val2    判断
 *      jnl endwhile    跳转
 *      something
 *      jmp beginwhile
 * endwhile:
 *      something
 * 
 * for 和 while 差不多 就多执行一条语句而已
 * gebinfor:
 *      cmp 判断
 *      je L1
 *      jmp L2
 * L1：、、、
 * L2：outher
*/