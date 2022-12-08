use std::thread;
use std::time::Duration;

pub fn main() {
    // 什么是闭包：可以捕获其所在环境的匿名函数
    // 1. 匿名函数
    // 2. 可以保存为变量、作为参数
    // 3. 可以在一个地方创建闭包，然后再另一个上线文中调用闭包来完成运算
    // 4. 可以从定义的作用域捕获值
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 模拟复杂的业务逻辑
fn _simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 将simulated_expensive_calculation提取成闭包
    // 可以手动指定类型参数，也可以通过后续调用让编译器进行推断
    let _expensive_closure: fn(u32) -> u32 = |intensity| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    let mut expensive_closure = Cacher::new(|intensity| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });


    if intensity < 25 {
        println!(
            "Today, do {} pushups!", expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!", expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

// 闭包的类型推断
/// 闭包不要求标注参数和返回值类型
/// 为什么函数需要？因为函数是暴露给用户的接口，就必须严格要求输入输出参数的类型，取的共识
/// 闭包不会被用于接口，被存储在变量中
/// 闭包通常是短小的，只在狭小上下文中工作，编译器通常是可以推断出类型的
/// 闭包只会为每个参数和返回值推断一次


// 泛型闭包
struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

// 实现缓存
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    // 构造方法，实例化闭包
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    // 为value实现getter方法
    fn value(&mut self, intensity: u32) -> u32 {
        match self.value {
            None => {
                // 如果value为空则调用函数并赋值
                let result = (self.calculation)(intensity);
                self.value = Some(result);
                result
            }
            Some(v) => v
        }
    }
}

// 闭包可以捕获上下文的变量，函数不行，但会有额外的开销
fn demo_closures() {
    let a = 1;
    let closures = |x| { x == a };
    println!("{}", closures(1))
}