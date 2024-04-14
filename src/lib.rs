#![allow(non_snake_case)]

use std::collections::{hash_map, HashMap};

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

    pub fn add (&mut self, characterArray: &Vec<char>) {
        if characterArray.len() == 0 {
            return;
        }
        let firstChar = characterArray[0];
        match self.children.entry(firstChar) {
            hash_map::Entry::Vacant(entry) => {
                let insertedVal = entry.insert(Box::new(Node {
                    children: HashMap::new(),
                    is_word: characterArray.len() == 1
                }));
                insertedVal.add(&characterArray[1..].to_vec());
            },
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().add(&characterArray[1..].to_vec())
            }
        }

    }

    pub fn search (&self, characterArray: &Vec<char>) -> bool {
        let firstChar = characterArray[0];
        match self.children.get(&firstChar) {
            None => false,
            Some(entry) => {
                if characterArray.len() == 1 {
                    return entry.is_word;
                }
                entry.search(&characterArray[1..].to_vec())
            }
        }
    }
}