use crate::helpers::prime::PrimeFactorisation;

const N: u64 = 600_851_475_143;

pub fn solution() -> u64 {
    PrimeFactorisation::new(N as usize)
        .expect(&format!("could not generate prime factorisation for `{N}`"))
        .largest_prime_factor()
        .expect("could not determine largest prime factor") as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 6_857)
    }
}
