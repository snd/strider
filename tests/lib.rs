use std::collections::VecDeque;

extern crate sliding_window;
use sliding_window::SliceRing;

#[test]
fn test_deque_push_many_back() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    let vec = (0..10000).collect::<Vec<i32>>();
    deque.push_many_back(&vec[..]);
    assert_eq!(deque.len(), 10000);
    assert_eq!(deque[0], 0);
    assert_eq!(deque[9999], 9999);
    let vec = (500..1000).collect::<Vec<i32>>();
    deque.push_many_back(&vec[..]);
    assert_eq!(deque.len(), 10500);
    assert_eq!(deque[10000], 500);
    assert_eq!(deque[10499], 999);
}

// impl<T: Copy> SlidingWindow<T> {
//     pub fn new(window_size: usize, step_size: usize) -> Self {
//         assert!(0 < window_size);
//         assert!(0 < step_size);
//
//         SlidingWindow {
//             window_size: window_size,
//             step_size: step_size,
//             // TODO with_capacity
//             ringbuffer: VecDeque::new(),
//         }
//     }
//
//     // TODO how fast is this ?
//     // time complexity
//     // append `samples`
//     pub fn len
//
//     pub fn is_full(&self) -> bool {
//         self.window_size <= self.ringbuffer.len()
//     }
//
//     pub fn read_front(&mut self, fill_me: &mut [T]) -> bool {
//         if !self.can_fill() { return false; }
//         assert_eq!(fill_me.len(), self.window_size);
//         for i in 0..self.window_size {
//             fill_me[i] = self.ringbuffer[i];
//         }
//         true
//     }
//
//     pub fn drop_front(&mut self, count: usize) {
//         for _ in 0..count {
//             self.buf.pop_front();
//         }
//     }
//
//     // if `self.can_fill()` fills `window` fully with the next
//     // `window_size` samples.
//     // then makes a step discards `self.step_size` samples.
//     // else does nothing.
//     // `window.len()` must be equal to `self.window_size`.
//     // returns whether `self.can_fill()`.
//     pub fn fill_and_step(&mut self, fill_me: &mut [T]) -> bool {
//         if !self.fill(fill_me) { return false; }
//         self.step();
//         true
//     }
// }

