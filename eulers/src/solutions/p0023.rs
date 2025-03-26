use std::collections::HashSet;

use crate::helpers::prime::PrimeFactorisation;

fn is_abundant(n: usize) -> bool {
    PrimeFactorisation::new(n)
        .unwrap()
        .divisors()
        .iter()
        .rev()
        .skip(1)
        .sum::<usize>()
        > n
}

pub fn solution() -> u64 {
    let abundant_nums = (12..28_123).filter(|i| is_abundant(*i)).collect::<Vec<_>>();
    let mut sum_two_abundant = (1..28_124).collect::<HashSet<_>>();
    'outer: for (idx, i) in abundant_nums.iter().enumerate() {
        for j in &abundant_nums[idx..] {
            let sum = i + j;
            if sum > 28_123 {
                continue 'outer;
            }
            sum_two_abundant.remove(&sum);
        }
    }

    sum_two_abundant.iter().sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 4_179_871)
    }

    #[test]
    fn test_is_abundant_returns_correct_result() {
        (1..12).for_each(|i| assert_eq!(is_abundant(i), false));
        assert_eq!(is_abundant(12), true);
    }
}
