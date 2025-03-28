use crate::helpers::prime::sieve_of_eratosthenes;

fn is_prime(n: i64) -> bool {
    // Negatives are not prime; 2 is lowest positive prime.
    if n < 2 {
        return false;
    }
    let n: usize = n.try_into().unwrap();
    let possible_factors = sieve_of_eratosthenes(n.isqrt() + 1);
    !possible_factors.iter().any(|i| n % *i == 0)
}

pub fn solution() -> i64 {
    let mut highest_prod = 0;
    let mut highest_n_consec = 0;

    for a in -999_i64..1_000 {
        for b in -1_000..1_001 {
            let mut counter = 0;
            let n_primes = loop {
                let result = counter * counter + a * counter + b;
                if !is_prime(result) {
                    break counter;
                }
                counter += 1;
            };

            if n_primes > highest_n_consec {
                (highest_prod, highest_n_consec) = (a * b, n_primes)
            }
        }
    }

    highest_prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), -59_231)
    }
}
