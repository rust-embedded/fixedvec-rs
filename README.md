fixedvec-rs
===========

[![Build Status] (https://img.shields.io/travis/nastevens/fixedvec-rs.svg)](https://travis-ci.org/nastevens/fixedvec-rs)

- [API Documentation](http://nastevens.github.io/fixedvec-rs/)

`fixedvec-rs` is a Rust library/crate providing a heapless version of the Rust
vector type. Although more limited than the libstd version, fixedvec-rs
provides a much-needed "managed" array type for embedded systems or other
projects that cannot rely on the heap.

TODO
----

`fixedvec-rs` as it exists now is not ready for any sort of general use, and is
extremely limited at this point. This is a work in progress. Once a reasonable
level of functionality has been achieved, this warning will be removed and the
crate will be uploaded to crates.io.

License
-------

```
Copyright (c) 2015, Nick Stevens <nick@bitcurry.com>

The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
