mod cell_refcell;

use std::ops::Deref;

fn main() {
    what_smart_pointer();
    where_smart_pointer();
}

// 什么是智能指针？
// Rust 的 trait 控制类型的行为，我们称实现了 Deref 和 Drop 任意一个的类型为智能指针
fn what_smart_pointer() {
    let element = 1;
    let my_box = MyBox(1);
    assert_eq!(1, element);
    assert_eq!(element, *my_box);
    assert_eq!(element, *my_box.deref());
}

// 智能指针到底智能在何处？
// 1. 可以自动解引用，提高开发体验
// 2. 可以自动化管理内存，安全无忧
fn where_smart_pointer() {
    // 1. 可以自动解引用，提高开发体验
    let user = User { name: "王一川" };
    let my_box = MyBox::new(user);
    // my_box可以直接调用user的方法
    my_box.name(); // 当遇到 . 操作调用的时候，会触发自动解引用。注：* 是强制解引用
    // 第二种自动解引用，作为函数的参数
    let line = String::from("王一川");
    print_str(&line);// 函数参数要求 &str,为什么 &String 也可以，是因为出发了自动解引用
}

// 自动解引用总结
// 1. *x 等价于 *(x.deref())
// 2. 使用点调用或在函数参数位置上对 x 自动解引用等价于 x.deref()
fn print_str(line: &str) {
    println!("{}", line);
}

// 定一个元组结构体
struct MyBox<T>(T);

// 定义构造方法
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 为 MyBox 实现 Deref
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct User {
    name: &'static str,
}

impl User {
    fn name(&self) {
        println!("{}", &self.name)
    }
}