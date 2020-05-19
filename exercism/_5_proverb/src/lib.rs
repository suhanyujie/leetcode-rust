//! 组装谚语
// 地址 https://exercism.io/my/solutions/ce0a318b55c844d1b31aabc141e85112

pub fn build_proverb1(list: &[&str]) -> String {
    let v1 = list;
    let mut output = String::from("");
    for index in 0..v1.len() {
        let next_label_op = v1.get(index + 1);
        if let Some(next_label) = next_label_op {
            let cur_label = v1.get(index).unwrap();
            output = format!(
                "{}For want of a {} the {} was lost.\n",
                output, *cur_label, *next_label
            );
        } else {
            output = format!("{}And all for the want of a nail.", output);
        }
    }
    output
}


/// 更好的方案
pub fn build_proverb(list: &[&str]) -> String {
    list.iter()
        .enumerate()
        .fold(String::new(), |acc, (index, v)| match list.get(index + 1) {
            Some(next_label) => {
                format!("{}For want of a {} the {} was lost.\n", acc, v, next_label)
            }
            None => format!("{}And all for the want of a {}.", acc, list[0]),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_proverb() {
        let list = [
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ];
        let str1 = build_proverb(&list);
        println!("{}", str1);
        let res = r#"For want of a nail the shoe was lost.
For want of a shoe the horse was lost.
For want of a horse the rider was lost.
For want of a rider the message was lost.
For want of a message the battle was lost.
For want of a battle the kingdom was lost.
And all for the want of a nail."#;
        assert_eq!(str1, res);
    }
}
