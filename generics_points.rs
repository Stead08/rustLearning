struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn point(self) -> (T, T) {
        (self.x, self.y)
    }
}

fn print_point<T: std::fmt::Display>(p: Point<T>) {
    //タプルでx座標とy座標を取り出しておく
    let (x, y) = p.point();
    println!("({}, {})", x, y);
}

fn main() {
    //(a)型推論によりPointのジェネリック型がf32と判明
    let point = Point{
        //これらはf32型
        x: 1.0,
        y: 2.0,
    };

    //(b)Pointのジェネリック型がf32と判明しているので
    //この関数のジェネリック型もf32と推論
    print_point(point);
}
