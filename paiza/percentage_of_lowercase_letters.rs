// https://paiza.jp/works/mondai/data_structure/data_structure__dict_step2

use std::collections::HashMap;
use std::io;

fn main() {
    // 入力文字列を取得
    let s = input_string();
    // 各文字の出現回数をカウント
    let count = count_chars(&s);

    // a から z までの文字について
    for (i, c) in (b'a'..=b'z').enumerate() {
        let ch = c as char;
        // 最後の文字以外の場合、空白で区切って出力
        if i < 25 {
            print!("{} ", count.get(&ch).unwrap_or(&0));
        } else {
            // 最後の文字の場合、空白を含めずに出力
            print!("{}", count.get(&ch).unwrap_or(&0));
        }
    }
    // 改行を出力
    println!();
}

fn input_string() -> String {
    let mut s = String::new();
    // 標準入力から文字列を読み込む
    io::stdin().read_line(&mut s).unwrap();
    // 末尾の改行文字を削除して返す
    s.trim().to_string()
}

fn count_chars(s: &str) -> HashMap<char, usize> {
    // 文字をキー、出現回数を値とするハッシュマップを作成
    let mut count: HashMap<char, usize> = HashMap::new();

    // 文字列内の各文字について
    for ch in s.chars() {
        // 文字が小文字のアルファベットである場合
        if ch.is_ascii_lowercase() {
            // ハッシュマップに文字が存在しなければ0を初期値として、値をインクリメント
            let counter = count.entry(ch).or_insert(0);
            *counter += 1;
        }
    }

    // 文字の出現回数を格納したハッシュマップを返す
    count
}
