use std::cell::{Cell, RefCell};

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;
    use super::*;

    #[test]
    fn demo1() {
        // 可变性需要使用mut来修饰
        let mut x = 0;
        x += 1;
        println!("{}", x)
    }

    #[test]
    fn demo2() {
        // 可变性需要使用mut来修饰
        let mut x = 0;
        add_one(&mut x);
        x += 1;
        println!("x={}", x)
    }

    #[test]
    fn demo3() {
        let mut x = 1;
        // 这里有一个x的可变引用
        let x_struct = XStruct { x: &mut x };
        // 这里有进行了一次可变引用，但 rust 在一个作用域只允许一个可变引用
        //x += 1;
        // 虽然有借用规则限制，但还是想要多个可变引用，这就需要内部可变性
        // 将可变的变量包装在一个不可变的结构中
        // 什么是内部可变性：没有显示的mut修饰，但却可以多处对内部的数据进行修改
        println!("{:?}", x_struct);
        println!("{}", x_struct.x);
        println!("{}", x);
    }

    #[test]
    fn demo4() {
        let x = RefCell::new(1);
        let struct_ref_cell = XStructRefCell { x: &x };
        *x.borrow_mut() += 1;
        println!("{:?}", struct_ref_cell);
        println!("{:?}", struct_ref_cell.x);
        println!("{:?}", x);
    }

    #[test]
    fn demo5() {
        let x = Cell::new(1);
        let struct_ref_cell = XStructCell { x: &x };
        // get 是 copy 语义，因此建议使用 RefCell
        x.set(x.get() + 1);
        println!("{:?}", struct_ref_cell);
        println!("{:?}", struct_ref_cell.x);
        println!("{:?}", x);
    }
}

#[derive(Debug)]
struct XStruct<'a> {
    x: &'a mut i32,
}

#[derive(Debug)]
struct XStructRefCell<'a> {
    x: &'a RefCell<i32>,
}

#[derive(Debug)]
struct XStructCell<'a> {
    x: &'a Cell<i32>,
}

fn add_one(x: &mut i32) {
    *x += 1;
    println!("x={}", x);
}