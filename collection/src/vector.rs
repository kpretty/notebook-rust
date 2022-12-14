#[allow(dead_code)]
pub fn main() {
    create_vector();
    add_vector();
    read_vector();
    read_write_vector();
}


// 创建vector
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    // for 循环 这里用引用同理
    for x in &v {
        println!("{}", x);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn read_write_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    // 在同一个作用域中进行读写会存在问题
    let x = &v[1];
    v.push(6);
    // 有下面这句话就会报错，因为如果下面这行代码存在
    // x 的作用于会延伸到71行，v.push 需要一个可变的引用，而67行已经获取到了一个不可变的引用
    // 如果没有下面的代码 x 的生命周期在68行就结束了，不存在可变和不可变并存的情况
    // todo 为什么要这么设计
    // 因为 vector 在push的时候如果空间不足会开辟新的内存空间将数据复制到新的内存地址上，而x已经获取到了之前的引用
    // v在push之后会发生x指向一个未知地址
    //println!("x:{}", x);
}