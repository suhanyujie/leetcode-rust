pub fn nth(n: u32) -> u32 {
    let mut prime_count = 0;
    let mut tmp_num = 2;
    let mut prime_arr: Vec<u32> = vec![];
    loop {
        if is_prime(tmp_num) {
            prime_arr.push(tmp_num);
            prime_count += 1;
        }
        if prime_count > n {
            break;
        }
        tmp_num += 1;
    }
    if n == 0 {
        return prime_arr[0];
    }
    prime_arr[n as usize]
}

pub fn is_prime(num: u32) -> bool {
    // 1 不是素数
    if num <= 1 {
        return false;
    }
    // 2,3 是素数
    if num <= 3 {
        return true;
    }
    // clippy 规范是 num 不会出现大于 MAX 的情况，所以直接用 == 
    // 详见 https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons
    if num == std::u32::MAX {
        return false;
    }
    let mut is_prime = true;
    for n in 2..(num - 1) {
        if num % n == 0 {
            is_prime = false;
            break;
        }
    }
    is_prime
}

#[test]
fn test_nth() {
    let num = nth(0);
    assert!(num == 2);
    let num = nth(6);
    assert!(num == 17);
}

#[test]
fn test_is_prime_1() {
    let is_prime = is_prime(4);
    println!("{}", is_prime);
    assert!(false == is_prime);
}
