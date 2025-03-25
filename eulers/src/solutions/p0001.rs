use crate::helpers::fizzbuzz::FizzBuzzIter;

pub fn solution() -> u64 {
    FizzBuzzIter::default().take_while(|i| *i < 1_000).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 233_168)
    }
}
