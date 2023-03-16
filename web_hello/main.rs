use actix_web::{web, App, HttpServer, HttpRequest};

//アドレスとポートの指定
const SERVER_ADDR: &str = "127.0.0.1:3000";

//Actix_webのメイン関数
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("[SERVER] http://{}/", SERVER_ADDR);
    //httpサーバを起動
    HttpServer::new(|| {
        //ルーティングを指定
        App::new()
            .route("/", web::get().to(index))
    })
        .bind(SERVER_ADDR)?
        .run()
        .await
}

//実行される関数
async fn index(req: HttpRequest) -> &'static str {
    println!("request: {:?}", req);
    "Hello, World!"
}
