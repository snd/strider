#![feature(test)]
extern crate test;

#[bench]
fn bench_empty_to_probe_for_release_mode(b: &mut test::Bencher) {
    b.iter(|| 1)
}
