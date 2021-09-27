use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::process::exit;

mod cod;


fn main() {
    // 首先获取命令行的参数
    // 参考代码：https://www.perfcode.com/p/rust-gets-command-line-parameters.html
    
    // 假设输入的是： rustc/cargo run (需要运行的c文件目录)
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);

    // 如果输入的参数有问题 报错并退出
    if args.len() != 2 {
        eprintln!("Error: Some question occur in input");
        exit(1);
    }
    // println!("{}", &args[1]);
    let tokens = match read_file(&args[1]) {
        
        // 在这里将所有的字符串进行lex
        Ok(s) => cod::lex::lex(&s),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };
}

fn read_file(input: &str) -> Result<String, Error> {
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    Ok(contents)
}