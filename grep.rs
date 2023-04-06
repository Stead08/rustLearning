use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

struct GrepArgs {
    pattern: String,
    file_path: String,
}

//コンストラクタの実装
impl GrepArgs {
    fn new(pattern: String, file_path: String) -> Self {
        GrepArgs {
            pattern: pattern,
            file_path: file_path,
        }
    }
}

fn grep(args: GrepArgs) {
    //指定されたファイルパスにあるファイルを読み込む
    let content = read_to_string(args.file_path);
    //指定された検索パターンを含む行を探し出して表示する
    match content {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(args.pattern.as_str()) {
                    println!("{}", line);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn main() {
    // 標準ライブラリーで実行時引数を解析しデータを取り出す
    let args: Vec<String> = args().collect();
    // 引数の数をチェック
    if args.len() != 3 {
        eprintln!("引数は二個でなければなりません");
        exit(1);
    }

    let pattern = &args[1];
    let file_path = &args[2];

    // 受け取ったデータをGrepArgs構造体に詰める
    let args = GrepArgs::new(pattern.to_string(), file_path.to_string());
    // grep関数を呼び出す
    grep(args);


}
