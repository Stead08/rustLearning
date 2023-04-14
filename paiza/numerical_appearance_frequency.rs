use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: usize = s.trim().parse().unwrap();
    let vec: Vec<usize> = input_split();
    let mut count = [0; 10];
    for i in 0..s {
        count[vec[i]] += 1;
    }
    let dst: Vec<String> = count.iter().map(|x| x.to_string()).collect();
    println!("{}", dst.join(" "));
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
