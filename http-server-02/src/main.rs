use std::fs;
use tokio::{
    fs::read_to_string,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream}
};

async fn start() -> std::io::Result<()> {
    // TCPソケットを特定のアドレスを結びつけたTcpListener
    let listener = TcpListener::bind("0.0.0.0:9999").await?;
    //リスナーで接続を待ち受け、やってきた場合は受け付ける
    while let Ok((stream, _)) = listener.accept().await {
        //スレッドを立ち上げ、スレッド内部でハンドリングする
        tokio::spawn(handler(stream));
    }
    Ok(())
}

async fn handler(mut stream: TcpStream) -> std::io::Result<()> {
    //用意したバッファーにストリームを読み込む
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;
    //許容するリクエストの文字列
    let get = b"GET / HTTP/1.1\r\n";

    //レスポンスを用意する
    //bufferの内容がget
    //bufferの内容がgetで定義した文字列から始まるかどうかを確認する
    let response = if buffer.starts_with(get) {
        //Http/1.1で200OKを出す
        let status_line = "HTTP/1.1 200 OK";
        //Htmlファイルを読み込む
        let html = read_to_string("index.html").await?;
        //Content-LengthやContent-typeなどのHTTPヘッダーを用意する
        let http_header = format!(
            "Content-Length: {}\r\nContent-Type: text/html;charset=UTF-8",
            html.len()
        );
        format!("{}\r\n{}\r\n\r\n{}", status_line, http_header, html)
    } else {
        // GET以外のメソッドやルート宛以外の場合は404 Not Foundを返す
        "HTTP/1.1 404 NotFound\r\n\r\n".to_string()
    };

    // 用意したレスポンスをストリームに書き出す
    stream.write_all(response.as_bytes()).await?;
    //streamをフラッシュする
    stream.flush().await

}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    start().await
}
