#[macro_use]
extern crate criterion;

use xkcp_sys::cshake::CShake;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let state = CShake::new("testing".as_bytes());
    c.bench_function("hashing", |b| b.iter(|| {
        let mut state = state.clone();
        state.update(criterion::black_box("someinput".as_bytes()));
        let digest = state.finalize();
        assert_eq!(digest, [169, 78, 48, 230, 118, 51, 183, 191, 229, 68, 138, 32, 153, 195, 93, 64, 169, 233, 231, 33, 211, 139, 46, 69, 29, 202, 109, 184, 29, 148, 143, 93]);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
