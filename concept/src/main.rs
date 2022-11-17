fn main() {
    // variable();
    // datatype();
    // function(1, "2");
    println!("{}!={}", 3, factorial(3));
    fibonacci(10);
}

#[allow(dead_code)]
// 变量与常量
fn variable() {
    // 默认为不可变类型
    let x = 5;
    println!("The value of x is : {x}");
    // 下面代码将报错
    // x = 6
    // 使用 mut 定义为可变
    let mut y = 5;
    println!("The value of y is : {y}");
    y = 6;
    println!("The value of y is : {y}");
    // 常量
    // 不允许对常量使用 mut。常量不光默认不能变，它总是不能变
    // 只能被设置成常量表达式，不可以是其他任何只能在运行时计算出的值
    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is : {}", THREE_HOURS_IN_SECONDS);
    // 隐藏shadowing
    // 与 mut 的区别在于 mut 修饰的变量只能改变值不能改变类型，且前后是同一个变量
    // 使用隐藏后，本质上是重新创建了一个同名变量并覆盖上一个变量，因此可以改变数据类型
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is : {}", spaces)
}


#[allow(dead_code)]
/// # 整型
/// | 长度   | 有符号 | 无符号 |
/// | ------ | ------ | ------ |
/// | 8bit   | i8     | u8     |
/// | 16bit  | i16    | u16    |
/// | 32bit  | i32    | u32    |
/// | 64bit  | i64    | u64    |
/// | 128bit | i128   | u128   |
/// | arch   | isize  | usize  |
/// isize 和 usize 依赖计算机架构来断定自己是32位还是64位，主要作为某些集合的索引<br>
/// 整型溢出：在debug阶段rust会直接报错，在release阶段会采用二进制补码回绕的方式(经测试当前版本release也编译不通过)<br>
/// 例如：u8的最大值为255，若存256则回绕成0<br>
/// # 浮点数
/// f32 f64
/// # 布尔型
/// true false
/// # 字符型
/// char(支持emoji)
/// # 复合型
/// ## 元组
/// (a,b,c)
/// ## 数组
/// \[a,b,c\]
///
// 数据类型
fn datatype() {
    // rust 是静态类型语言，编译器往往都是可以推断出，但存在多种可能的数据类型时则需要手动指定<br>
    // 例如：字符串转数字，下面代码会报错
    // let guess = "123".parse().expect("Not a number");
    // 必须指定数据类型
    let guess: u32 = "123".parse().expect("Not a number");
    println!("{}", guess);
    // 整型
    let num: u8 = 255;
    println!("{}", num);
    // 浮点型
    let f1: f32 = 3.14;
    let f2: f64 = 3.14;
    println!("f1:{},f2:{}", f1, f2);
    // 布尔值
    let ok: bool = true;
    let err: bool = false;
    println!("are you {ok}? no i'm {err}");
    // 字符型
    let c: char = '🎉';
    println!("恭喜恭喜 {}{}{}", c, c, c);
    // 元组，长度不可变
    let tuple: (i8, char, &str) = (1, '2', "3");
    // 元组裂变
    let (a, b, c) = tuple;
    // 两种方式访问元组
    println!("a:{},b:{},c:{}", a, b, c);
    println!("a:{},b:{},c:{}", tuple.0, tuple.1, tuple.2);
    // 数组，长度不可变且元素类型必须保持一致
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for ele in arr {
        print!("{} ", ele)
    }
    println!("{:?}", arr);
}

#[allow(dead_code)]
fn function(param1: i32, param2: &str) -> bool {
    println!("param1:{},param2:{}", param1, param2);
    // 代码块也可以有返回值
    let result = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("result:{}", result);
    // 可以省略return
    //true;
    return true;
}


#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return n * factorial(n - 1);
}

fn fibonacci(n: usize) {
    if n == 1 {
        print!("{} ", 0);
        return;
    }
    let mut tmp1 = 0;
    let mut tmp2 = 1;
    let mut tmp: i32;
    print!("{} {} ", tmp1, tmp2);
    for _ in 2..n {
        tmp = tmp1 + tmp2;
        print!("{} ", tmp);
        // 交换
        tmp1 = tmp2;
        tmp2 = tmp;
    }
}