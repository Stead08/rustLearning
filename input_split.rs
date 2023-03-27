use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    
    let vec: Vec<&str> = input.split_whitespace().collect();
    //　a1 a2 a3 ... an の標準入力を受け取りプリント
    for i in 0..2 {
        let x = vec[i].trim().to_string();
        println!("{}", x);
    }
}
