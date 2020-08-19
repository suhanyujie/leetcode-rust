pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    (1..s).fold(1, |acc: u64, _| acc + acc)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, square(1));
        assert_eq!(2, square(2));
        assert_eq!(4, square(3));
        assert_eq!(8, square(4));
        assert_eq!(16, square(5));
        assert_eq!(32, square(6));
        assert_eq!(64, square(7));
        assert_eq!(128, square(8));
    }
}
