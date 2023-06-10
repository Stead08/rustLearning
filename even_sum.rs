fn main() {
    //0から100未満の偶数を累計する
    let num_list: Vec<i32> = (0..100).collect();
    println!("{}", for_sum(&num_list));
    println!("{}", list_sum(&num_list));

}
//for文を用いたsum
fn for_sum(num_list: &Vec<i32>) -> i32 {
    let mut count = 0;
    for i in num_list {
        if i % 2 == 0 {
            count += i
        }
    }
    count
}
//関数型思考を用いたsum
fn list_sum(num_list: &[i32]) -> i32 {
    let even_sum: i32 = num_list.iter()
        .filter(|n| *n % 2 == 0)
        .sum();
    even_sum
}
