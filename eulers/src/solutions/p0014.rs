fn collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        return n / 2;
    } else {
        return n * 3 + 1;
    }
}

fn collatz_length(mut n: u64) -> usize {
    let mut count = 0;

    while n != 1 {
        n = collatz(n);
        count += 1;
    }

    count
}

pub fn solution() -> u64 {
    (1..1_000_000)
        .map(|i| (i, collatz_length(i)))
        .max_by(|(_, xlen), (_, ylen)| xlen.cmp(ylen))
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 837_799)
    }
}
