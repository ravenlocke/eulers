use crate::helpers::prime::PrimeFactorisation;

pub fn solution() -> u64 {
    (2..21)
        .map(|i| {
            PrimeFactorisation::new(i)
                .unwrap_or_else(|| panic!("could not generate prime factorisation for `{i}`"))
        })
        .fold(
            PrimeFactorisation::new(1)
                .unwrap_or_else(|| panic!("could not generate prime factorisation for `1`")),
            |a, b| a.lcm(&b),
        )
        .resolve() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 232_792_560)
    }
}
