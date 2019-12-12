#[macro_use]
extern crate criterion;

use xkcp_sys::kangaroo_twelve::KangarooTwelve;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let state = KangarooTwelve::new(b"testing", 32);
    c.bench_function("hashing", |b| b.iter(|| {
        let mut state = state.clone();
        state.update(criterion::black_box(b"someinput"));
        let digest = state.finalize();
        assert_eq!(digest, [187, 19, 67, 214, 73, 178, 187, 16, 174, 135, 82, 238, 25, 49, 129, 242, 63, 255, 171, 107, 253, 27, 108, 124, 37, 144, 106, 194, 91, 12, 229, 67]);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
