use rand::Rng;

fn main() {
    //乱数生成機を用意
    let mut rng = rand::thread_rng();
    //5回サイコロをふる
    for _ in 0..5 {
        //1から6までの乱数を生成
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}
