// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(fn_must_use)]
#![warn(unused_must_use)]

struct MyStruct {
    n: usize,
}

impl MyStruct {
    #[must_use]
    fn need_to_use_this_method_value(&self) -> usize {
        self.n
    }
}

trait EvenNature {
    #[must_use = "no side effects"]
    fn is_even(&self) -> bool;
}

impl EvenNature for MyStruct {
    fn is_even(&self) -> bool {
        self.n % 2 == 0
    }
}

trait Replaceable {
    fn replace(&mut self, substitute: usize) -> usize;
}

impl Replaceable for MyStruct {
    // ↓ N.b.: `#[must_use]` attribute on a particular trait implementation
    // method won't work; the attribute should be on the method signature in
    // the trait's definition.
    #[must_use]
    fn replace(&mut self, substitute: usize) -> usize {
        let previously = self.n;
        self.n = substitute;
        previously
    }
}

#[must_use = "it's important"]
fn need_to_use_this_value() -> bool {
    false
}

fn main() {
    need_to_use_this_value();

    let mut m = MyStruct { n: 2 };
    m.need_to_use_this_method_value();
    m.is_even(); // trait method!

    m.replace(3); // won't warn (annotation needs to be in trait definition)

    2.eq(&3); // comparison methods are `must_use`

    2 == 3; // lint includes comparison operators
}
