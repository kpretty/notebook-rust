#[allow(dead_code)]
pub fn main() {
    // 出现 panic，rust默认会回溯栈，并清理遇到的每一个函数的数据
    // 因此回溯会有很多工作，另一种方式是 abort，rust程序直接终止
    // 不清理数据直接退出，程序所使用的内存后续将由操作系统来处理
    // 如果你想让你的rust最终二进制文件足够的小，在 Cargo.toml中[profile]中添加 panic = "abort"
    // 或者在构建release的时候使用 [profile.release]
    panic!("发生了一个错误");
}