//! 题目标题：208. 实现 Trie (前缀树)
//! 题目地址：https://leetcode-cn.com/problems/implement-trie-prefix-tree

use std::collections::HashMap;

/// 一开始的想法是使用 HashMap，可是将数据节点包装成 HashMap<char, Option<Box<Vec<Node>>>> 处理起来会比较繁琐，
/// 所以打算看一下[题解](https://leetcode-cn.com/problems/implement-trie-prefix-tree/solution/rust-shi-xian-qian-zhui-shu-trie-by-dxmq-e4xl/)，找找其他思路。
/// 按照题解中的解法有局限性，字符串中只允许 a-zA-Z，而不允许其他字符串出现。
#[derive(Default)]
struct Trie {
    child: [Option<Box<Trie>>; 26],
    is_end: bool,
}

/// TriePlus 是我在 Trie 结构的基础上的进一步实现，由于 Trie 只支持 ASCII 字符，而不支持其他字符
/// 因此，TriePlus 是 Trie 的改进版。
#[derive(Default)]
struct TriePlus {
    entry: HashMap<char, Option<Box<Vec<Node>>>>,
}

struct Node {
    c: char,
    children: Option<Box<Vec<Node>>>,
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for i in word.chars().map(|c| {
            let v1 = c as u8 - 'a' as u8;
            println!("{}", v1);
            v1 as usize
        }) {
            cur = cur.child[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        cur.is_end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for i in word.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            let res = cur.child[i].as_ref();
            match res {
                Some(node) => cur = node,
                None => return false,
            }
        }
        cur.is_end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for i in prefix.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            let res = cur.child[i].as_ref();
            match res {
                Some(node) => cur = node,
                None => return false,
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

impl TriePlus {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        // let mut cur = self;
        // for i in word.chars().map(|c| {
        //     let v1 = c as u8 - 'a' as u8;
        //     println!("{}", v1);
        //     v1 as usize
        // }) {
        //     cur = cur.child[i].get_or_insert_with(|| Box::new(Trie::new()));
        // }
        // cur.is_end = true;
        let mut cur = self;
        for c in word.chars().map(|c| c) {
            cur.char
        }
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for i in word.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            let res = cur.child[i].as_ref();
            match res {
                Some(node) => cur = node,
                None => return false,
            }
        }
        cur.is_end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for i in prefix.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            let res = cur.child[i].as_ref();
            match res {
                Some(node) => cur = node,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "hellowworld".into();
        let mut obj = Trie::default();
        obj.insert(word);
        assert_eq!(obj.starts_with("hello".into()), true);
        assert_eq!(obj.starts_with("hellow".into()), true);
        assert_eq!(obj.starts_with("hellowa".into()), false);
    }

    #[test]
    fn test_str1() {
        let s1: String = "hello".into();
        let b1: Vec<char> = s1.chars().collect();
        let mut d1 = Some(Box::new('h'));
        let d2 = d1.get_or_insert_with(|| Box::new('a'));
        println!("{:?}", d2);
    }
}
