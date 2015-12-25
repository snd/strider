#![feature(test)]
extern crate test;

extern crate sliding_window;
use sliding_window::SliceRing;
use sliding_window::OptimizedSliceRing;

#[bench]
fn bench_optimized_push_many_back(b: &mut test::Bencher) {
    let vec = (0..1000).collect::<Vec<i32>>();
    b.iter(|| {
        let mut deque: OptimizedSliceRing<i32> = OptimizedSliceRing::new();
        deque.push_many_back(&vec[..]);
        deque
    });
}

#[bench]
fn bench_optimized_drop_many_front(b: &mut test::Bencher) {
    let vec = (0..1000).collect::<Vec<i32>>();
    b.iter(|| {
        let mut deque: OptimizedSliceRing<i32> = OptimizedSliceRing::new();
        deque.push_many_back(&vec[..]);
        deque.drop_many_front(1000)
    });
}

#[bench]
fn bench_optimized_read_many_front(b: &mut test::Bencher) {
    let mut output: Vec<i32> = std::iter::repeat(0).take(1000).collect();
    let vec = (0..1000).collect::<Vec<i32>>();
    b.iter(|| {
        let mut deque: OptimizedSliceRing<i32> = OptimizedSliceRing::new();
        deque.push_many_back(&vec[..]);
        deque.read_many_front(&mut output[..])
    });
}
