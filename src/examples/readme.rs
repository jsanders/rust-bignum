extern crate bignum;

use bignum::ToBigUint;

fn main() {
    let x = 42.to_biguint().unwrap();
    let two = 2.to_biguint().unwrap();

    println!("{}", x + two); //-> 44
    println!("{}", x - two); //-> 40
    println!("{}", x * two); //-> 84
}
