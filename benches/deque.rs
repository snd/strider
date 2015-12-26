#![feature(test)]
extern crate test;

use std::collections::VecDeque;

#[macro_use]
extern crate sliding_window;
use sliding_window::SliceRing;

#[bench]
fn bench_deque_push_many_back(b: &mut test::Bencher) {
    let vec = (0..1000).collect::<Vec<i32>>();
    b.iter(|| {
        let mut deque: VecDeque<i32> = VecDeque::new();
        deque.push_many_back(&vec[..]);
        deque
    });
}

#[bench]
fn bench_deque_push_many_back_drop_many_front(b: &mut test::Bencher) {
    let vec = (0..1000).collect::<Vec<i32>>();
    b.iter(|| {
        let mut deque: VecDeque<i32> = VecDeque::new();
        deque.push_many_back(&vec[..]);
        deque.drop_many_front(1000)
    });
}

#[bench]
fn bench_deque_read_many_front(b: &mut test::Bencher) {
    let mut output: Vec<i32> = std::iter::repeat(0).take(1000).collect();
    let vec = (0..1000).collect::<Vec<i32>>();
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_many_back(&vec[..]);
    b.iter(|| {
        deque.read_many_front(&mut output[..])
    });
}

#[bench]
fn bench_deque_test_slice_ring(b: &mut test::Bencher) {
    b.iter(|| {
        test_slice_ring!(VecDeque::<i32>::new());
    });
}
