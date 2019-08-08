use std::str;

pub fn reverse(input: &str) -> String {
    let res: Vec<char> = input.chars().rev().collect();
    let mut res_str = String::new();
    for one_char in res {
        res_str.push(one_char);
    }
    res_str
}

pub fn reverse_v2(input: &str) -> String {
    let mut len = input.len();
    let mut char_indices = input.char_indices();
    let mut new_str = String::from("");
    loop {
        if len < 0 {
            break;
        }
        let tmp_indices = char_indices.next();
        if tmp_indices == None {
            break;
        }
        let (count, tmp_char) = tmp_indices.unwrap();
        len -= 1;
        new_str = format!("{}{}", &tmp_char, &new_str);
    }
    println!("{}", &new_str);
    new_str
}

pub fn reverse_v1(input: &str) -> String {
    let str_b = input.as_bytes();
    let mut len = input.len();
    let mut new_bytes: Vec<u8> = vec![0; len];
    for one in str_b {
        new_bytes[len - 1] = *one;
        len -= 1;
    }
    println!("{:#?}", new_bytes);
    let result = str::from_utf8(&new_bytes).unwrap();
    result.to_string()
}

#[test]
fn test_reverse() {
    let str1 = String::from("cool");
    let str1 = reverse(&str1);
    println!("{}", str1);
    assert!(&str1 == "looc");
}

#[test]
fn test_reverse2() {
    let str1 = String::from("你是");
    let str1 = reverse(&str1);
    println!("{}", &str1);
    assert!(&str1 == "是你");
}
