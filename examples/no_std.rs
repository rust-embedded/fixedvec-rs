// The MIT License (MIT)
//
// Copyright (c) 2015-2016 Nick Stevens <nick@bitcurry.com>
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

#![feature(lang_items, no_std, libc)]
#![no_std]
#![no_main]

/// Absolute minimum program that uses `fixedvec`. Primarily here to make sure
/// that the library is free of the libstd shackles.

// Pull in the system libc library for what crt0.o likely requires
extern crate libc;

// Pull in fixedvec
#[macro_use] extern crate fixedvec;

use fixedvec::FixedVec;

// Entry point for this program
#[no_mangle]
pub extern fn main(_: isize, _: *const *const u8) -> isize {
    let mut space = alloc_stack!([u8; 10]);
    let mut vec = FixedVec::new(&mut space);
    vec.extend(0..6);

    // We're quite limited, but to make sure we actually did something, check
    // the vector length and return that as the error code.
    if vec.len() == 6 {
        0
    } else {
        1
    }
}

// These functions and traits are usually provided by libstd: we have to
// provide them ourselves in no_std.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
