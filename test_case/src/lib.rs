pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn test_panic() {
    panic!("惶恐啦");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 遇到panic则则是报错
    #[test]
    #[should_panic]
    fn it_works3() {
        test_panic();
    }
}
