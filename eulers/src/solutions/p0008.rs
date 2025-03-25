use std::{collections::vec_deque::VecDeque, usize};

const INPUTS: &str = "
73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";

fn solve(string: &str, n_consec: usize) -> Option<u64> {
    let mut zero_count = 0;
    let mut curr_prod = 1;
    let mut high_prod = None;
    let mut v: VecDeque<u64> = VecDeque::with_capacity(n_consec);

    string
        .chars()
        .filter_map(|i| if i == '\n' { None } else { Some(i as u64 - 48) })
        .for_each(|i| {
            if v.len() == n_consec {
                let out = v.pop_front().expect("VecDeque was empty");
                if out == 0 {
                    zero_count -= 1
                } else {
                    curr_prod /= out
                }
            };

            v.push_back(i);
            if i == 0 {
                zero_count += 1
            } else {
                curr_prod *= i;
            }

            if v.len() == n_consec {
                match zero_count {
                    0 => {
                        if high_prod.is_none_or(|i| i < curr_prod) {
                            high_prod = Some(curr_prod)
                        }
                    }
                    _ => {
                        if high_prod.is_none() {
                            high_prod = Some(0)
                        }
                    }
                }
            }
        });

    high_prod
}

pub fn solution() -> u64 {
    const N_CONSEC: usize = 13;

    solve(INPUTS, N_CONSEC).expect("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 23_514_624_000)
    }

    macro_rules! solve_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (string, n_consec, expected) = $value;
                assert_eq!(solve(string, n_consec), expected);
            }
        )*
        }
    }

    solve_tests! {
        test_returns_expected_result_when_str_len_equals_n_consec: ("1234", 4, Some(24)),
        test_returns_expected_result_when_highest_product_is_at_end: ("12345", 4, Some(120)),
        test_returns_expected_result_when_highest_product_is_at_start: ("54321", 4, Some(120)),
        test_returns_expected_result_if_highest_product_occurs_before_n_consec_reached: ("99901234", 4, Some(24)),
        test_returns_none_if_str_len_is_less_than_n_consec: ("123", 4, None),
        test_returns_some_zero_if_highest_product_is_zero: ("0123", 4, Some(0)),
    }
}
