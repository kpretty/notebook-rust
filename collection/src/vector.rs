pub fn main() {
    create_vector();
    add_vector();
    read_vector();
}


// 创建vector
fn create_vector() {
    // 必须指定类型，否编译器无法推断
    let v: Vec<i32> = Vec::new();
    println!("vec:{:?}", v);
    // 将字符串的每一个字符转换成ascii
    let v = Vec::from(String::from("1 2 3"));
    println!("vec:{:?}", v);
    // 使用宏
    let v = vec![1, 2, 3, 4, 5];
    println!("vec:{:?}", v);
}

// add
fn add_vector() {
    // 默认为不可变类型，添加数据相当于修改集合
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("vec:{:?}", v);
}

// read
fn read_vector() {
    let v = vec![1, 2, 3, 4, 5];
    // 看起来两种方式都能获取到数据，但涉及到所有权
    // 移交了所有权，但因为存储的是基本数据类型实现了Copy，所以无感
    let v1 = v[1];
    // 没有移交所有权
    let v2 = &v[1];
    println!("v1:{},v2:{}", v1, v2);
    // 看下面例子
    let v = vec![String::from("1"), String::from("2"), String::from("3")];
    // 这种方式就不行，移交了所有权，那么集合该位置的数据指向谁就变得很模糊。存在内存不安全情况
    // let v1 = v[1];
    // 使用这种方式获取引用
    let v2 = &v[1];
    println!("v2:{}", v2);
    // 通过方法获取数据
    let option = v.get(2);
    //let option = v.get(200);
    match option {
        None => println!("没有这个值"),
        Some(element) => println!("这个值是：{}", element)
    }
    // 通过[]方式 越界会报错
    //let v2 = &v[10];
}