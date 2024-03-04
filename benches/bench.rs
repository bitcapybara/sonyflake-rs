use bencher::{benchmark_group, benchmark_main, Bencher};
use chrono::Utc;
use sonyflake::{decompose, Sonyflake};

fn bench_new(b: &mut Bencher) {
    b.iter(|| Sonyflake::new(1, Utc::now()));
}

fn bench_next_id(b: &mut Bencher) {
    let sf = Sonyflake::new(1, Utc::now());
    b.iter(|| sf.next_id());
}

fn bench_decompose(b: &mut Bencher) {
    let sf = Sonyflake::new(1, Utc::now());
    let next_id = sf.next_id();

    b.iter(|| decompose(next_id));
}

benchmark_group!(sonyflake_perf, bench_new, bench_next_id, bench_decompose);

benchmark_main!(sonyflake_perf);
