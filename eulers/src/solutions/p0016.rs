use num_bigint::BigUint;

pub fn solution() -> u64 {
    BigUint::from(2u64)
        .pow(1_000)
        .to_str_radix(10)
        .bytes()
        .map(|i| (i - b'0') as u64)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 1_366)
    }
}
