// Solve during compilation for speed.
const SOLUTION: u64 = crabtime::eval! {
    let mut a = 0u64;
    let mut b = 1;
    let mut total = 0;

    while a < 4_000_000 {
        if a % 2 == 0 {
            total += a
        }

        a += b;
        std::mem::swap(&mut a, &mut b);
    }

    total
};

pub fn solution() -> u64 {
    SOLUTION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 4_613_732)
    }
}
