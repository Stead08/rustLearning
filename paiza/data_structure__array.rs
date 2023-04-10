// https://paiza.jp/works/mondai/data_structure/data_structure__array_boss
use std::io;
use std::str::FromStr;

fn main() {
    // n と q の初期化
    let (n, q) = {
        let input = input_split::<usize>();
        (input[0], input[1])
    };

    // 配列 a の初期化
    let mut a = input_split::<u32>()
        .into_iter() // イテレータに変換
        .take(n) // 最初の n 要素を取得
        .collect::<Vec<_>>(); // ベクタに変換

    // q 個のクエリを処理
    for _ in 0..q {
        let query = input_split::<u32>();

        match query[0] {
            0 => a.push(query[1]), // 要素を追加
            1 => {
                a.pop(); // 最後の要素を削除
            }
            2 => println!(
                "{}",
                a.iter() // イテレータに変換
                    .map(ToString::to_string) // 文字列に変換
                    .collect::<Vec<_>>() // ベクタに変換
                    .join(" ") // 文字列に結合
            ),
            _ => {}
        };
    }
}

// 入力を取得して指定された型に変換するジェネリック関数
fn input_split<T: FromStr>() -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap(); // 1行を読み取る
    s.trim() // 両端の空白文字を削除
        .split_whitespace() // 空白文字で分割
        .map(|x| x.parse::<T>().unwrap()) // 指定された型に変換
        .collect() // ベクタに変換
}
