use std::{collections::HashMap, sync::LazyLock};

static MAPPINGS: LazyLock<HashMap<u64, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        (0, ""),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
    ])
});

// Return the length of the number as a word, excluding spaces and dashes.
pub fn number_to_string_length(n: u64) -> usize {
    let mut length = 0usize;

    if (100..1000).contains(&n) {
        let hundreds = n / 100;
        length += MAPPINGS.get(&hundreds).unwrap().len();
        length += 7; // hundred

        if n % 100 != 0 {
            length += 3; // and
            length += number_to_string_length(n % 100);
        }
    } else if (1..100).contains(&n) {
        if let Some(val) = MAPPINGS.get(&n) {
            length += val.len()
        } else {
            let rem10 = n % 10;
            length += MAPPINGS.get(&(n - rem10)).unwrap().len();
            length += MAPPINGS.get(&rem10).unwrap().len();
        }
    }

    length
}

pub fn solution() -> u64 {
    let mut length_acc = "onethousand".len();
    length_acc += (1..1_000).map(number_to_string_length).sum::<usize>();

    length_acc as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_string_returns_expected_lengths() {
        assert_eq!(number_to_string_length(90), 6); // ninety
        assert_eq!(number_to_string_length(14), 8); // fourteen
        assert_eq!(number_to_string_length(24), 10); // twentyfour
        assert_eq!(number_to_string_length(100), 10); // onehundred
        assert_eq!(number_to_string_length(112), 19); // onehundredandtwelve
    }

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 21_124)
    }
}
