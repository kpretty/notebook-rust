use std::cmp::Ordering;
// std 为标准库，io为输入/输出库，使用use引入
// rust 也存在一些预导入的库成为 preclude
use std::io;
use rand::Rng;

// 程序入口，无参数无返回值
fn main() {
    // 带 ! 为宏，表明调用的是宏而并非普通函数，且宏并不总是遵循函数相同的规则
    println!("Guess the number!");
    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your number:");
        // 默认创建的变量为不可变的 let apples，使用关键字mut声明可变的变量
        // :: 表明new是String的关联函数(java中成为静态方法)，关联函数是针对某个类型来说的而不是某个类型具体的实例
        // 因此下面语句的含义是创建一个可变变量并绑定到一个String的空实例上
        let mut guess = String::new();
        // 调用 read_line 将 &mut guess 作为参数传入函数中，让用户输入的值保存在guess变量中
        // read_line 将内容追加到字符串中，因此要求字符串参数可变
        // read_line 返回值为Result，是一个枚举这里用来编码错误处理信息，有两个值OK，Err
        // 没有expect程序也能运行(警告)，expect本质是对Result的值进行匹配
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // 去读一行会包含回车符
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("Illegal number, re-enter");
                continue;
            }
        };
        println!("You guess: {guess}");
        // 验证是否是追加
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        // guess = guess.trim().to_string();
        // println!("You guess: {guess}");
        // 处理比较两个数字可能的返回值
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
