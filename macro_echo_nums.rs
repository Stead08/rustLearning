//複数の数値を表示するマクロを定義
#[macro_export]
macro_rules! echo_nums {
    ($( $num:expr ),*) => {
        $ (
        print!("{}, ", $num);
        ) *
        println!("");
    }
}

//マクロを利用する
fn main() {
    echo_nums![20, 30, 40, 50, 60];
}
