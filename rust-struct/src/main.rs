// 结构体与元组类似，其作用是组织数据，但与元组的区别在于
// 结构体的组织的数据有名称，比元组更直观易读
#[derive(Debug)] // 当前结构体派生自 Debug
#[allow(dead_code)] // 允许代码未被使用
struct User {
    user_name: String,
    age: u16,
    email: String,
    active: bool,
}

// 元组结构体，相当于给元组起一个名字，但它是结构体不是元组，有相当于没有字段名称的结构体
#[derive(Debug)] // 当前结构体派生自 Debug
#[allow(dead_code)] // 允许代码未被使用
struct Color(i8, i8, i8);

// 没有字段的结构体
#[allow(dead_code)] // 允许代码未被使用
struct AlwaysEqual;

// 元组可以定义方法
// 与函数不同在于
// 方法定义在元组的上下文中
// 需要使用实例进行调用
// 第一个参数总是self
// 例如：定义一个长方体，并提供计算面积的方法
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 定义Rectangle方法
#[allow(dead_code)]
impl Rectangle {
    // todo 为什么是 &self 或者是 &mut self
    // 这里的参数建议是引用，如果直接是 self，则会发生值所有权的移交，方法调用完毕后结构体实例将不能再被使用
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    // 第一个参数不是self的，成为关联函数，类比Java的静态方法，使用结构体调用
    // 通常用于结构体的初始化
    // Self 类型就是 impl 块类型的别称
    fn square(size: u32) -> Self /*Rectangle*/ {
        Rectangle { height: size, width: size }
    }
}


fn main() {
    // 实例化结构体
    let user = User {
        user_name: String::from("张三"),
        age: 16,
        email: String::from("123@gmail.com"),
        active: true,
    };
    println!("{:?}", user);
    let mut user1 = User {
        user_name: String::from("张三"),
        age: 16,
        email: String::from("123@gmail.com"),
        active: true,
    };
    // 修改结构体的值
    // 注：修改需要保证结构体的实例是可变的，如果结构体可变那么结构体里面的所有字段都是可变的
    // rust 不允许结构体内部分字段可变部分字段不可变
    user1.age = 20;
    println!("{:?}", user1);
    // 简单初识化，如果变量名与结构体字段名称一致，赋值时可以省略且顺序可以随便更改
    let user_name = String::from("李四");
    let age = 24;
    let email = String::from("123@gmail.com");
    let active = false;
    let user2 = User { age, email, active, user_name };
    println!("{:?}", user2);
    // 从已有的结构体实例中创建新的结构体实例
    // 即：新的结构体部分数据来源于已有的结构体
    let user3 = User {
        user_name: String::from("王五"),
        ..user
    };
    println!("{:?}", user3);

    let color = Color(1, 2, 3);
    println!("{:?}", color);

    let rectangle = Rectangle { height: 20, width: 20 };
    println!("结构体{:?} 面积是：{}", rectangle, rectangle.area());
}
