#[macro_use]
extern crate criterion;

fn bench_ring_sha2(c: &mut criterion::Criterion) {
    let mut ctx = ring::digest::Context::new(&ring::digest::SHA256);
    ctx.update(b"testing");

    c.bench_function("hashing with ring", |b| b.iter(|| {
        let mut ctx = ctx.clone();
        ctx.update(criterion::black_box(b"someinput"));
        let digest = ctx.finish();
        assert_eq!(digest.as_ref(), [126, 95, 252, 85, 113, 109, 112, 3, 11, 130, 133, 70, 74, 104, 99, 129, 176, 158, 55, 233, 104, 71, 174, 0, 214, 183, 146, 102, 114, 46, 115, 246]);
    }));
}

fn bench_crates_sha2(c: &mut criterion::Criterion) {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.input(b"testing");

    c.bench_function("hashing with sha2", |b| b.iter(|| {
        let mut hasher = hasher.clone();
        hasher.input(criterion::black_box(b"someinput"));
        let digest = hasher.result();
        assert_eq!(digest.as_ref(), [126, 95, 252, 85, 113, 109, 112, 3, 11, 130, 133, 70, 74, 104, 99, 129, 176, 158, 55, 233, 104, 71, 174, 0, 214, 183, 146, 102, 114, 46, 115, 246]);
    }));
}

criterion_group!(benches, bench_ring_sha2, bench_crates_sha2);
criterion_main!(benches);
