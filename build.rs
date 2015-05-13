// build.rs

// Bring in a dependency on an externally maintained `cc` package which manages
// invoking the C compiler.
extern crate gcc;

fn main() {
    gcc::compile_library("libcurrent_desktop.a", &["src/cur_desk.c"]);
}
