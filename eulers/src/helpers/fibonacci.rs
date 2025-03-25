use std::ops::Add;

pub struct Fibonacci<T> {
    a: T,
    b: T,
}

impl<T: Add<T, Output = T> + Copy> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(self.a)
    }
}

impl Default for Fibonacci<u64> {
    fn default() -> Self {
        Fibonacci { a: 1, b: 1 }
    }
}

impl<T> Fibonacci<T> {
    pub fn new(a: T, b: T) -> Self {
        Fibonacci { a, b }
    }
}
