const NAMES_STR: &str = include_str!("../../data/0022_names.txt");
const DELTA: u8 = b'A' - 1;

fn score_name(name: &str) -> usize {
    // NOTE: Assumes name is UPPERCASE.
    name.bytes().map(|i| (i - DELTA) as usize).sum()
}

pub fn solution() -> u64 {
    let mut names = NAMES_STR[1..NAMES_STR.len() - 1]
        .split("\",\"")
        .collect::<Vec<_>>();
    names.sort_unstable();

    names
        .iter()
        .enumerate()
        .map(|(pos, name)| (pos + 1) * score_name(name))
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 871_198_282)
    }
}
