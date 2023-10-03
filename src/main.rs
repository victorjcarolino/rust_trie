use rust_trie::{Node, Trie, hash};

fn main() {

    let mut items = Trie, hash::new();
    println!("{items:?}");

    items.insert("apple");
    items.insert("appetizer");
    items.insert("append");
    items.insert("application");

    let contains_stub = items.contains("app");
    println!("{contains_stub}");
    println!("Len = {trie_len}");

}