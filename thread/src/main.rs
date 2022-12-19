use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

fn main() {
    //create_thread();
    // move_var_thread();
    // demo_channel();
    // demo_channel_repeatedly();
    demo_channel_multiple();
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
    // move 会改变所有权，将值的所有权移交到线程中，线程结束后生命周期结束
    // println!("{:?}",vec1);

    handle.join().unwrap();
}

// 通过channel进行消息传递
fn demo_channel() {
    let (producer, consumer) = mpsc::channel();
    // 开启新线程来发送消息
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        // 发送完 msg 所有权就被移交了，这也是为什么规定只能有一个consumer
        let msg = "hello rust";
        producer.send(msg).unwrap();
    });

    // recv会阻塞方法直到获取到值
    // let msg = consumer.recv().unwrap();
    // println!("{}", msg);
    // try_recv 非阻塞方法，会立刻返回Result，可以通过循环不断的获取
    loop {
        match consumer.try_recv() {
            Ok(msg) => {
                println!("通过 try 获取的:{}", msg);
                break;
            }
            Err(_) => println!("没有获取到数据"),
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn demo_channel_repeatedly() {
    let (producer, consumer) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec!["hello", "rust", "hi", "coder"];
        for msg in &msgs {
            producer.send(*msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("{:?}", msgs);
    });

    // 将consumer直接应用到迭代器中
    for msg in consumer {
        println!("{}", msg);
    }
}

fn demo_channel_multiple() {
    // 多个producer，这里需要使用
    let (producer, consumer) = mpsc::channel();
    // 必须要先复制一个
    let sender = producer.clone();
    thread::spawn(move || {
        let msgs = vec!["hello", "rust", "hi", "coder"];
        for msg in &msgs {
            producer.send(*msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("{:?}", msgs);
    });
    thread::spawn(move || {
        let msgs = vec!["one", "two", "three", "four"];
        for msg in &msgs {
            sender.send(*msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("{:?}", msgs);
    });

    // 将consumer直接应用到迭代器中
    for msg in consumer {
        println!("{}", msg);
    }
}