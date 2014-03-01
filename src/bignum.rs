extern crate gmp;

use gmp::Mpz;
use std::fmt;

struct BigUint {
    data: Mpz
}

pub trait ToBigUint {
    fn to_biguint(&self) -> Option<BigUint>;
}

impl ToBigUint for uint {
    fn to_biguint(&self) -> Option<BigUint> {
        let mpz: Option<Mpz> = FromPrimitive::from_uint(*self);
        match mpz {
            Some(mpz) => Some(BigUint{ data: mpz }),
            None      => None
        }
    }
}

impl ToStrRadix for BigUint {
    fn to_str_radix(&self, radix: uint) -> ~str {
        self.data.to_str_radix(radix)
    }
}

impl fmt::Show for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}", self.to_str_radix(10))
    }
}

impl Add<BigUint, BigUint> for BigUint {
    fn add(&self, other: &BigUint) -> BigUint {
        BigUint{data: self.data.add(&other.data)}
    }
}

impl Sub<BigUint, BigUint> for BigUint {
    fn sub(&self, other: &BigUint) -> BigUint {
        BigUint{data: self.data.sub(&other.data)}
    }
}

impl Mul<BigUint, BigUint> for BigUint {
    fn mul(&self, other: &BigUint) -> BigUint {
        BigUint{data: self.data.mul(&other.data)}
    }
}

#[test]
fn test_to_and_fro() {
    let two = 2u.to_biguint().unwrap();
    assert_eq!(two.to_str(), ~"2");
}

#[test]
fn test_printing() {
    println!("{} == 2!", 2u.to_biguint().unwrap()); // Doesn't fail
}

#[test]
fn test_add() {
    let two = 2u.to_biguint().unwrap();
    let three = 3u.to_biguint().unwrap();

    assert_eq!(two.add(&three).to_str(), ~"5");
    assert_eq!((two + three).to_str(), ~"5");
}

#[test]
fn test_simple_sub() {
    let two = 2u.to_biguint().unwrap();
    let three = 3u.to_biguint().unwrap();

    assert_eq!(three.sub(&two).to_str(), ~"1");
    assert_eq!((three - two).to_str(), ~"1");
}

#[test]
fn test_mul() {
    let two = 2u.to_biguint().unwrap();
    let three = 3u.to_biguint().unwrap();

    assert_eq!(two.mul(&three).to_str(), ~"6");
    assert_eq!((two * three).to_str(), ~"6");
}

#[cfg(not(test))]
fn main() {
    let x = 42.to_biguint().unwrap();
    let two = 2.to_biguint().unwrap();

    println!("{}", x + two); //-> 44
    println!("{}", x - two); //-> 40
    println!("{}", x * two); //-> 84
}
