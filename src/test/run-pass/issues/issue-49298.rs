// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![feature(test)]
#![allow(unused_mut)] // under NLL we get warning about `x` below: rust-lang/rust#54499

// This test is bogus (i.e., should be compile-fail) during the period
// where #54986 is implemented and #54987 is *not* implemented. For
// now: just ignore it under nll
//
// ignore-compare-mode-nll

// This test is checking that the space allocated for `x.1` does not
// overlap with `y`. (The reason why such a thing happened at one
// point was because `x.0: Void` and thus the whole type of `x` was
// uninhabited, and so the compiler thought it was safe to use the
// space of `x.1` to hold `y`.)
//
// That's a fine thing to test when this code is accepted by the
// compiler, and this code is being transcribed accordingly into
// the ui test issue-21232-partial-init-and-use.rs

extern crate test;

enum Void {}

fn main() {
    let mut x: (Void, usize);
    let mut y = 42;
    x.1 = 13;

    // Make sure `y` stays on the stack.
    test::black_box(&mut y);

    // Check that the write to `x.1` did not overwrite `y`.
    // Note that this doesn't fail with optimizations enabled,
    // because we can't keep `x.1` on the stack, like we can `y`,
    // as we can't borrow partially initialized variables.
    assert_eq!(y.to_string(), "42");

    // Check that `(Void, usize)` has space for the `usize` field.
    assert_eq!(std::mem::size_of::<(Void, usize)>(),
               std::mem::size_of::<usize>());
}
