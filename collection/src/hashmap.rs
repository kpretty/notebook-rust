use std::collections::HashMap;

pub fn main() {
    create_hashmap_insert();
    read_hashmap();
    update_hashmap();
    word_count();
}

// 创建hashmap并添加数据
fn create_hashmap_insert() {
    // hashmap 不在标准库中 需要自己导入
    // 仅使用 new 进行创建，编译器无法推断出变量类型，可以手动指定，也可以通过上下文添加数据
    let mut hash_map = HashMap::new();
    hash_map.insert(String::from("k1"), 10);
    hash_map.insert(String::from("k2"), 20);
    println!("hashmap:{:?}", hash_map);
    // 使用zip将两个vec压缩成一个hashmap
    let keys = vec!["k1".to_string(), "k2".to_string()];
    let values = vec![10, 20];
    // 必须显式指定类型
    let result: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();
    println!("result:{:?}", result);
}

// 读取hashmap数据
fn read_hashmap() {
    let mut hash_map = HashMap::new();
    let k1 = String::from("k1");
    let k2 = String::from("k2");
    // 推荐插入变量的引用，否则会发生所有权的移交
    hash_map.insert(&k1, 10);
    hash_map.insert(&k2, 20);
    // 通过get获取，同样返回 option
    match hash_map.get(&k1) {
        None => println!("没有这个key"),
        Some(v) => println!("值：{}", v),
    }
    // for 循环遍历
    for (k, v) in &hash_map {
        println!("{}: {}", k, v);
    }
}

// 更新hashmap
fn update_hashmap() {
    let mut hash_map = HashMap::new();
    let k1 = String::from("k1");
    let k2 = String::from("k2");
    // 推荐插入变量的引用，否则会发生所有权的移交
    hash_map.insert(&k1, 10);
    hash_map.insert(&k1, 20);
    // 插入相同值会覆盖
    println!("{:?}", hash_map);
    // 如果k不存在就插入
    hash_map.entry(&k1).or_insert(30);
    hash_map.entry(&k2).or_insert(30);
    println!("{:?}", hash_map);
}

// 统计单词出现的个数
fn word_count() {
    let text = "hello world hello rust";
    let mut map = HashMap::new();
    for k in text.split_whitespace() {
        let v = map.entry(k).or_insert(0);
        // 如果存在则返回，此时返回value的引用，对其解引用后+1
        *v += 1
    }
    println!("{} 单词个数 {:?}", text, map);
}