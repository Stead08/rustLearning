use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work)
        }
    }
}
fn sort_work(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string()]);
    table.insert("Carabaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Mesuda".to_string(), "a salt celler".to_string()]);

    sort_work(&mut table);
    show(&table);


}
