//! 题目标题：208. 实现 Trie (前缀树)
//! 题目地址：https://leetcode-cn.com/problems/implement-trie-prefix-tree

use std::collections::HashMap;

struct Trie {
    root: HashMap<char, Option<Box<Vec<Node>>>>,
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
        let m: HashMap<char, Option<Box<Vec<Node>>>> = HashMap::new();
        Trie { root: m }
    }

    /** Inserts a word into the trie. */
    fn insert(&self, word: String) {
        let root = &self.root;
        let word_list: Vec<char> = word.chars().collect();
        let mut cur_node = root;
        for c in &word_list {
            let tmp_node = cur_node.get(c);
            if tmp_node.is_none() {
                cur_node[c] = Self::insert_char(&mut cur_node, &word_list[1..=word_list.len()])
            }
        }
    }

    fn insert_char(
        node: &mut HashMap<char, Option<Box<Vec<Node>>>>,
        c: &[char],
    ) -> Option<Box<Vec<Node>>> {
        None
    }

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
