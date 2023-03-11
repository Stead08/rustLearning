#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let pt_i = Point { x: 1, y: 2 };
    let pt_f = Point { x:1.0, y:2.0 };
    println!("{:?}", pt_i);
    println!("{:?}", pt_f);
}
