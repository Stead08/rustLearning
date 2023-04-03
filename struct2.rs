//書籍を表す構造体の定義
struct Book {
    name: String,
    author: String,
    isbn: String,
}

impl Book {
    //コンストラクタ
    fn new(name: String, author: String, isbn: String) -> Book {
        Book {
            name,
            author,
            isbn,
        }
    }

    //自身が保持する情報を所定のフォーマットにしたがって標準出力するメソッド
    fn print_book(self, start_text: String, end_text: String) {
        println!(
            "{}書籍名: {}, 著者名: {}, ISBN: {} {}",
            start_text, self.name, self.author, self.isbn, end_text
        );
    }
}

fn main() {
    let book = Book::new (
        "Rust入門".to_string(),
        "著者太郎".to_string(),
        "978-4-7981-5757-5".to_string(),
        );

    book.print_book("[".to_string(), "]".to_string());
}
