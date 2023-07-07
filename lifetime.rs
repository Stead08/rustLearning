use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2.as_str(),
        321);
    println!("{}", result);

}

fn longest<'a>(a: &'a str, b: &str) -> &'a str {
    a
}

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display {
    // "アナウンス"　{}
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
