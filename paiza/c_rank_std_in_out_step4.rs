// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_std_in_out_step4
//最大値をプリント

use std::io;

fn main() {
    let n = input_num();
    let mut max_value = 0;
    for _ in 0..n {
        let x = input_num();
        if x > max_value {
            max_value = x;
        }
    }
    println!("{}", max_value);
}

fn input_num() -> i32 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    n.trim().parse().unwrap()
}
