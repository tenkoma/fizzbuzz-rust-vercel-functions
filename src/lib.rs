
pub fn fizzbuzz(num: u64) -> String {
    match (num % 3, num % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => num.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;
    /// 1を渡すと文字列'1'を返す
    #[test]
    fn passing_one_returns_the_string_one() {
        assert_eq!(fizzbuzz(1), String::from("1"));
    }

    /// 2を渡すと文字列'2'を返す
    #[test]
    fn passing_two_returns_the_string_two() {
        assert_eq!(fizzbuzz(2), String::from("2"));
    }

    /// 3を渡すと文字列'Fizz'を返す
    #[test]
    fn passing_three_returns_the_string_fizz() {
        assert_eq!(fizzbuzz(3), String::from("Fizz"));
    }

    /// 5を渡すと文字列'Buzz'を返す
    #[test]
    fn passing_five_returns_the_string_buzz() {
        assert_eq!(fizzbuzz(5), String::from("Buzz"));
    }

    /// 15を渡すと文字列'FizzBuzz'を返す
    #[test]
    fn passing_fifteen_returns_the_string_fizzbuzz() {
        assert_eq!(fizzbuzz(15), String::from("FizzBuzz"));
    }
}
