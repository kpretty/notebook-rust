use package::front_of_house::hosting::add_to_wait_list;
use crate::service::serving::take_order;

mod service;

fn main() {
    println!("Hello, world!");
    // 调用模块里的方法
    add_to_wait_list();
    take_order()
}
