use std::{collections::HashMap, fs::File, io::prelude::*, env, iter};
use sha2::{Sha256, Digest};

// hash function to hash the address
pub fn hash(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

// function to parse the cmd line arguments
pub fn parse_command_line() {

    
}

// transfer each line of addresses to the buffers
pub fn file_to_buf(filename: String) {
    println!("Opening {}", &filename);
    let mut file = File::open(filename).expect("Error opening file");
    let mut raw_contents = String::new();
}

// will be used to collect the information from the files that we will need to insert into the trie
#[derive(Debug, Default)]
struct Buffer {
    address: Vec<u8>,
    balance: Vec<u8>,
}

#[derive(Debug, Default)]
struct Node {
    leaf: bool, // provides a guard to distinguish between leaf and inner nodes
    remaining_hash: Vec<u8>,
    value : String,
    children: HashMap<u8,Node>,
}

#[derive(Debug, Default)]
struct Trie {
    root: Node,
}

impl Buffer {
    // conventional 'constructor' of InfoCollector structure
    fn new(_address: Vec<u8>, _balance: Vec<u8>) -> Self {
        Buffer {
            address: hash(_address),
            balance: _balance,
        }
    }
}

impl Node {

    // conventional 'constructor of Node structure'
    fn new(key : Vec<u8>, val : String) -> Self {
        Node {
            leaf: false,
            remaining_hash: key,
            value: val,
            children: HashMap::new(), 
        }
    }

    // constructor with no args
    fn new_no_params() -> Self {
        Node {
            leaf: true,
            remaining_hash: Vec::new(),
            value: String::new(),
            children: HashMap::new(),
        }
    }

    // will need to be preceeded by a check to make sure the key being searched exists 
    fn display_children(&self) {
        let cur = &self.children;
        println!("children of node: [ ");
        for c in cur.keys() {
            print!("{c}, ");
        }
        println!(" ]");
    }

    // will be used to check if the current node has children
    fn has_children(&self) -> bool{
        let cur = &self.children;
        !cur.is_empty()
    }

    // will be used for debugging purposes to see how nodes can be modularly switched from inner nodes to leaf nodes
    fn display_remaining_hash(&self) {
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
            root: Node::new_no_params()
        }
    }

    // insert method to place a new node into the trie
    fn insert(&mut self, mut inserted_node: Node) {
        let mut current_node = &mut self.root;

        for (i,c) in inserted_node.remaining_hash.enumerate() {
            // entry() api allows us to look into a collection and return a mutable pointer if the element we are searching for exists, otherwise enter an empty hashmap(which is the .or_default part)
            // If the current node contains the inserting key, go into the child with the key
            if (current_node.children.contains_key(&c)) { //accessing the child at c
                // Check if the child with the key is a leaf node, 
                if (current_node.is_leaf()) {
                    // if it is a leaf node, take (the remaining hash of the current leaf node) and (the current value of the leaf node) and put them in a temp_node
                    let cur_rem_hash = &current_node.remaining_hash[c..];
                    let cur_value = current_node.value;
                    // make the current_node's leaf status false
                    // check to see how many hashes x will need to be traversed such that temp_node.remaining_hash[i] != inserted_node.remaining_hash[i]
                    // in a loop insert into the Hashmap of current_node.children x keys(from remaining_hash) and the value from temp_node, making sure to traverse down the tree with current_node
                    // mark the temp_node as a leaf
                    // insert into the Hashmap of current_node.children the key(from remaining hash) and value from inserted_node
                }
                // if it is not a leaf node, keep continue looping
            } 
            // If the current node does not contain the inserting key, insert the 
            else {

            }
            
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

#[cfg(test)]
mod tests {
    use super::*;

    // test to see if contains works correctly
        // 1. searching for an existing node
        // 2. searching for a non-existant node
    // test to see if insertion works correctly
        // 1. search for node that was inserted
        // 2. search to see if you can insert two identical nodes (shouldn't happen becuase of the hashing of the address)
}