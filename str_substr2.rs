//enumerateメソッドを使ったインデックスを用いて任意の部分文字列を取り出すプログラム
fn main() {
    let pr = "知恵は武器よりも価値がある";

    //　先頭2文字の部分文字列を得る
    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 {sub1.push(c); continue;}
        break;
    }
    println!("先頭2文字: {}", sub1);

    //[武器]の部分を得る
    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        // (0から数えて3から4文字目）
        if (3..=4).contains(&i) {sub2.push(c)}
    };
    println!("4-5文字目: {}", sub2);
}
