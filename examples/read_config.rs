use std::{env, fs::File, io::Read};

use serde::Deserialize;

// 1. 读取yaml文件内容
// 2. 把文件内容转成struct

#[derive(Debug, Deserialize)]
struct Config {
    a: i32,
    b: String,
    c: Option<String>,
    #[serde(default)]
    ddd: String,
}

fn main() {
    // pwd
    let current_path = env::current_dir().unwrap();
    let cwd = current_path.into_os_string().into_string().unwrap();
    let mut content = String::new();
    let example_path = format!("{}/examples/a.yaml", &cwd);
    println!("example_path: {}", example_path);
    let mut file = File::open(&example_path).unwrap();
    file.read_to_string(&mut content).unwrap();
    // read success
    println!("{}", content);
    let config: Config = serde_yaml::from_str(&content).unwrap();
    println!("{:?}", config);
}
