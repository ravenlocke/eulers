const SOLUTION: u64 = crabtime::eval!(
    (1u64..1_000)
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .sum::<u64>()
);

pub fn solution() -> u64 {
    SOLUTION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 233_168)
    }
}
