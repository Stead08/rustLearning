fn main() {
    //0から100未満の偶数を累計する
    let num: Vec<u32> = (0..100).collect();
    let even_sum: u32 = num.iter()
        .filter(|n| *n % 2 == 0)
        .sum();
    println!("{:?}", num);
    println!("{}", even_sum);
}
