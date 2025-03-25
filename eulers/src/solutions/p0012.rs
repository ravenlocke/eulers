use std::ops::AddAssign;

use crate::helpers::prime::PrimeFactorisation;
use num_integer::Integer;

#[derive(Default)]
struct TriangleNumberIter<T> {
    current: T,
    counter: T,
}

impl<T: AddAssign<T> + Integer + Copy> Iterator for TriangleNumberIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += T::one();
        self.current += self.counter;
        Some(self.current)
    }
}

pub fn solution() -> u64 {
    const TARGET_N_FACTORS: usize = 500;

    for i in TriangleNumberIter::default() {
        let factorisation = PrimeFactorisation::new(i)
            .expect(&format!("could not generate prime factorisation for `{i}`"));
        if factorisation.n_factors() > TARGET_N_FACTORS {
            return i as u64;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 76_576_500)
    }
}
