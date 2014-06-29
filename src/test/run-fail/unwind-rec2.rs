// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:fail


fn build1() -> Vec<int> {
    vec!(0,0,0,0,0,0,0)
}

fn build2() -> Vec<int> {
    fail!();
}

struct Blk { node: Vec<int> , span: Vec<int> }

fn main() {
    let _blk = Blk {
        node: build1(),
        span: build2()
    };
}