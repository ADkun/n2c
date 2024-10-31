use std::env;

fn main() {
    let n: f64 = env::args()
        .into_iter()
        .skip(1)
        .next()
        .expect("需要有一个数字参数")
        .parse()
        .expect("参数必须是一个数字");

    let readable = get_human_readable_size(n);
    println!("{readable}");
}

fn get_human_readable_size(mut size: f64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut unit_index = 0;
    while size > 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    format!("{:.2} {}", size, units[unit_index])
}
