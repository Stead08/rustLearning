use std::ops::Add;

//お金の値を保持する構造体
#[derive(Debug)]
struct Money(u32);

//addトレイトの実装を用意
impl Add for Money {
    //関連型
    type Output = Self;

    fn add(self, rhs: Money) -> Self::Output {
        //addメソッドにその方における演算内容を定義
        Money(self.0 + rhs.0)
    }
}

fn main() {
    let hundred = Money(100);
    let thousand = Money(1000);
    //Addトレイトを実装しているので加算できる
    let total = hundred + thousand;
    println!("{:?}", total);
}
