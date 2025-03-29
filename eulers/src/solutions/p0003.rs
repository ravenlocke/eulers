const SOLUTION: u64 = crabtime::eval! {
    let mut n = 600_851_475_143;
    let mut result: u64 = 1;

    while n != 1 {
        result += 2;
        while n % result == 0 {
            n /= result;
        }
    }

    result
};

pub fn solution() -> u64 {
    SOLUTION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 6_857)
    }
}
