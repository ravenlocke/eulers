const TRIANGLE: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

pub fn solution() -> u64 {
    let mut row_iter = TRIANGLE[1..].lines().map(|i| {
        i.split_whitespace()
            .map(|j| {
                j.parse::<u64>()
                    .expect(&format!("could not parse `{i}` as u64"))
            })
            .collect::<Vec<_>>()
    });

    let mut current = row_iter.next().expect("could not get the first row");

    for row in row_iter {
        let mut next = Vec::with_capacity(row.len());
        for (idx, item) in row.iter().enumerate() {
            match idx {
                0 => next.push(item + current[0]),
                _ => next.push(u64::max(
                    item + current.get(idx).copied().unwrap_or(0),
                    item + current[idx - 1],
                )),
            }
        }

        current = next;
    }

    *current
        .iter()
        .max()
        .expect("current was empty when trying to determine max")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 1_074)
    }
}
