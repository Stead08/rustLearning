//二つのtxtファイルを比較する
use std::fs;

fn main() {
    //ファイル名を指定
    let afile= "./lib/FizzBuzzA.txt";
    let bfile= "./lib/FizzBuzzB.txt";

    //ファイルを文字列として読む
    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();

    //念の為トリム
    let astr = astr.trim();
    let bstr = bstr.trim();

    //比較
    if astr == bstr {
        println!("Ok");
    } else {
        println!("NG");
    }
}
