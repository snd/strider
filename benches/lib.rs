#![feature(test)]
extern crate test;

use std::collections::VecDeque;

#[macro_use]
extern crate sliding_window;
use sliding_window::SliceRing;
use sliding_window::OptimizedSliceRing;

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
fn bench_push_many_back_deque(b: &mut test::Bencher) {
    bench_push_many_back!(b, VecDeque::new());
}
#[bench]
fn bench_push_many_back_deque_with_capacity(b: &mut test::Bencher) {
    bench_push_many_back!(b, VecDeque::with_capacity(1000));
}
#[bench]
fn bench_push_many_back_optimized(b: &mut test::Bencher) {
    bench_push_many_back!(b, OptimizedSliceRing::new());
}
#[bench]
fn bench_push_many_back_optimized_with_capacity(b: &mut test::Bencher) {
    bench_push_many_back!(b, OptimizedSliceRing::with_capacity(1000));
}

macro_rules! bench_drop_many_front {
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
fn bench_drop_many_front_deque(b: &mut test::Bencher) {
    bench_drop_many_front!(b, VecDeque::new());
}
#[bench]
fn bench_drop_many_front_deque_with_capacity(b: &mut test::Bencher) {
    bench_drop_many_front!(b, VecDeque::with_capacity(1000));
}
#[bench]
fn bench_drop_many_front_optimized(b: &mut test::Bencher) {
    bench_drop_many_front!(b, OptimizedSliceRing::new());
}
#[bench]
fn bench_drop_many_front_optimized_with_capacity(b: &mut test::Bencher) {
    bench_drop_many_front!(b, OptimizedSliceRing::with_capacity(1000));
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
fn bench_read_many_front_deque(b: &mut test::Bencher) {
    bench_read_many_front!(b, VecDeque::new());
}
#[bench]
fn bench_read_many_front_deque_with_capacity(b: &mut test::Bencher) {
    bench_read_many_front!(b, VecDeque::with_capacity(1000));
}
#[bench]
fn bench_read_many_front_optimized(b: &mut test::Bencher) {
    bench_read_many_front!(b, OptimizedSliceRing::new());
}
#[bench]
fn bench_read_many_front_optimized_with_capacity(b: &mut test::Bencher) {
    bench_read_many_front!(b, OptimizedSliceRing::with_capacity(1000));
}

#[bench]
fn bench_test_slice_ring_deque(b: &mut test::Bencher) {
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
#[macro_export]
macro_rules! bench_slice_ring_windowing {
    ($new:expr) => {{
        let window_size = 4096;
        let step_size = 512;
        let mut testable = $new;
        let input: Vec<i32> = std::iter::repeat(0).take(6000).collect();
        let mut output: Vec<i32> = std::iter::repeat(0).take(window_size).collect();
        for _ in 0..100 {
            testable.push_many_back(&input[..]);
            while testable.len() >= window_size {
                testable.read_many_front(&mut output[..]);
                testable.drop_many_front(step_size);
            }
        }
        testable
    }};
}
#[bench]
fn bench_slice_ring_windowing_deque(b: &mut test::Bencher) {
    b.iter(|| {
        bench_slice_ring_windowing!(VecDeque::<i32>::new());
    });
}
#[bench]
fn bench_slice_ring_windowing_optimized(b: &mut test::Bencher) {
    b.iter(|| {
        bench_slice_ring_windowing!(OptimizedSliceRing::<i32>::new())
    });
}
