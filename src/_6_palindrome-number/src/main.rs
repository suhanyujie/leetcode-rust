
#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x: i32 = x;
        if x < 0 || (x!=0 && x % 10 == 0) {
            return false;
        }
        let mut revertedNumber: i32 = 0;
        while x > revertedNumber {
            revertedNumber = revertedNumber*10 + x%10;
            x /= 10;
        }
        x == revertedNumber || x == revertedNumber/10
    }
}

#[test]
fn test_one() {
    let num: i32 = 12323;
    assert_eq!(Solution::is_palindrome(num), false);
}

#[test]
fn test_nagtive_number() {
    let num: i32 = -12321;
    assert_eq!(Solution::is_palindrome(num), false);
}

#[test]
fn test_right_number_1() {
    let num: i32 = 12321;
    assert_eq!(Solution::is_palindrome(num), true);
}

#[test]
fn test_right_number_2() {
    let num: i32 = 112211;
    assert_eq!(Solution::is_palindrome(num), true);
}

fn main() {
    println!("Hello, world!");
}
