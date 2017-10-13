#![crate_id = "bignum#0.1.1-pre"]

#![comment = "Bignum library for Rust"]
#![crate_type = "rlib"]

#![feature(macro_rules)]

extern crate libc;
extern crate gmp;
extern crate num;
extern crate rand;

pub mod bigint;
pub mod rational;
