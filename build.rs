// build.rs

// Bring in a dependency on an externally maintained `cc` package which manages
// invoking the C compiler.
extern crate gcc;

use std::default::Default;

fn main() {
    gcc::compile_library("libcurrent_desktop.a", &Default::default(), &["src/cur_desk.c"]);
}
