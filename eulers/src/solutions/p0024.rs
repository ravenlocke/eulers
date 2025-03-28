const FACTORIALS: [usize; 10] = [0, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

pub fn solution() -> u64 {
    let mut n = 999_999;
    let mut alphabet = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut string = String::with_capacity(alphabet.len());
    while alphabet.len() != 1 {
        string.push(alphabet.remove(n / FACTORIALS[alphabet.len() - 1]));
        n %= FACTORIALS[alphabet.len()]
    }

    // Unwrap because we couldn't get here unless one item was left in the vec.
    string.push(alphabet.pop().unwrap());

    string
        .parse()
        .unwrap_or_else(|_| panic!("could not parse `{string}` as u64"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 2_783_915_460)
    }
}
