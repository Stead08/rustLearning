fn print_content<T: std::fmt::Display>(content: T) {
    println!("{}", content);
}

fn main() {
    print_content(42);
    print_content("Hello, world!");
    print_content(42.0)
}
