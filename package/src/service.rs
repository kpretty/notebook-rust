pub mod serving {
    use crate::service::Student;

    pub fn take_order() {
        Student {
            name: String::from("张三"),
            age: 12,
        };
        println!()
    }

    fn serve_order() {}

    fn take_payment() {}
}

// 结构体的可见性
struct Student {
    name: String,
    age: i8,
}