// #[test]
// fn test_window_1_step_1() {
//     let mut sliding_window: SlidingWindow<i32> = SlidingWindow::new(1, 1);
//     assert!(!sliding_window.can_fill());
//     let samples: Vec<i32> = vec![];
//     sliding_window.write(&samples[..]);
//     assert!(!sliding_window.can_fill());
//     assert_eq!(sliding_window.ringbuffer.len(), 0);
//
//     let mut window: Vec<i32> = vec![0];
//     assert!(!sliding_window.read(&mut window[..]));
//     assert_eq!(window, vec![0]);
//
//     let samples: Vec<i32> = vec![1];
//     sliding_window.write(&samples[..]);
//     assert_eq!(sliding_window.ringbuffer.len(), 1);
//     assert!(sliding_window.can_fill());
//     assert!(sliding_window.read(&mut window[..]));
//     assert_eq!(window, vec![1]);
//     assert_eq!(sliding_window.ringbuffer.len(), 0);
//
//     let samples: Vec<i32> = vec![1, 2, 3];
//     sliding_window.write(&samples[..]);
//     assert_eq!(sliding_window.ringbuffer.len(), 3);
//
//     assert!(sliding_window.can_fill());
//     assert!(sliding_window.read(&mut window[..]));
//     assert_eq!(window, vec![1]);
//     assert_eq!(sliding_window.ringbuffer.len(), 2);
//
//     assert!(sliding_window.can_fill());
//     assert!(sliding_window.read(&mut window[..]));
//     assert_eq!(window, vec![2]);
//     assert_eq!(sliding_window.ringbuffer.len(), 1);
//
//     assert!(sliding_window.can_fill());
//     assert!(sliding_window.read(&mut window[..]));
//     assert_eq!(window, vec![3]);
//     assert_eq!(sliding_window.ringbuffer.len(), 0);
//     assert!(!sliding_window.can_fill());
// }
//
// #[test]
// fn test_window_4_step_2() {
//     // 50% overlap
//
//     let mut sliding_window: SlidingWindow<i32> = SlidingWindow::new(4, 2);
//     assert!(!sliding_window.can_fill());
//     let samples: Vec<i32> = vec![];
//     sliding_window.write(&samples[..]);
//     assert!(!sliding_window.can_fill());
//     assert_eq!(sliding_window.ringbuffer.len(), 0);
//
//     let mut window: Vec<i32> = vec![0];
//     assert!(!sliding_window.read(&mut window[..]));
//     assert_eq!(window, vec![0]);
//
//     {
//         let mut ring: VecDeque<i32> = VecDeque::new();
//         let mut window: Vec<i32> = vec![0, 0, 0, 0];
//         let samples: Vec<i32> = vec![1, 2, 3];
//         // println!("samples {:?}", samples);
//         let mut calls = 0;
//         window!(2, ring, &samples[..], &mut window[..], {
//             calls += 1;
//         });
//         assert_eq!(calls, 0);
//         assert_eq!(window, vec![0, 0, 0, 0]);
//         assert_eq!(ring.len(), 3);
//     }
//
//     {
//         let mut ring: VecDeque<i32> = VecDeque::new();
//         let mut window: Vec<i32> = vec![0, 0, 0, 0];
//         let samples: Vec<i32> = vec![1, 2, 3, 4];
//         // println!("samples {:?}", samples);
//         let mut calls = 0;
//         window!(2, ring, &samples[..], &mut window[..], {
//             assert_eq!(ring.len(), 2);
//             assert_eq!(window, vec![1, 2, 3, 4]);
//             calls += 1;
//         });
//         assert_eq!(calls, 1);
//         assert_eq!(window, vec![1, 2, 3, 4]);
//         assert_eq!(ring.len(), 2);
//     }
//
//     {
//         let mut ring: VecDeque<i32> = VecDeque::new();
//         let mut window: Vec<i32> = vec![0, 0, 0, 0];
//         let samples: Vec<i32> = vec![1, 2, 3 ,4, 5, 6, 7, 8, 9, 10, 11];
//         let mut calls = 0;
//         window!(2, ring, &samples[..], &mut window[..], {
//             match calls {
//                 0 => { assert_eq!(window, vec![1, 2, 3, 4]) },
//                 1 => { assert_eq!(window, vec![3, 4, 5, 6]) },
//                 2 => { assert_eq!(window, vec![5, 6, 7, 8]) },
//                 3 => { assert_eq!(window, vec![7, 8, 9, 10]) },
//                 _ => { assert!(false) },
//             }
//             calls += 1;
//         });
//         assert_eq!(calls, 4);
//         assert_eq!(window, vec![7, 8, 9, 10]);
//         assert_eq!(ring.len(), 3);
//
//         let samples: Vec<i32> = vec![12, 13, 14, 15, 16];
//         let mut calls = 0;
//         window!(2, ring, &samples[..], &mut window[..], {
//             match calls {
//                 0 => { assert_eq!(window, vec![9, 10, 11, 12]) },
//                 1 => { assert_eq!(window, vec![11, 12, 13, 14]) },
//                 2 => { assert_eq!(window, vec![13, 14, 15, 16]) },
//                 _ => { assert!(false) },
//             }
//             calls += 1;
//         });
//         assert_eq!(calls, 3);
//         assert_eq!(window, vec![13, 14, 15, 16]);
//         assert_eq!(ring.len(), 2);
//     }
// }
//
// #[test]
// fn test_window_1024_step_512() {
//     // 50% overlap
//     use std::collections::VecDeque;
//     use std::iter;
//
//     {
//         let mut ring: VecDeque<i32> = VecDeque::new();
//         let mut window: Vec<i32> = vec![0; 1024];
//         let samples: Vec<i32> =
//             iter::repeat(1).take(1024)
//             .chain(iter::repeat(2).take(1024))
//             .chain(iter::repeat(3).take(1024))
//             .chain(iter::repeat(4).take(1024))
//             .collect();
//         assert_eq!(samples.len(), 1024 * 4);
//         let mut calls = 0;
//         window!(512, ring, &samples[..], &mut window[..], {
//             match calls {
//                 0 => {
//                     assert_eq!(
//                         window,
//                         iter::repeat(1).take(1024).collect::<Vec<i32>>()
//                     );
//                 },
//                 1 => {
//                     assert_eq!(
//                         window,
//                         iter::repeat(1).take(512)
//                         .chain(iter::repeat(2).take(512))
//                         .collect::<Vec<i32>>()
//                     );
//                 },
//                 2 => {
//                     assert_eq!(
//                         window,
//                         iter::repeat(2).take(1024).collect::<Vec<i32>>()
//                     );
//                 },
//                 3 => {
//                     assert_eq!(
//                         window,
//                         iter::repeat(2).take(512)
//                         .chain(iter::repeat(3).take(512))
//                         .collect::<Vec<i32>>()
//                     );
//                 },
//                 4 => {
//                     assert_eq!(
//                         window,
//                         iter::repeat(3).take(1024).collect::<Vec<i32>>()
//                     );
//                 },
//                 5 => {
//                     assert_eq!(
//                         window,
//                         iter::repeat(3).take(512)
//                         .chain(iter::repeat(4).take(512))
//                         .collect::<Vec<i32>>()
//                     );
//                 },
//                 6 => {
//                     assert_eq!(
//                         window,
//                         iter::repeat(4).take(1024).collect::<Vec<i32>>()
//                     );
//                 },
//                 _ => {}
//             }
//             calls += 1;
//         });
//         assert_eq!(calls, 7);
//         assert_eq!(ring.len(), 512);
//     }
// }


// 4048 and 512
