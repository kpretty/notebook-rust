#[allow(dead_code)]
pub fn main() {
    create_string();
    update_string();
    index_string();
}

// 新建字符串
#[allow(dead_code)]
fn create_string() {
    // 通过关联函数
    // 创建空字符串
    let s = String::new();
    println!("s:{}", s);
    // 通过字符串字面量创建字符串
    let s = String::from("你好");
    println!("s:{}", s);
    let s = "我是小王".to_string();
    println!("s:{}", s);
}

// 更新字符串
#[allow(dead_code)]
fn update_string() {
    let mut s = String::from("foo");
    // 追加，传入字符串切片，所以也不会发生所有权的移交，可以放心使用
    // todo &str 可以称为字符串切片也可以成为字符串字面量，&String和&str本质上是一样的，字符串引用和切片[..]是一样的结果
    s.push_str("bar");
    println!("s:{}", s);
    // 使用 format宏 拼接字符串，不会发生所有权的移交
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s = format!("{}-{}", s1, s2);
    println!("s:{},s1:{},s2:{}", s, s1, s2);
    // 使用 + 拼接，+ 号左边的变量会发生所有权的移交，因为底层调用的是add(self, s: &str)
    let s = s1 + &s2;
    println!("s:{}", s);
}

// 索引字符串
#[allow(dead_code)]
fn index_string() {
    let s = "hello 你好".to_string();
    // 不允许通过索引下标获取，因为String底层使用vec<u8>存储，&s[1] 为获取第一个字节，
    // 因为字符串存储utf8各字符的字节数不一致，例如对于中文为3个字节，取第一个字节可能就不是一个合法编码
    //let x = &s[1];
    let x = s.len();
    println!("s:{s},x:{x}");
    // 遍历unicode标量值，对于一些奇奇怪怪的语言 这个也是有问题的
    // 这不是rust的问题，所有语言都是这样，除非编程语言支持字形簇，才是我们理解那样的字符
    for x in s.chars() {
        println!("x:{}", x)
    }
    // 字符串切片 慎用 可能会报错
    let s1 = &s[..2];
    println!("s:{s1}");
}