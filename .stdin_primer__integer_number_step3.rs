//1 行目で、整数 N と、続けて N 個の整数 a_1, ... , a_N が半角スペース区切りで与えられます。
//a_1, ... , a_N を改行区切りで出力してください。

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    
    let vec: Vec<&str> = input.split_whitespace().collect();
    
    for i in 1..=vec[0].parse().unwrap() {
        let x = vec[i].trim().to_string();
        println!("{}", x);
    }
}
