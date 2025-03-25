const N_PRIME: usize = 10_001;

pub fn solution() -> u64 {
    let mut primes = Vec::with_capacity(N_PRIME);
    primes.push(2);

    for i in (3..).step_by(2) {
        if !primes.iter().any(|prime| i % prime == 0) {
            primes.push(i);
            if primes.len() == N_PRIME {
                break;
            }
        }
    }

    *primes.last().expect("primes should not be empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 104_743)
    }
}
