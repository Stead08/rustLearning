use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    //rcで包んだNodeを用意する
    let node_c = Rc::new(Node{val: 3, next: None});
    //この時点の参照カウント「1」を表示
    println!("count: {}", Rc::strong_count(&node_c));
    //（b’）ブロックをわざと増やす
    {
        let node_a = Node {
            val: 1,
            next: Some(Rc::clone(&node_c)),
        };
        //（c’）node_cへのポインターを追加しため参照カウントは「2」
        println!("count: {}", Rc::strong_count(&node_c));

        let node_b = Node {
            val: 2,
            next: Some(Rc::clone(&node_c)),
        };
        //（d’）node_cへのポインターを追加しため参照カウントは「3」
        println!("count: {}", Rc::strong_count(&node_c));

        println!("{:?}", node_a);
        println!("{:?}", node_b);

        //（e’）ブロック終了によりブロック内で増えた参照カウントが引かれる
    }

    //（f’）参照カウントが再び「1」になる
    println!("count: {}", Rc::strong_count(&node_c));

} //（g’）node_cの寿命が尽きて参照カウントが0になり破棄される
}
