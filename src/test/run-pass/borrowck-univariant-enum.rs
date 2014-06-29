// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(managed_boxes)]

use std::cell::Cell;
use std::gc::GC;

enum newtype {
    newtype(int)
}

pub fn main() {

    // Test that borrowck treats enums with a single variant
    // specially.

    let x = box(GC) Cell::new(5);
    let y = box(GC) Cell::new(newtype(3));
    let z = match y.get() {
      newtype(b) => {
        x.set(x.get() + 1);
        x.get() * b
      }
    };
    assert_eq!(z, 18);
}