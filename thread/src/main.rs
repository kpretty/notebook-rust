use std::thread;
use std::time::Duration;

fn main() {
    //create_thread();
    move_var_thread();
}

fn create_thread() {
    // 创建一个线程
    let handle = thread::spawn(|| {
        for i in 0..100 {
            println!("number-inner：{}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    for i in 0..100 {
        println!("number-outer：{}", i);
        thread::sleep(Duration::from_millis(10));
    }
    // 阻塞当前线程，让子线程顺利执行完
    handle.join().unwrap();
}

fn move_var_thread() {
    // 闭包可以借用外部作用域的数据
    let vec1 = vec![1, 2, 3, 4];
    // 新开一个线程来打印vec
    let handle = thread::spawn(move || {
        println!("{:?}", vec1);
    });

    handle.join().unwrap();
}
