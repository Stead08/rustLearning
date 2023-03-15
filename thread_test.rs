use std::{thread, time};
fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: {}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    //スレッドなしの場合
    println!("---スレッドなし---");
    sleep_print("スレッドなし");

    //スリープを使う場合
    println!("---スレッドを利用---");
    //スレッド1
    thread::spawn(|| {
        sleep_print("二郎")
    });

    //スレッド2
    thread::spawn(|| {
        sleep_print("三郎")
    });

    //メインスレッド
    sleep_print("太郎")
}
