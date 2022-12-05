use std::fmt::{Debug, Display, Formatter};

fn main() {
    // let vec1 = vec![1, 2, 3, 4, 5];
    // search_max(&vec1);
    // 实例化泛型结构体
    let _point = Point { x: 1, y: 1.1 };
    // 实现类似多态的概念
    let people = People {};
    say(&people);
    say1(&people);
    let dog = Dog {};
    say(&dog);
    say1(&dog);
    say2(&people, &dog);
    say3(&people, &dog);
}

// 定义泛型方法，当前缺少泛型边界后续调试
// fn search_max<T>(arr: &Vec<T>) -> T {
//     // 假设第一个元素就是最大的
//     let mut max_element = &arr[0];
//     for x in arr {
//         if x > max_element {
//             max_element = x;
//         }
//     }
//     return max_element;
// }

// 定义泛型结构体
#[allow(dead_code)]
struct Point<T, V> {
    x: T,
    y: V,
}

// 定义泛型枚举
#[allow(dead_code)]
enum MyResult<T> {
    Ok(T),
    Err,
}

// 为结构体实现泛型方法
// T V 为结构体的泛型
impl<T, V> Point<T, V> {
    // T1 V1 为方法的泛型
    #[allow(dead_code)]
    fn distance<T1, V1>(&self, _point: &Point<T1, V1>) -> f64 {
        1.2
    }
}

// 为具体类型的结构体实现方法'
impl Point<i32, i32> {
    #[allow(dead_code)]
    fn distance_i_f(&self, point: &Point<i32, i32>) -> f64 {
        let x = (self.x - point.x).pow(2) + (self.y - point.y).pow(2);
        return x as f64;
    }
}
// 注：rust的泛型是单态化

// trait 特质，某些特定的类型拥有与其他类型共享的功能，通过 trait 以一种抽象的方式定义共享行为
// 定义 trait
trait Animal {
    fn say(&self);

    fn eat(&self);
}

// 定义结构体
#[derive(Debug)]
struct People {}

#[derive(Debug)]
struct Dog {}

// 为结构体实现trait
impl Animal for People {
    fn say(&self) {
        println!("我是人")
    }

    fn eat(&self) {
        println!("我吃熟的")
    }
}

impl Animal for Dog {
    fn say(&self) {
        println!("我是狗")
    }

    fn eat(&self) {
        println!("我吃生的")
    }
}

impl Display for People {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Display for Dog {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

// 一旦结构体实现了某个trait，就可以看成是具体的trait数据类型。例如上述的People和Dog都可以统一看成Animal类型
fn say(animal: &impl Animal) {
    animal.eat()
}

// 对say进行改写
fn say1<T: Animal>(animal: &T) {
    animal.eat();
}

// 对say1在进行一次改写，要求参数不行实现了Animal还要实现Display和Debug，同时有两个参数
fn say2<T: Animal + Display + Debug, K: Animal + Display>(animal1: &T, animal2: &K) {
    println!("{:#?}", animal1);
    // 因为animal2没有实现Debug
    // println!("{:#?}", animal1);
    println!("{}", animal2);
}

// 对say2在进行一次改写，trait bound 这样写太长了，进行简化
fn say3<T, K>(animal1: &T, animal2: &K)
    where T: Animal + Display + Debug,
          K: Animal + Display {
    println!("{:#?}", animal1);
    // 因为animal2没有实现Debug
    // println!("{:#?}", animal1);
    println!("{}", animal2);
}