pub fn main() {
    iter_v1();
    iter_v2();
}

// 迭代器简单使用1
fn iter_v1() {
    let array = vec![1, 2, 3];
    for iterm in array.iter() {
        println!("{}", iterm)
    }
}

// 迭代器简单使用2
fn iter_v2() {
    let array = vec![1, 2, 3];
    let mut iter = array.iter();
    loop {
        match iter.next() {
            None => break,
            Some(iterm) => println!("{}", iterm),
        };
    }
}

// 迭代器简单使用3
fn iter_v3() {
    let mut array = vec![1, 2, 3];
    // 迭代不可变的引用
    let iter1 = array.iter();
    // 迭代可变的引用
    let iter3 = array.iter_mut();
    // 创建的迭代器会改变变量的所有权
    let iter2 = array.into_iter();
}