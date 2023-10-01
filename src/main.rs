use std::{collections::HashMap, default, fmt, hash::Hash};
use sha2::{Sha256, Digest};

// hash function to hash the address
pub fn hash(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[derive(Debug, Default)]
struct Node {
    leaf: bool, // provides a guard to distinguish between leaf and inner nodes
    remaining_hash: Vec<u8>,
    children: HashMap<u8,Node>,
}

#[derive(Debug, Default)]
struct Trie {
    root: Node,
}

impl Node {

    // conventional 'constructor of Node structure'
    fn new() -> Self {
        Node {
            leaf: false,
            remaining_hash: Vec::new(),
            children: HashMap::new(), 
        }
    }

    // will need to be preceeded by a check to make sure the key being searched exists 
    fn _display_children(&self) {
        let cur = &self.children;
        println!("children of node: [ ");
        for c in cur.keys() {
            print!("{c}, ");
        }
        println!(" ]");
    }

    // will be used for debugging purposes to see how nodes can be modularly switched from inner nodes to leaf nodes
    fn _display_remaining_hash(&self) {
        let cur = &self.remaining_hash;
        print!("remaining hash of node: [ ");
        print!("{:?}", cur);
        println!(" ]");
    }

    fn is_leaf(&self) -> bool {
        self.leaf.clone() //TODO: Consider if clone is necessary
    }

}

impl Trie {
    
    // Conventional 'constructor' for Trie structure
    fn new() -> Self {
        Trie {
            root: Node::new()
        }
    }

    // insert method to place a new node into the trie
    fn insert(&mut self, hash: &str) {
        let mut current_node = &mut self.root;

        for c in text.chars() {
            // entry() api allows us to look into a collection and return a mutable pointer if the element we are searching for exists, otherwise enter an empty hashmap(which is the .or_default part)
            // this line below will continually create new nodes down the trie if c does not exist until c is found or the end of text is reached
            current_node = current_node.children.entry(c).or_default(); 
        }

        current_node.leaf = true;
    }

    fn contains(&self, text: Vec<u8>) -> bool {
        let mut current_node = &self.root;

        for c in text {
            match current_node.children.get(&c) {   // Look for c in the children of the current node
                Some(node) => current_node = node, // Great, we found it! Replace our current node with the node we found
                None => return false,
            }
        }

        // if we are not at the end of a word, even if we have the data available that we searched for, we haven't inserted, that specific text before
        current_node.leaf == true
    }
}

fn main() {

    let mut items = Trie::new();
    println!("{items:?}");

    items.insert("apple");
    items.insert("appetizer");
    items.insert("append");
    items.insert("application");

    let contains_stub = items.contains("app");
    println!("{contains_stub}");
    println!("Len = {trie_len}");

}