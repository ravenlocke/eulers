use eulers::solutions::*;

use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! add_benches {
    ([$($name:ident),*]) => {
        fn criterion_benchmark(c: &mut Criterion) {

        $(
            paste::paste! {
                c.bench_function(stringify!($name), |b| {
                    b.iter(|| $name::solution())
                });
            }
        )*
    }
    };
}
add_benches!([
    p0001, p0002, p0003, p0004, p0005, p0006, p0007, p0008, p0009, p0010, p0011, p0012, p0013,
    p0014, p0015, p0016, p0017, p0018, p0019, p0020, p0021, p0022, p0023, p0024, p0025, p0026,
    p0027, p0028, p0029, p0030, p0031, p0032
]);

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
