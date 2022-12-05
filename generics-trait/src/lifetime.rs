// 生命周期避免悬垂引用
// rust 的借用检查器(borrow checker)，通过比较作用域来确保所有的借用都是有效的
// 下面的例子作用域比较如下
/// ```rust
/// {
///     let r;                // ---------+-- 'a
///                           //          |
///     {                     //          |
///         let x = 5;        // -+-- 'b  |
///         r = &x;           //  |       |
///     }                     // -+       |
///                           //          |
///     println!("r: {}", r); //          |
/// }                         // ---------+
/// ```
// 显然 'a > 'b 但 r 确借用了一个比它自身生命周期还要短的变量，因此编译不过
// 因此对于 let a = &b; 要求 'b ≥ 'a
fn _demo() {
    let r;
    {
        let x = 5;
        r = &x; // borrowed value does not live long enough
    }
    //println!("r: {}", r);
}

// 函数中的泛型生命周期
// 比较两个数组谁的长度更长，并返回长度更长的数组引用
// 因为在编译期间不清楚函数的传参，borrow checker 无法判断x，y与返回值的生命周期是如何关联的
// 因此需要显示标注每个变量的生命周期让 borrow checker
// 'a 只是一个标识符 rust 规定以 ' 开头表示注明一个生命周期，约定生命周期名称尽可能的短
// &i32         引用
// &'a i32      带有显式生命周期的引用
// &'a mut i32  带有显式生命周期的可变引用
// 下面生命周期的意思是：声明一个叫'a的生命周期，且x,y和函数返回值的生命周期是一样的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 深入理解生命周期
// 例如上述代码，如果返回值引用的生命周期与y没有关系，那么y可以不用声明生命周期
fn longest_without_y<'a>(x: &'a str, y: &str) -> &'a str {
    println!("和y:{}没有关系", y);
    x
}

fn longest_with_x(x: &str) -> &str {
    x
}

// 即使都声明了生命周期，但函数实际返回值与xy没有关系，也会报错
// fn longest_without_x_y<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let result = String::from("和xy都没有关系的返回值");
//     result.as_str()
// }

// 结构体生命周期
// 存在的必要性：如果结构体需要存储变量的引用
struct ImportantExcerpt<'a> {
    part: &'a str,
}
