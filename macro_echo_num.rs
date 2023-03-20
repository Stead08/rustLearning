// 数値を画面に表示するマクロを定義
macro_rules! echo_num {
    ($num:expr) => { println!("{}", $num);}
}

fn main() {
    echo_num!(10);
    echo_num!(25);
    echo_num!(670);
}
