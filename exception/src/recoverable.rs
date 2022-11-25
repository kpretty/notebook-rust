use std::fs::File;
use std::intrinsics::mul_with_overflow;
use std::io::{Error, ErrorKind, Read};

pub fn main() {
    // e1();
    // e2();
    // e3();
    println!("{:?}", e4());
}

// 使用Result处理可恢复错误
#[allow(dead_code)]
fn e1() {
    // 大多数的错误都是可恢复的，没有必要停止程序
    let result = File::open("hello.txt");
    // 可恢复类型的错误大多使用枚举来处理，rust提供Result枚举类
    // Result<T,E> 若正常则返回E，否则返回E，E封装错误类型
    let file = match result {
        Ok(file) => file,
        Err(e) => panic!("文件不存在，错误信息{}", e)
    };
    println!("{:?}", file)
}

// 匹配不同错误
#[allow(dead_code)]
fn e2() {
    // 大多数的错误都是可恢复的，没有必要停止程序
    let result = File::open("hello.txt");
    // 可恢复类型的错误大多使用枚举来处理，rust提供Result枚举类
    // Result<T,E> 若正常则返回E，否则返回E，E封装错误类型
    let file = match result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("文件不存在,{}", e),
            _ => panic!("其他错误,{}", e)
        }
    };
    println!("{:?}", file)
}

// 对e2的简化
#[allow(dead_code)]
fn e3() {
    // 如果是Ok则返回Ok的值，如果是Err，则执行panic!
    let file = File::open("hello.txt").unwrap();
    // unwrap唯一的缺点是不能自定义异常信息，使用except可以
    let file = File::open("hello.txt").expect("打开文件发生了错误");
    println!("{:?}", file)
}

// 传播错误，即将错误往上抛，让调用者来解决
#[allow(dead_code)]
fn e4() -> Result<String, Error> {
    let result = File::open("hello.txt");
    let mut file = match result {
        Ok(file) => file,
        Err(e) => return Err(e), // 如果发生错误，直接将错误return
    };
    let mut string = String::new();
    // 最后一行 默认返回
    match file.read_to_string(&mut string) {
        Ok(_) => Ok(string),
        Err(e) => Err(e),
    }
}

// 简化e4
#[allow(dead_code)]
fn e5() -> Result<String, Error> {
    // ? 如果Result 是 Ok 则返回 Ok 的值，否则return错误信息
    // 注：? 处理的Err 需要和函数返回值相匹配，否则将会报错，即相当于return一个与函数返回值不匹配的值
    let mut file = File::open("hello.txt")?;
    let mut string = String::new();
    // 最后一行 默认返回
    file.read_to_string(&mut string)?;
    Ok(string)
}

// 简化e5
fn e6() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}