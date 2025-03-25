pub fn solution() -> u64 {
    'outer: for a in 1..1_001 {
        for b in (a + 1)..(1001 - a) {
            let c = 1000 - (a + b);
            if c <= b {
                continue 'outer;
            }

            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 31_875_000)
    }
}
