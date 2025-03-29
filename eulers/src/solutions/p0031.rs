const COINS: [u64; 7] = [200, 100, 50, 20, 10, 5, 2];

fn get_n_solutions(current: u64, coins: &[u64]) -> u64 {
    if coins.is_empty() {
        return 1;
    }

    (0..(200 - current) / coins[0] + 1)
        .map(|i| get_n_solutions(current + i * coins[0], &coins[1..]))
        .sum()
}

pub fn solution() -> u64 {
    get_n_solutions(0, &COINS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 73_682)
    }
}
