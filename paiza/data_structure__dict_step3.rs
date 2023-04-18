// https://paiza.jp/works/mondai/data_structure/data_structure__dict_step3

use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let v: usize = lines.next().unwrap().trim().parse().unwrap();
    
    let mut count = BTreeMap::new();
    for _ in 0..v {
        let line = lines.next().unwrap();
        let entry = count.entry(line).or_insert(0);
        *entry += 1;
    }

    for (string, number) in count {
        println!("{} {}", string, number);
    }
}
