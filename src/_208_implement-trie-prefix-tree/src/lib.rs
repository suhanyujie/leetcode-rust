//! 题目标题：208. 实现 Trie (前缀树)
//! 题目地址：https://leetcode-cn.com/problems/implement-trie-prefix-tree

use std::collections::HashMap;

struct Trie {
    root: Option<Box<Node>>,
}

struct Node {
    c: char,
    children: Option<Box<Vec<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie { root: None }
    }

    /** Inserts a word into the trie. */
    fn insert(&self, word: String) {
        let root = &self.root;
        let word_list: Vec<char> = word.chars().collect();
        if root.is_none() {
            self.insert_char(c);
        } else {
        }
    }

    fn insert_char(node: &Node, c: char) {}

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {}

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {}
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "hello world".into();
        let obj = Trie::new();
        obj.insert(word);
        let ret_2: bool = obj.search(word);
        let ret_3: bool = obj.starts_with("hell".into());
    }
}
