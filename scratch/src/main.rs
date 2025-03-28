const RESULT: usize = crabtime::eval!(
    (0..1_000)
        .filter(|i| i % 5 == 0 || i % 3 == 0)
        .sum::<usize>()
);

fn main() {
    println!("{RESULT}")
}
