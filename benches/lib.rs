#![feature(test)]
extern crate test;

use std::collections::VecDeque;

#[macro_use]
extern crate strider;
use strider::SliceRing;
use strider::SliceRingImpl;

#[bench]
fn bench_empty_to_probe_for_release_mode(bencher: &mut test::Bencher) {
    bencher.iter(|| 1)
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
fn bench_push_many_back_deque(bencher: &mut test::Bencher) {
    bench_push_many_back!(bencher, VecDeque::new());
}
#[bench]
fn bench_push_many_back_deque_with_capacity(bencher: &mut test::Bencher) {
    bench_push_many_back!(bencher, VecDeque::with_capacity(1000));
}
#[bench]
fn bench_push_many_back_optimized(bencher: &mut test::Bencher) {
    bench_push_many_back!(bencher, SliceRingImpl::new());
}
#[bench]
fn bench_push_many_back_optimized_with_capacity(bencher: &mut test::Bencher) {
    bench_push_many_back!(bencher, SliceRingImpl::with_capacity(1000));
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
fn bench_drop_many_front_deque(bencher: &mut test::Bencher) {
    bench_drop_many_front!(bencher, VecDeque::new());
}
#[bench]
fn bench_drop_many_front_deque_with_capacity(bencher: &mut test::Bencher) {
    bench_drop_many_front!(bencher, VecDeque::with_capacity(1000));
}
#[bench]
fn bench_drop_many_front_optimized(bencher: &mut test::Bencher) {
    bench_drop_many_front!(bencher, SliceRingImpl::new());
}
#[bench]
fn bench_drop_many_front_optimized_with_capacity(bencher: &mut test::Bencher) {
    bench_drop_many_front!(bencher, SliceRingImpl::with_capacity(1000));
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
fn bench_read_many_front_deque(bencher: &mut test::Bencher) {
    bench_read_many_front!(bencher, VecDeque::new());
}
#[bench]
fn bench_read_many_front_deque_with_capacity(bencher: &mut test::Bencher) {
    bench_read_many_front!(bencher, VecDeque::with_capacity(1000));
}
#[bench]
fn bench_read_many_front_optimized(bencher: &mut test::Bencher) {
    bench_read_many_front!(bencher, SliceRingImpl::new());
}
#[bench]
fn bench_read_many_front_optimized_with_capacity(bencher: &mut test::Bencher) {
    bench_read_many_front!(bencher, SliceRingImpl::with_capacity(1000));
}

#[bench]
fn bench_test_slice_ring_deque(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        test_slice_ring!(VecDeque::<i32>::new());
    });
}
#[bench]
fn bench_test_slice_ring_optimized(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        test_slice_ring!(SliceRingImpl::<i32>::new())
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
fn bench_slice_ring_windowing_deque(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        bench_slice_ring_windowing!(VecDeque::<i32>::new());
    });
}
#[bench]
fn bench_slice_ring_windowing_optimized(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        bench_slice_ring_windowing!(SliceRingImpl::<i32>::new())
    });
}

#[macro_export]
macro_rules! bench_slice_ring_one_minute_audio {
    ($bencher:expr, $new:expr) => {{
        // we move this outside the bencher
        // to measure only the time taken by the
        // slice ring operations.
        // keep all allocations out of the benchmark iteration.
        let samples_per_second = 44100;
        let seconds = 60;
        let sample_count: usize = seconds * samples_per_second;
        // println!("sample_count = {}", sample_count);
        let samples = (0..(sample_count as i32)).collect::<Vec<i32>>();
        let input_slice_size = 10000;
        let window_size = 1024;
        let step_size = 512;

        // determine the output_size by
        // simulating just the indexes without data
        let mut output_size = 0;
        let mut input_index = 0;
        let mut output_index = 0;
        let mut ring_size = 0;
        while input_index < samples.len() {
            let input_range_start = input_index;
            let input_range_end = std::cmp::min(sample_count, input_index + input_slice_size);
            // push_many_back
            ring_size += input_range_end - input_range_start;
            while ring_size >= window_size {
                let output_range_start = output_index;
                let output_range_end = output_index + window_size;
                output_size += output_range_end - output_range_start;
                ring_size -= step_size;
                output_index += window_size;
            }
            input_index += input_slice_size;
        }
        // println!("output_size = {}", output_size);
        let mut output = std::iter::repeat(0).take(output_size).collect::<Vec<i32>>();
        // println!("output.len() = {}", output.len());
        let mut expected_output = Vec::<i32>::new();
        let mut expected_output_index = 0;
        while expected_output_index < sample_count {
            let range_start = expected_output_index as i32;
            let range_end = (expected_output_index + window_size) as i32;
            let mut range_vec = (range_start..range_end).collect::<Vec<i32>>();
            expected_output.append(&mut range_vec);
            expected_output_index += step_size;
        }
        // println!("expected_output.len() = {}", expected_output.len());
        $bencher.iter(|| {
            let mut testable = $new;
            let mut input_index = 0;
            let mut output_index = 0;
            while input_index < samples.len() {
                let input_range_start = input_index;
                let input_range_end = std::cmp::min(sample_count, input_index + input_slice_size);
                let input_slice =
                    &samples[input_range_start..input_range_end];
                testable.push_many_back(input_slice);
                while testable.len() >= window_size {
                    let output_range_start = output_index;
                    let output_range_end = output_index + window_size;
                    let output_slice =
                        &mut output[output_range_start..output_range_end];
                    testable.read_many_front(output_slice);
                    testable.drop_many_front(step_size);
                    output_index += window_size;
                }
                input_index += input_slice_size;
            }
            // println!("input_index = {}", input_index);
            // println!("output_index = {}", output_index);
            testable
        });
        for i in 0..output.len() {
            assert!(output[i] == expected_output[i], "{} != {} at {}", output[i], expected_output[i], i);
        }
        // assert_eq!(output.len(), expected_output.len());
    }};
}
#[bench]
fn bench_slice_ring_one_minute_audio_deque(bencher: &mut test::Bencher) {
    bench_slice_ring_one_minute_audio!(bencher, VecDeque::<i32>::new());
}
#[bench]
fn bench_slice_ring_one_minute_audio_optimized(bencher: &mut test::Bencher) {
    bench_slice_ring_one_minute_audio!(bencher, SliceRingImpl::<i32>::new());
}
