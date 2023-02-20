use std::{env, fs};
fn main() {
    //コマンドライン引数を得る
    let args = env::args();
    let mut total: f64 = 0.0;
    //全てのコマンドライン引数を処理
    for (i, fname) in args.enumerate() {
        if i == 0 {
            continue;
        }
        //テキストファイルを読み込む
        let text = fs::read_to_string(fname).unwrap();
        // 一行ごとに区切る
        let lines = text.split('\n');
        // 繰り返し加算
        for line in lines {
            //数値に変換
            let n:f64 = line.parse().unwrap_or(0.0);
            total += n;
        }
    }
    // 結果を表示
    println!("{}", total);
}
