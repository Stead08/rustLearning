use std::time::{SystemTime, UNIX_EPOCH};

static mut SEED: u32 = 0;

//start から endの乱数を生成するunsafeな関数

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    //必要ならSEEDを初期化
    if SEED == 0 {
        //現在時刻を利用して乱数を初期化
        let epoc = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32
    }
    //シードを利用して乱数を生成
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    SEED % (end - start + 1) + start
}
fn main() {
    //100個の乱数を表示
    for _ in 0..100 {
        //安全でないことを明示
        unsafe {
            //乱数を生成して表示
            let v = rand_global(1, 6);
            println!("{}", v)
        }
    }
}
