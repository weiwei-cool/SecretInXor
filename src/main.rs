use std::fs;
use std::io::stdin;

fn main() {
    println!("请输入密钥！");
    let mut key = String::new();
    stdin().read_line(&mut key)
        .expect("错误！");
    let key: u8 = key.trim().parse().expect("Invalid key");

    let mut args = std::env::args();
    let file_path = args.nth(1).unwrap();
    let context = fs::read(&file_path);

    let mut out_context = context.unwrap();
    out_context = out_context.iter().map(| b | b ^ key).collect();

    fs::write(&file_path, out_context)
        .unwrap();
}
