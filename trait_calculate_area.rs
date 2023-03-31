//長方形
struct Rectangle {
    width: f32,
    height: f32,
}

//三角形
struct Triangle {
    width: f32,
    height: f32,
}

//台形
struct Trapezium {
    upper_base: f32,
    lower_base: f32,
    height: f32,
}

//面積を計算するという振る舞いを定義するトレイと
trait AreaCalculator {
    fn area(&self) -> f32;
}

//長方形に対して面積の計算を実装する
impl AreaCalculator for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

//三角形に対して面積の計算を実装する
impl AreaCalculator for Triangle {
    fn area(&self) -> f32 {
        self.width * self.height / 2_f32 //f32として解釈する
    }
}

//台形に対して面積の計算を実装する
impl AreaCalculator for Trapezium {
    fn area(&self) -> f32 {
        (self.upper_base + self.lower_base) * self.height / 2_f32
    }
}

fn print_result<A: AreaCalculator>(japanese_name:&str, figure: A) {
    println!("{}の面積は{}です", japanese_name, figure.area());
}

fn main() {
    //それぞれの図形の構造体を用意する
    let rectangle = Rectangle {
        width: 10_f32,
        height: 5_f32,
    };

    let triangle = Triangle {
        width: 10_f32,
        height: 5_f32,
    };

    let trapezium = Trapezium {
        upper_base: 10_f32,
        lower_base: 5_f32,
        height: 5_f32,
    };
    print_result("長方形", rectangle);
    print_result("三角形", triangle);
    print_result("台形", trapezium);
}

