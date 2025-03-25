pub fn solution() -> u64 {
    (1..101).sum::<u64>().pow(2) - (1..101).map(|i| i * i).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 25_164_150)
    }
}
