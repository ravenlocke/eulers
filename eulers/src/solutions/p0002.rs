use crate::helpers::fibonacci::Fibonacci;

pub fn solution() -> u64 {
    Fibonacci::default()
        .skip(1)
        .step_by(3)
        .take_while(|i| *i < 4_000_000)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 4_613_732)
    }
}
