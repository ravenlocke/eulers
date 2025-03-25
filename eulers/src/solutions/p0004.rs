pub fn is_palindromic(n: u64) -> bool {
    let n_str = n.to_string();

    n_str
        .chars()
        .zip(n_str.chars().rev())
        .take(n_str.len() / 2)
        .all(|(i, j)| i == j)
}

pub fn solution() -> u64 {
    let mut hpp: u64 = 0;

    for i in 100..1000 {
        for j in i..1_000 {
            let product = i * j;
            if product <= hpp {
                continue;
            }

            if is_palindromic(product) {
                hpp = product
            }
        }
    }

    hpp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 906_609)
    }
}
