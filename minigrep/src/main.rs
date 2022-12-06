use std::{env, process};
use minigrep::Config;

fn main() {
    // args 返回 string 类型，无法接受非法unicode字符
    // args_os 返回 OsString 类型，可以解决
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


