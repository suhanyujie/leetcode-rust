pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result: Vec<u32> = vec![];
    // 必须小于 limit，不可等于
    for cur_num in 1..limit {
        for num in factors {
            if *num > 0 && cur_num % num == 0 {
                result.push(cur_num);
                break;
            }
        }
    }

    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            sum_of_multiples(4, &[3,5])
        );
    }
}

