use std::collections::HashMap;

/// A representation of a number decomposed into its prime factors.
#[derive(Debug)]
pub struct PrimeFactorisation {
    factorisation: HashMap<usize, usize>,
}

impl PrimeFactorisation {
    pub fn new(mut n: usize) -> Option<Self> {
        let mut factorisation = HashMap::new();
        if n == 0 {
            return None;
        };

        let largest_possible_factor = n.isqrt();
        'outer: for i in sieve_of_eratosthenes(largest_possible_factor) {
            while n % i == 0 {
                n /= i;
                *factorisation.entry(i).or_insert(0) += 1;

                if n == 1 {
                    break 'outer;
                }
            }
        }

        if n != 1 {
            factorisation.insert(n, 1);
        }

        Some(PrimeFactorisation { factorisation })
    }

    pub fn largest_prime_factor(&self) -> Option<usize> {
        self.factorisation.keys().max().copied()
    }

    pub fn lcm(&self, other: &PrimeFactorisation) -> PrimeFactorisation {
        let mut factorisation = HashMap::new();
        self.factorisation
            .keys()
            .chain(other.factorisation.keys())
            .for_each(|k| {
                // We may see some keys twice if they are appear in both factorisations.
                if factorisation.contains_key(k) {
                    return ();
                }
                factorisation.insert(
                    *k,
                    usize::max(
                        self.factorisation.get(k).copied().unwrap_or(0),
                        other.factorisation.get(k).copied().unwrap_or(0),
                    ),
                );
            });

        PrimeFactorisation { factorisation }
    }

    pub fn n_factors(&self) -> usize {
        self.factorisation.values().map(|i| *i + 1).product()
    }

    pub fn resolve(&self) -> usize {
        self.factorisation
            .iter()
            .map(|(k, v)| k.pow(*v as u32))
            .product()
    }

    /// Calculate all divisors of a number.
    pub fn divisors(&self) -> Vec<usize> {
        let mut vec: Vec<usize> = vec![1];
        for (k, v) in self.factorisation.iter() {
            let mut next_vec = Vec::with_capacity(vec.len() * (v + 1));
            for power in 0..*v + 1 {
                next_vec.extend(vec.iter().map(|i| *i * k.pow(power as u32)));
            }

            vec = next_vec
        }
        vec.sort_unstable();
        vec
    }
}

/// Calculate all prime numbers ≤ `n` using the Sieve of Eratosthenes.
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if (0..2).contains(&n) {
        return vec![];
    }

    let mut arr = vec![true; n + 1];
    arr[0] = false;
    arr[1] = false;

    for i in 2..n.isqrt() + 1 {
        if arr[i] {
            (i * i..n + 1).step_by(i).for_each(|idx| arr[idx] = false)
        }
    }

    arr.iter()
        .enumerate()
        .filter_map(|(idx, val)| if *val { Some(idx) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes_returns_correct_result_for_square() {
        assert_eq!(sieve_of_eratosthenes(9), vec![2, 3, 5, 7])
    }

    #[test]
    fn test_sieve_of_eratosthenes_includes_n_if_n_is_prime() {
        assert_eq!(sieve_of_eratosthenes(7), vec![2, 3, 5, 7])
    }

    #[test]
    fn test_sieve_returns_emtpy_list_if_n_less_than_two() {
        assert_eq!(sieve_of_eratosthenes(0), vec![]);
        assert_eq!(sieve_of_eratosthenes(1), vec![]);
    }
}
