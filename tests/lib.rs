use std::collections::VecDeque;

use std::io::{Cursor, Read, Write};

#[macro_use]
extern crate strider;
use strider::SliceRing;
use strider::SliceRingImpl;

macro_rules! test_string_windowing {
    ($new:expr) => {{
        const WINDOW_SIZE: usize = 8;
        const STEP_SIZE: usize = 2;
        let mut input = Cursor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut output = Cursor::new(Vec::<u8>::new());
        let mut ring = $new;
        let mut input_buffer: &mut [u8] = &mut [0; 4];
        let mut window_buffer: &mut [u8] = &mut [0; WINDOW_SIZE];

        loop {
            let input_count = input.read(input_buffer).unwrap();
            if input_count == 0 { break; }

            ring.push_many_back(&input_buffer[..input_count]);
            // read as long as enough samples are present (remain) in ring
            while WINDOW_SIZE <= ring.len() {
                ring.read_many_front(window_buffer);
                output.write(window_buffer).unwrap();
                // step
                ring.drop_many_front(STEP_SIZE);
            }
        }
        let actual = String::from_utf8(output.into_inner()).unwrap();
        let mut expected = String::new();
        expected.push_str("ABCDEFGH");
        expected.push_str("CDEFGHIJ");
        expected.push_str("EFGHIJKL");
        expected.push_str("GHIJKLMN");
        expected.push_str("IJKLMNOP");
        expected.push_str("KLMNOPQR");
        expected.push_str("MNOPQRST");
        expected.push_str("OPQRSTUV");
        expected.push_str("QRSTUVWX");
        expected.push_str("STUVWXYZ");
        assert_eq!(actual, expected);
    }}
}
#[test]
fn test_test_string_windowing_deque() {
    test_string_windowing!(VecDeque::<u8>::new());
}
#[test]
fn test_string_windowing_optimized() {
    test_string_windowing!(SliceRingImpl::<u8>::new());
}

#[test]
fn test_slice_ring_deque() {
    test_slice_ring!(VecDeque::<i32>::new());
}
#[test]
fn test_slice_ring_optimized() {
    test_slice_ring!(SliceRingImpl::<i32>::new());
}
