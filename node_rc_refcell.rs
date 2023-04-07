use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>
}

fn main() {
    //内部可変性を許可するように変数を束縛する
    let node_c = Rc::new(RefCell::new(Node {val: 3, next: None}));
    let node_a = Rc::new(RefCell::new(Node {val: 1, next: None}));
    let node_b = Rc::new(RefCell::new(Node {val: 2, next: None}));

    //node_aから可変参照を得ることでnode_aの値を変更できるようにする
    node_a.borrow_mut().next = Some(Rc::clone(&node_c));
    //node_aから可変参照を得ることでnode_aの値を変更できるようにする
    node_b.borrow_mut().next = Some(Rc::clone(&node_c));

    println!("{:?}", node_a);
    println!("{:?}", node_b);


}
