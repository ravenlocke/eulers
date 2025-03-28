pub fn solution() -> usize {
    let mut max = 0;
    let mut max_len = 0;

    for i in 2..1_000 {
        let mut v: Vec<usize> = Vec::new();
        let mut n = 1;
        let maybe_len = loop {
            n *= 10;
            if let Some(i) = v.iter().position(|i| *i == n) {
                break Some(v.len() - i);
            } else {
                if n % i == 0 {
                    break None;
                }
                v.push(n);
                n %= i;
            }
        };

        if let Some(len) = maybe_len {
            if len > max_len {
                (max, max_len) = (i, len);
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 983)
    }
}
