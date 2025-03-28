pub fn is_sum_of_fifth_digit_powers(n: u64) -> bool {
    let mut digits = n;
    let mut total = 0;
    while digits != 0 {
        total += (digits % 10).pow(5);
        digits /= 10
    }
    total == n
}

pub fn solution() -> u64 {
    (10..1_000_000)
        .filter(|n| is_sum_of_fifth_digit_powers(*n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 443_839)
    }
}
