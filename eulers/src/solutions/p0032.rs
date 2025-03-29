use std::collections::HashSet;

pub fn is_pandigital(a: u64, b: u64, prod: u64) -> bool {
    let mut digits = Vec::new();

    for mut num in [a, b, prod] {
        while num != 0 {
            let rem = num % 10;
            if rem == 0 {
                return false;
            }
            if digits.contains(&rem) {
                return false;
            }
            digits.push(rem);
            num /= 10;
        }
    }

    digits.len() == 9
}

pub fn solution() -> u64 {
    let mut a: u64 = 1;
    let mut products = HashSet::new();

    'outer: loop {
        a += 1;
        let a_digits = a.ilog10();
        for b in a + 1.. {
            let prod = a * b;

            let n_digits = a_digits + b.ilog10() + prod.ilog10() + 3;

            match n_digits.cmp(&9) {
                std::cmp::Ordering::Less => {
                    continue;
                }
                std::cmp::Ordering::Greater => {
                    if b == a + 1 {
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                }
                std::cmp::Ordering::Equal => {
                    if is_pandigital(a, b, prod) {
                        products.insert(prod);
                    }
                }
            }
        }
    }

    products.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 45_228)
    }
}
