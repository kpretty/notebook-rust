mod service;
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_wait_list
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            // 除了 crate 外 还可以使用 super 和 self 引用父类和当前模块的函数
            self::seat_at_table();
            // 注意模块的可见性
            super::serving::take_order();
        }

        fn seat_at_table() {

        }
    }
    mod serving {
        struct Student {
            name: String,
            age: i32
        }

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
}

// 文件内引用模块的函数
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_wait_list();
    // 相对路径
    front_of_house::hosting::add_to_wait_list();
}


