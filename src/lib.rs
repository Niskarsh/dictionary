#![allow(non_snake_case)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    children: HashMap<char, Box<Node>>,
    is_word: bool,
}

impl Node {
    pub fn new () -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }

    pub fn addWord (&mut self, characterArray: &Vec<char>) {

    }
}