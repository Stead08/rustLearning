use image;
fn main() {
    //黒をRGBで定義
    let black = image::Rgb::<u8>([0, 0, 0]);
    //緑をRGBで定義
    let green = image::Rgb::<u8>([90, 200, 90]);
    //一ますのサイズ
    let w = 64;
    //一松模様を描くクロージャ
    let draw = |x, y| {
        let (xi, yi) = (x / w, y / w);
        match (xi % 2, yi % 2) {
            (0, 0) => black,
            (1, 0) => green,
            (0, 1) => green,
            (1, 1) => black,
            (_, _) => panic!("error"),
        }
    };
    //クロージャを指定してImageBufferを生成
    let img = image::ImageBuffer::from_fn(512, 512, draw);
    //ファイルへ保存
    img.save("image.png").unwrap();
}
