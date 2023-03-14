use image::{self, imageops, GenericImageView, Rgba, GenericImage};

fn main() {
    //リサイズするサイズ
    let size = 128;
    //コマンドライン引数を得る
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_thumb imagefile");
        return
    }
    //入力ファイルと出力ファイルを指定
    let infile = String::from(&args[1]);
    let outfile = format!("{}-thumb.png", infile);
    println!("input: {}", infile);
    println!("output: {}", outfile);
    //画像ファイルを読み込む
    let mut img = image::open(infile)
        .expect("ファイルを読み込めません");
    //画像の幅と高さを得る
    let (w, h) = img.dimensions();
    // 行と列をそれぞれ繰り返す
    for y in 0..h {
        for x in 0..w {
            //ピクセルデータを得る
            let c: Rgba<u8> = img.get_pixel(x, y);
            //ネガポジ反転処理
            let c = Rgba([
                255 - c[0],
                255 - c[1],
                255 - c[2],
                c[3]
            ]);
            //ピクセルを指定
            img.put_pixel(x, y, c);
        }
    }
    // 画像を保存
    img.save(outfile).unwrap();
}
