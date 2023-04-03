//書籍を表す構造体の定義
struct Book {
    name: String,
    author: String,
    isbn: String,
}

fn main() {
    let book = Book {
        name: "Rust入門".to_string(),
        author: "著者太郎".to_string(),
        isbn: "978-4-7981-5757-5".to_string(),
    };

    //それぞれの値を取り出して標準出力
    println!("書籍名: {}", book.name);
    println!("著者名: {}", book.author);
    println!("ISBN: {}", book.isbn);
}
