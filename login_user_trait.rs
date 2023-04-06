
struct LoginUser {
    //ユーザーID
    id: String,
    //ユーザー名
    name: String,
}

impl LoginUser {
    //loginUser構造体のコンストラクタ
    //文字列の受け渡しの際にいちいち”to_string"と書かなくて済む
    fn new(id: &str, name: &str) -> Self {
        LoginUser {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for LoginUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, name: {}", self.id, self.name)
    }
}

fn main() {
    let login_user = LoginUser::new("id", "name");
    println!("{}", login_user)
}
