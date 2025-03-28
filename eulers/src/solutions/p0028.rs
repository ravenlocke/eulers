pub fn solution() -> usize {
    const N: usize = 1_001 * 1_001;

    let mut total = 1;
    let mut current = 1;
    let mut delta = 2;

    while current != N {
        for _ in 0..4 {
            current += delta;
            total += current;
        }
        delta += 2
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 669_171_001)
    }
}
