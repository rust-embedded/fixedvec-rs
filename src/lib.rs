// The MIT License (MIT)
//
// Copyright (c) 2015  Nick Stevens <nick@bitcurry.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![crate_type = "lib"]
#![crate_name = "fixedvec"]

#![feature(core)]

///! Heapless Vec implementation using only libcore
///!
///! When developing for certain types of systems, especially embedded systems,
///! it is desireable to avoid the non-determinism that can be introduced by
///! using a heap. A commonly used data structure is a "buffer" - a
///! pre-allocated chunk of memory, either in static memory or on the stack.
///!
///! Thanks to the extensibility of Rust, it is possible to have a datatype
///! that performs _almost_ like the libstd `Vec` type, without requiring a
///! heap and while only using libcore.
///!
///! # Examples
///!
///! Typical usage looks like the following:
///!
///! ```rust
///! #![feature(core)]
///! extern crate core;
///!
///! #[macro_use] extern crate fixedvec;
///!
///! use fixedvec::FixedVec;
///!
///! #[derive(Debug, Default, Copy, Clone)]
///! struct MyStruct {
///!     a: i32,
///!     b: i32
///! }
///!
///! fn main() {
///!     let mut preallocated_space = alloc_stack!([MyStruct; 16]);
///!     let vec = FixedVec::new(&mut preallocated_space);
///! }
///! ```

extern crate core;

#[macro_export]
macro_rules! alloc_stack {
    ([$item_type:ty; $len:expr]) => ({
        let space: [$item_type; $len] = [ Default::default() ; $len ];
        space
    })
}

#[derive(Debug)]
pub struct FixedVec<'a, T: 'a> {
    memory: &'a mut [T],
    used: usize,
}

impl <'a, T: 'a> FixedVec<'a, T> {
    /// Create a new `FixedVec` from the provided memory
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate fixedvec;
    /// use fixedvec::FixedVec;
    ///
    /// fn main() {
    ///     let mut space = alloc_stack!([u8; 16]);
    ///     let vec = FixedVec::new(&mut space);
    ///     assert!(vec.capacity() == 16);
    ///     assert!(vec.len() == 0);
    /// }
    /// ```
    ///
    pub fn new(memory: &'a mut [T]) -> Self {
        FixedVec {
            memory: memory,
            used: 0,
        }
    }

    /// Returns the capacity of the `FixedVec`
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate fixedvec;
    /// use fixedvec::FixedVec;
    ///
    /// fn main() {
    ///     let mut space = alloc_stack!([u8; 16]);
    ///     let vec = FixedVec::new(&mut space);
    ///     assert_eq!(vec.capacity(), 16);
    /// }
    /// ```
    pub fn capacity(&self) -> usize {
        self.memory.len()
    }

    /// Returns the number of elements in the `FixedVec`. This will always be
    /// less than or equal to the `capacity()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate fixedvec;
    /// use fixedvec::FixedVec;
    ///
    /// fn main() {
    ///     let mut space = alloc_stack!([u8; 16]);
    ///     let mut vec = FixedVec::new(&mut space);
    ///     vec.push(1);
    ///     vec.push(2);
    ///     assert_eq!(vec.len(), 2);
    /// }
    /// ```
    pub fn len(&self) -> usize {
        self.used
    }

    /// Appends an element to the back of the `FixedVec`.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the collection exceeds its
    /// capacity.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate fixedvec;
    /// use fixedvec::FixedVec;
    ///
    /// fn main() {
    ///     let mut space = alloc_stack!([u8; 16]);
    ///     let mut vec = FixedVec::new(&mut space);
    ///     vec.push(1);
    ///     vec.push(2);
    ///     assert_eq!(vec.len(), 2);
    /// }
    /// ```
    pub fn push(&mut self, value: T) {
        self.memory[self.used] = value;
        self.used += 1;
    }
}
