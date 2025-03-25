use num_integer::binomial;

pub fn solution() -> u64 {
    const N: u64 = 20 + 20;
    const K: u64 = 20;

    binomial(N, K)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 137_846_528_820)
    }
}
