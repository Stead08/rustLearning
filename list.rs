//単方向リストを実装したいエラーになる
pub struct Node {
    data: i64,
    link: Option<Box<Node>>
}

//手軽に単方向リストを生成する関数
fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node{data:v, link}))
}

fn main() {
    //単方向リストを表示
    let c = node(10, node(20, node(30, None))).unwrap();

    //先頭から各要素をたどって値を表示
    let mut p = &c;
    loop {
        println!("{}", p.data);
        //pが次の要素を指すように変更
        match p.link {
            None => break,
            Some(ref link) => p = link,
        }
    }
}
