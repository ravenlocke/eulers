use num_bigint::BigUint;

pub fn solution() -> u64 {
    let mut counter = 0;

    let mut a = BigUint::from(0u64);
    let mut b = BigUint::from(1u64);

    let threshold = BigUint::from(10u64).pow(999);

    while a < threshold {
        a += &b;
        std::mem::swap(&mut a, &mut b);
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 4_782)
    }
}
