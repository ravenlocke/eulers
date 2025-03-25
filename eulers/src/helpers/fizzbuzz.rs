const FIZZBUZZ_DELTAS: [u64; 7] = [3, 2, 1, 3, 1, 2, 3];

#[derive(Default)]
pub struct FizzBuzzIter {
    current: u64,
    idx: usize,
}

impl Iterator for FizzBuzzIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += FIZZBUZZ_DELTAS[self.idx];

        match self.idx {
            6 => self.idx = 0,
            _ => self.idx += 1,
        }

        Some(self.current)
    }
}
