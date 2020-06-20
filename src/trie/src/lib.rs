use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    root: Box<Node>,
}

#[derive(Debug)]
struct Node {
    is_root_node: bool,
    is_path_end: bool,
    charater: u8,
    children: HashMap<u8, Box<Node>>,
}

impl Trie {
    fn new() -> Trie {
        let root_node = Node::new("0".to_string());
        Trie {
            root: Box::new(root_node),
        }
    }

    // 新增敏感词
    fn add(&mut self, word: String) {
        let mut current = &mut self.root;
        let bytes: Vec<u8> = word.as_bytes().into();
        let mut posi = 0;
        while posi < bytes.len() {
            let r = bytes[posi];
            let mut childs = current.children;
            if let Some(mut b1) = childs.get(&r) {
                // 存在
                unsafe {
                    current = b1;
                }
            } else {
                // 不存在
                let new_bytes = vec![r];
                if let Ok(s) = std::str::from_utf8(&new_bytes) {
                    let new_node = Node::new(s.to_string());
                    childs.insert(r, Box::new(new_node));
                } else {
                    panic!("1");
                }
            }
            posi += 1;
        }
        println!("{:?}", &bytes);
    }
}

impl Node {
    fn new(charater: String) -> Node {
        let charater = charater.as_bytes();
        Node {
            is_root_node: true,
            is_path_end: true,
            charater: *(charater.get(0).unwrap()),
            children: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut tree = Trie::new();
        tree.add("wh".to_string());
        assert!(false);
    }

    #[test]
    fn test_bytes() {
        let s1 = "你好 Rust";
        let b2 = s1.as_bytes();
        println!("{:?}", b2);
        println!("{:?}", std::str::from_utf8(b2));
        assert!(false);
    }

    #[test]
    fn test_new_trie() {
        let t1 = Trie::new();
        assert_eq!(
            t1.root.charater,
            *("0".to_string().as_bytes().get(0).unwrap())
        );
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
