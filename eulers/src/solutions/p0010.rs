use crate::helpers::prime::sieve_of_eratosthenes;

pub fn solution() -> u64 {
    sieve_of_eratosthenes(1_999_999).iter().sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 142_913_828_922)
    }
}
