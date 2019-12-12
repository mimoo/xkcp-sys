#[macro_use]
extern crate criterion;

use ring::digest;

fn criterion_benchmark(c: &mut criterion::Criterion) {


    let mut ctx = digest::Context::new(&digest::SHA256);
    ctx.update(b"testing");

    c.bench_function("hashing", |b| b.iter(|| {
        let mut ctx = ctx.clone();
        ctx.update(criterion::black_box(b"someinput"));
        let digest = ctx.finish();
        assert_eq!(digest.as_ref(), [126, 95, 252, 85, 113, 109, 112, 3, 11, 130, 133, 70, 74, 104, 99, 129, 176, 158, 55, 233, 104, 71, 174, 0, 214, 183, 146, 102, 114, 46, 115, 246]);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
