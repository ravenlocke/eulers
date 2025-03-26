use num_bigint::BigUint;

pub fn solution() -> u64 {
    (2u64..100)
        .map(|i| BigUint::from(i))
        .product::<BigUint>()
        .to_str_radix(10)
        .bytes()
        .map(|i| (i - b'0') as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 648)
    }
}
