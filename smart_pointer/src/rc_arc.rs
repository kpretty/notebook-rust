/*
rust所有权机制
1. 每个值都有一个所有者
2. 同一个作用域只能有一个所有者
3. 当所有者离开作用于，对应值指定drop

如果被多个变量共享，又无法用引用来解决，无法确定生命周期

Rc  -> Reference count 引用技术
Arc -> atomic reference count 线程安全的引用技术
*/
use std::rc::Rc;

#[cfg(test)]
mod rc {
    use std::rc::Rc;
    use crate::rc_arc::{XStruct, XStructRc};

    #[test]
    #[allow(unused)]
    fn demo1() {
        let x = String::from("hello");
        let s1 = XStruct { x };
        // x 对应值的所有权已经被移交了，x 已经被 drop 掉了
        // let s2 = XStruct { x };
    }

    #[test]
    #[allow(unused)]
    fn demo2() {
        // 通过引用计数来判断变量什么时候被 drop，使用 Rc 来实现共享所有权
        let x = Rc::new(String::from("hello"));
        let rc1 = XStructRc { x: Rc::clone(&x) };
        println!("{}", Rc::strong_count(&x));
        let rc1 = XStructRc { x: Rc::clone(&x) };
        println!("{}", Rc::strong_count(&x));
    }
}
#[allow(dead_code)]
struct XStructRc {
    x: Rc<String>,
}

#[allow(dead_code)]
struct XStruct {
    x: String,
}