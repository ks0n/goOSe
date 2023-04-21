#![no_std]

pub mod mm;

#[derive(Debug)]
pub enum Error {
}

// /// A range similar to core::ops::Range but that is copyable.
// /// The range is half-open, inclusive below, exclusive above, ie. [start; end[
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Range<T: Copy> {
//     pub start: T,
//     pub end: T,
// }
//
// impl<T: Copy + core::cmp::PartialOrd + core::cmp::PartialEq + core::ops::Sub<Output = T>>
//     Range<T>
// {
//     pub fn new(start: T, end: T) -> Self {
//         Self { start, end }
//     }
//
//     pub fn contains(&self, val: T) -> bool {
//         self.start <= val && val < self.end
//     }
//
//     pub fn size(&self) -> T {
//         self.end - self.start
//     }
// }
