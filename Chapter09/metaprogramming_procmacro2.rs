#![feature(proc_macro_non_items)]
#![feature(use_extern_macros)]
extern crate procmacro2;

fn main() {
   procmacro2::misc_syntax!(
      where while abcd : u64 >> 1 + 2 * 3; where T: 'x + A<B='y+C+D>;[M];A::f
   );
}
