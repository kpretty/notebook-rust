#[derive(Debug)]
enum IpAddrKind {
    IPV4,
    IPV6,
}

// 使用枚举作为函数的参数，可以接受枚举类型的任意实力
fn route(ip_addr_kind: &IpAddrKind) {
    println!("route:{:?}", ip_addr_kind)
}

// 枚举可以附带数据
#[derive(Debug)]
enum IpAddr {
    IPV4(String),
    IPV6(String),
}

// 枚举成员附带的数据类型可以不一致
#[derive(Debug)]
enum IpAddrEx {
    // 长度为4的u8数组
    IPV4([u8; 4]),
    // 字符串
    IPV6(String),
}

// 枚举也可以定义方法
impl IpAddrEx {
    fn call(&self) {
        // 下面就会说到的模式匹配
        match self {
            IpAddrEx::IPV4(_) => println!("我是IPV4"),
            IpAddrEx::IPV6(_) => println!("我是IPV6"),
        }
    }
}

#[allow(dead_code)]
#[allow(unused_doc_comments)]
#[allow(unused_variables)]
fn test_option() {
    // Rust没有Null，但是可以用于编码存在或不存在的枚举类型Option类型
    /// ```rust
    /// enum Option<T> {
    ///     None,
    ///     Some(T),
    /// }
    /// ```
    // Option 被定义在标准库中，属于Preclude，因此可以直接使用枚举类型和其成员
    let x: Option<u32> = Some(1);
    // 只有一个 None 编译器无法推断出 y 的数据类型，因此需要显示指定
    let y: Option<u32> = None;
    // 使用 Option 好处在于 Option 屏蔽了原始数据的操作，当需要操作原始数据时发现被Option装饰，就必须去显式的研判原始数据有没有可能是Null
    // 同时 Option 搭配模式匹配可以更好的操作数据
    let z = 1;
    // 下面报错
    //println!("{}", y + z)
    // 搭配模式匹配
    let result = match y {
        None => None,
        Some(i) => Some(i + 1)
    };
}

// 模式匹配
// 定义一个有很多成员的枚举类型
#[allow(dead_code)]
enum Color {
    Red,
    Yellow,
    Blue,
    Black,
    Green(String),
}

fn test_match(color: &Color) {
    match color {
        Color::Red => println!("我是红色"),
        Color::Yellow => println!("我是黄色"),
        Color::Blue => println!("我是蓝色"),
        Color::Black => println!("我是黑色"),
        Color::Green(message) => println!("我是绿色，我有名字，我叫:{}", message),
    }
}

// 模式匹配必须匹配所有，但可以使用通配符 _
fn test_match1(color: &Color) {
    match color {
        Color::Red => println!("我是红色"),
        _ => println!("我不是红色"),
    }
}

// 如果指向匹配某一个模式而忽略其他模式 可以使用if let
fn test_match2(color: &Color) {
    if let Color::Green(str) = color {
        println!("我是绿色，我有名字，我叫:{}", str)
    } else {
        // 这里相当于 _ 部分，也可以不要 else 代码块
        println!("怎么进入了else代码块呢")
    }
}

fn main() {
    // 创建枚举的实例
    let four = IpAddrKind::IPV4;
    let six = IpAddrKind::IPV6;
    // {} 调用DisPlay {:?} 调用Debug {:#?} 调用Debug并进行美化
    println!("ipv4:{:#?},ipv6:{:#?}", four, six);
    route(&four);
    route(&six);
    // 创建附带数据的枚举实例
    let ipv4 = IpAddr::IPV4(String::from("127.0.0.1"));
    let ipv6 = IpAddr::IPV6(String::from("::1"));
    // 创建附带不同数据类型的枚举实例
    let ipv4_ex = IpAddrEx::IPV4([127, 0, 0, 1]);
    let ipv6_ex = IpAddrEx::IPV6(String::from("::1"));
    println!("ipv4:{:?},ipv6:{:?}", ipv4, ipv6);
    println!("ipv4:{:?},ipv6:{:?}", ipv4_ex, ipv6_ex);
    // 调用枚举的方法
    ipv4_ex.call();
    // 测试模式匹配
    let color = Color::Green(String::from("绿帽子"));
    test_match(&color);
    test_match1(&color);
    test_match2(&color);
}
