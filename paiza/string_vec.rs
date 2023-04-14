// https://paiza.jp/works/mondai/data_structure/data_structure__string_boss

use std::io;

fn main(){
    let s: Vec<usize> = input_split();
    let (h, w, r, c) = (s[0], s[1], s[2] - 1, s[3] - 1);
    let mut vec: Vec<Vec<char>> = vec![];
    for i in 0..h {
        let mut v = String::new();
        io::stdin().read_line(&mut v).unwrap();
        let v: Vec<char> = v.chars().collect();
        vec.push(v);
    }
    if vec[r][c] == '#' {
        println!("Yes");
    } else {
        println!("No");
    }

}

fn input_split<T: std::str::FromStr>() -> Vec<T>
    where
        T::Err: std::fmt::Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}
