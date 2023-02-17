//3の倍数と3の付く整数の場合はAを表示
fn main() {
    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3 {
            println!("A");
            continue;
        }
        if (3..8).contains(&i) {
            println!("A");
            continue;
        }
        println!("{}", i);
    }
}
