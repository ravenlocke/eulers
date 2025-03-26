use std::collections::HashMap;

use crate::helpers::prime::PrimeFactorisation;

pub fn solution() -> u64 {
    let map: HashMap<usize, usize> = (1..10_000)
        .map(|i| {
            let x = PrimeFactorisation::new(i).unwrap().divisors();
            (i, x[..x.len() - 1].iter().sum())
        })
        .collect();

    map.iter()
        .filter_map(|(n, proper_div_sum)| {
            if n == proper_div_sum {
                return None;
            }

            if let Some(m) = map.get(proper_div_sum) {
                if m == n { Some(n) } else { None }
            } else {
                None
            }
        })
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 31_626)
    }
}
