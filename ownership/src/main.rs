fn main() {
    // ownership_1();
    // ownership_2();
    // ownership_3();
    // ownership_4();
    // ownership_5();
    // ownership_6();
    // ownership_7();
    let s = String::from("hello world");

    println!("{}", first_word(&s));
}

#[allow(dead_code)]
fn ownership_1() {
    {
        // s 在这一行无效，因为尚未声明
        let s = 1; // 从此处起开始有效

        println!("{:?}", s);
        // 使用 s
    } // 此作用域已经结束，s 不再生效
}

#[allow(dead_code)]
fn ownership_2() {
    // 字符串字面量是固定大小不可变的，可以存储在编译前就已经知道其大小的数据
    // 但更多的字符串在运行时才可以知道长度，例如接收用户的输入，因此需要可边长的字符串数据类型
    // todo 为什么要加 mut
    // 因为push_str是修改堆内存的数据，所以需要可变的引用
    let mut s = String::from("hello");
    // s 为String类型，可变
    s.push_str(" world");
    println!("new str: {}", s);
}

// 变量与数据的交互-移交
#[allow(dead_code)]
fn ownership_3() {
    let x = 5;
    let y = x;
    println!("x:{},y:{}", x, y);
    // 下面的代码将是报错的
    // 对于rust而言 运行到 let s1 = s; 根据值的所有权原则，"hello"的所有权已经移交到s1，可以理解为s此时已经被回收了
    // 如果 s 和 s1 都持有值的所有权会存在double free的bug，当s和s1离开作用域时相同地址的堆内存会被释放两次
    // 注：s存储在栈中，"hello"存储在堆中，s1 = s;做的操作只是在栈中拷贝了s，堆中数据没有被拷贝，即其他语言中的浅拷贝
    // rust永远不会主动地进行深拷贝，因为拷贝堆中数据会影响效率
    // let s = String::from("hello");
    // let s1 = s;
    // println!("s:{},s1:{}", s, s1);
}

// 变量与数据的交互-克隆
#[allow(dead_code)]
fn ownership_4() {
    // rust也提供了深拷贝的功能
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1:{},s2:{}", s1, s2);
    // 为什么 let x = 5;
    //       let y = x;
    //       println!("x:{},y:{}", x, y);
    // 不会发生报错，因为这些被存储在栈中的数据拷贝是快速地，换句话说就是没有深拷贝浅拷贝的概念
    // 注：在rust中凡是实现了Copy trait的类型在旧变量赋值给其他变量后依然可以使用
    // 同时 rust 不允许实现了 Drop 的类型再去实现 Copy，那么哪些类型实现了Copy
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。
}

// 所有权和函数
#[allow(dead_code)]
fn ownership_5() {
    // 将值传递给函数也会发生和变量赋值一样的操作，即：移交或者复制
    let s = String::from("hello");
    takes_ownership(s);
    // 下面的代码会报错，因为此时 s 已经将 "hello" 的所有权移交给 some_string，同时因为函数调用结束 some_string 已经离开了作用于
    // println!("s:{}", s);
    let n = 5;
    makes_copy(n);
    // i32 存在栈中
    println!("n:{}", n);
    // 做一次shadowing
    let s = String::from("hello");
    // 函数的返回值也会造成所有权的移交
    // 这里hello所有权的移交情况如下：
    // s 因为作为函数的传参所有权移交给 some_string
    // some_string 通过返回值返回被s1接收移交给s1，且some_string离开了作用于
    // 最终 hello 的所有权在s1
    let s1 = takes_ownership_1(s);
    println!("s1:{}", s1);
    // 如果想要实现值作为函数传参且变量不丢失值的所有权，可以将所有权通过返回值返回在使用原变量接收
    let (s1, len) = calculate_length(s1);
    // 看起来所有权还是s1的，但所有权的移交还是发生了，且借助shadowing特性而已，其实是创建了新的变量
    println!("s1:{},len:{}", s1, len);
}

// 引用与借用
#[allow(dead_code)]
fn ownership_6() {
    // 解决上面不伦不类的写法
    let s = String::from("hello");
    // 传入变量的引用，&s 即为创建一个变量的引用这种方式称为 借用
    // 引用默认是不可变的，即：只能访问变量的值，但没有值的所有权，因此这种方式不会造成所有权的移交
    let len = calculate_length_1(&s);
    println!("s:{},len:{}", s, len);
}

// 可变引用
#[allow(dead_code)]
fn ownership_7() {
    // 引用默认是不可变的，如果需要通过引用修改值则需要创建可变引用
    let mut s = String::from("hello");
    // 为什么要写这么多 mut？可变引用的变量一定是可变的，如果变量都是不可变的，引用则无法修改值
    concat(&mut s);
    println!("s:{}", s)
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn takes_ownership_1(some_string: String) -> String { // some_string 进入作用域
    some_string
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn calculate_length_1(s: &String) -> usize {
    return s.len(); // len() 返回字符串的长度
}

fn concat(s: &mut String) {
    s.push_str(" world");
}

// 悬停引用：指向的内存可能已经被分配给其它持有者
// fn dangle() -> &String { // dangle 返回一个字符串的引用
//
//     let s = String::from("hello"); // s 是一个新字符串
//
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
// // 危险！


// 编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // * 解引用，元组裂变可以使用&item来接收，这样下面直接使用item就可以了
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}