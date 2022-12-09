use prettytable::{format, row, Table};

fn main() {
    // 创建表格
    let mut table = Table::new();
    // 设置样式
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    // 设置表头
    table.set_titles(row!["name","sex","age","class"]);
    // 设置数据
    table.add_row(row!["zs","m",21,"3-2"]);
    table.add_row(row!["ls","n",22,"3-1"]);
    // 打印table
    table.printstd();
}
