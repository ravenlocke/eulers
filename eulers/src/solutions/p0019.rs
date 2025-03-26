use chrono::{Datelike, Days, TimeZone, Utc};

pub fn solution() -> u64 {
    // Sunday 6th Jan 1901, first Sunday in the time period.
    let mut start = Utc.with_ymd_and_hms(1901, 1, 6, 0, 0, 0).unwrap();
    // Exclusive end to allow < comparison.
    let end = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();

    let mut counter = 0u64;
    const DELTA: Days = Days::new(7);

    while start < end {
        if start.day() == 1 {
            counter += 1
        }
        start = start.checked_add_days(DELTA).unwrap();
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_expected_result() {
        assert_eq!(solution(), 171)
    }
}
