#![feature(proc_macro_non_items)]
#![feature(use_extern_macros)]
extern crate procmacro;

fn main() {
   let _ = procmacro::f!();
}
