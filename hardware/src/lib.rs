#[macro_use]
mod macros;

mod ch_1;
mod ch_2;
mod ch_3;
mod ch_5;

pub use ch_5::Computer;

#[cfg(test)]
#[macro_use]
extern crate test_case;
