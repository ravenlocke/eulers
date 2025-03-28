use num_bigint::BigUint;
use std::collections::HashSet;

pub fn solution() -> u64 {
    let mut set = HashSet::new();

    for a in 2u64..101 {
        for b in 2u32..101 {
            let base = BigUint::from(a);
            set.insert(base.pow(b));
        }
    }
    set.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 9_183)
    }
}
