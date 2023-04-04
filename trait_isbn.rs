struct Isbn10;
struct Isbn13;

//トレイと
trait IsbnKind {}

impl IsbnKind for Isbn10 {}
impl IsbnKind for Isbn13 {}

//ジェネリクス
struct Isbn<K: IsbnKind>(String, K);

fn main() {
    let isbn = Isbn("978-4-7981-5757-8".to_string(), Isbn13);
    println!("{}", isbn.0);
}
