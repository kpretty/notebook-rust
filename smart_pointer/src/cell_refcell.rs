#[cfg(test)]
mod test {
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
}

fn add_one(x: &mut i32) {
    *x += 1;
    println!("x={}", x);
}