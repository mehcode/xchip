
#[macro_use]
extern crate axal;

extern crate rand;

mod core;
mod cpu;

// Generate C API
ax_expose!(core::Core);
