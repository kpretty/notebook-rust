fn main() {
    // let vec1 = vec![1, 2, 3, 4, 5];
    // search_max(&vec1);
    // 实例化泛型结构体
    let point = Point { x: 1, y: 1.1 };
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
struct Point<T, V> {
    x: T,
    y: V,
}

// 定义泛型枚举
enum MyResult<T> {
    Ok(T),
    Err,
}

// 为结构体实现泛型方法
// T V 为结构体的泛型
impl<T, V> Point<T, V> {
    // T1 V1 为方法的泛型
    fn distance<T1, V1>(&self, point: &Point<T1, V1>) -> f64 {
        1.2
    }
}

// 为具体类型的结构体实现方法'
impl Point<i32, i32> {
    fn distance_i_f(&self, point: &Point<i32, i32>) -> f64 {
        let x = (self.x - point.x).pow(2) + (self.y - point.y).pow(2);
        return x as f64;
    }
}
// 注：rust的泛型是单态化