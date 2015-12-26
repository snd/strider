#![feature(test)]
extern crate test;

use std::collections::VecDeque;

#[macro_use]
extern crate sliding_window;
use sliding_window::SliceRing;
use sliding_window::OptimizedSliceRing;
use sliding_window::SlidingWindow;

#[bench]
fn bench_empty_to_probe_for_release_mode(b: &mut test::Bencher) {
    b.iter(|| 1)
}

macro_rules! bench_push_many_back {
    ($bencher:expr, $new:expr) => {{
        let vec = (0..1000).collect::<Vec<i32>>();
        $bencher.iter(|| {
            let mut testable = $new;
            testable.push_many_back(&vec[..]);
            testable
        });
    }}
}
#[bench]
fn bench_push_many_back_unoptimized(b: &mut test::Bencher) {
    bench_push_many_back!(b, VecDeque::new());
}
#[bench]
fn bench_push_many_back_optimized(b: &mut test::Bencher) {
    bench_push_many_back!(b, OptimizedSliceRing::new());
}

macro_rules! bench_push_many_back_drop_many_front {
    ($bencher:expr, $new:expr) => {{
        let vec = (0..1000).collect::<Vec<i32>>();
        $bencher.iter(|| {
            let mut testable = $new;
            testable.push_many_back(&vec[..]);
            testable.drop_many_front(1000)
        });
    }}
}

#[bench]
fn bench_push_many_back_drop_many_front_unoptimized(b: &mut test::Bencher) {
    bench_push_many_back!(b, VecDeque::new());
}
#[bench]
fn bench_push_many_back_drop_many_front_optimized(b: &mut test::Bencher) {
    bench_push_many_back!(b, OptimizedSliceRing::new());
}

macro_rules! bench_read_many_front {
    ($bencher:expr, $new:expr) => {{
        let mut output: Vec<i32> = std::iter::repeat(0).take(1000).collect();
        let vec = (0..1000).collect::<Vec<i32>>();
        let mut testable = $new;
        testable.push_many_back(&vec[..]);
        $bencher.iter(|| {
            testable.read_many_front(&mut output[..])
        });
    }}
}
#[bench]
fn bench_read_many_front_unoptimized(b: &mut test::Bencher) {
    bench_read_many_front!(b, VecDeque::new());
}
#[bench]
fn bench_read_many_front_optimized(b: &mut test::Bencher) {
    bench_read_many_front!(b, OptimizedSliceRing::new());
}

#[bench]
fn bench_test_slice_ring_unoptimized(b: &mut test::Bencher) {
    b.iter(|| {
        test_slice_ring!(VecDeque::<i32>::new());
    });
}
#[bench]
fn bench_test_slice_ring_optimized(b: &mut test::Bencher) {
    b.iter(|| {
        test_slice_ring!(OptimizedSliceRing::<i32>::new())
    });
}

#[bench]
fn bench_test_sliding_window_unoptimized(b: &mut test::Bencher) {
    b.iter(|| {
    // TODO this type declaration is soo ugly
        test_sliding_window!(SlidingWindow::<i32, VecDeque<i32>>::new_unoptimized(
            4096, 512));
    });
}
#[bench]
fn bench_test_sliding_window_optimized(b: &mut test::Bencher) {
    b.iter(|| {
        // TODO this type declaration is soo ugly
        test_sliding_window!(SlidingWindow::<i32, OptimizedSliceRing<i32>>::new(
            4096, 512));
    });
}
