#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let linked_list = Node {
        val: 1,
        next: Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node { val: 3, next: None})),
        })),
    };
    println!("{:?}", linked_list);
}
