#[crate_id = "bignum#0.1.0-pre"];

#[comment = "Bignum library for Rust"];
#[crate_type = "rlib"];

extern crate gmp;

use gmp::{Mpz, RandState};
use std::fmt;
use std::num::ToStrRadix;
use std::rand::Rng;
use std::libc::c_ulong;

pub struct BigUint {
    data: Mpz
}

impl FromPrimitive for BigUint {
    fn from_u64(other: u64) -> Option<BigUint> {
        let mpz: Option<Mpz> = FromPrimitive::from_u64(other);
        match mpz {
            Some(mpz) => Some(BigUint{ data: mpz }),
            None      => None
        }
    }

    fn from_i64(other: i64) -> Option<BigUint> {
        if other < 0 {
            None
        } else {
            let mpz: Option<Mpz> = FromPrimitive::from_i64(other);
            match mpz {
                Some(mpz) => Some(BigUint{ data: mpz }),
                None      => None
            }
        }
    }
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

pub trait RandBigInt {
    /// Generate a random `BigUint` of the given bit size.
    fn gen_biguint(&mut self, bit_size: uint) -> BigUint;

    // /// Generate a random BigInt of the given bit size.
    // fn gen_bigint(&mut self, bit_size: uint) -> BigInt;

    /// Generate a random `BigUint` less than the given bound. Fails
    /// when the bound is zero.
    fn gen_biguint_below(&mut self, bound: &BigUint) -> BigUint;

    /// Generate a random `BigUint` within the given range. The lower
    /// bound is inclusive; the upper bound is exclusive. Fails when
    /// the upper bound is not greater than the lower bound.
    fn gen_biguint_range(&mut self, lbound: &BigUint, ubound: &BigUint) -> BigUint;

    // /// Generate a random `BigInt` within the given range. The lower
    // /// bound is inclusive; the upper bound is exclusive. Fails when
    // /// the upper bound is not greater than the lower bound.
    // fn gen_bigint_range(&mut self, lbound: &BigInt, ubound: &BigInt) -> BigInt;
}

impl<R: Rng> RandBigInt for R {
    fn gen_biguint(&mut self, bit_size: uint) -> BigUint {
        let mut state = RandState::new();
        let seed: c_ulong = self.gen();
        state.seed_ui(seed);
        let mpz = state.urandom_2exp(bit_size as c_ulong);
        BigUint{ data: mpz }
    }

    fn gen_biguint_below(&mut self, bound: &BigUint) -> BigUint {
        // FIXME Add assertion once is_zero is implemented
        // assert!(!bound.is_zero());

        let mut state = RandState::new();
        let seed: c_ulong = self.gen();
        state.seed_ui(seed);
        let mpz = state.urandom(&bound.data);
        BigUint{ data: mpz }
    }

    fn gen_biguint_range(&mut self, lbound: &BigUint, ubound: &BigUint) -> BigUint {
        // FIXME Add assertion once cmp::Ord is implemented
        // assert!(lbound < ubound);

        return *lbound + self.gen_biguint_below(&(*ubound - *lbound));
    }
}

#[cfg(test)]
mod test {
    use super::{BigUint, RandBigInt};
    use std::{u32,u64};
    use std::rand::task_rng;

    #[test]
    fn test_to_and_fro() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        assert_eq!(two.to_str(), ~"2");
    }

    #[test]
    fn test_printing() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        println!("{} == 2!", two); // Doesn't fail
    }

    #[test]
    fn test_add() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(two.add(&three).to_str(), ~"5");
        assert_eq!((two + three).to_str(), ~"5");
    }

    #[test]
    fn test_simple_sub() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(three.sub(&two).to_str(), ~"1");
        assert_eq!((three - two).to_str(), ~"1");
    }

    #[test]
    fn test_mul() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(two.mul(&three).to_str(), ~"6");
        assert_eq!((two * three).to_str(), ~"6");
    }

    #[test]
    fn test_gen_biguint() {
        let mut rng = task_rng();
        let rand1 = rng.gen_biguint(128);
        let rand2 = rng.gen_biguint(128);
        assert!(rand1.to_str() != rand2.to_str());
    }

    #[test]
    fn test_gen_biguint_below() {
        let mut rng = task_rng();
        let max: BigUint = FromPrimitive::from_u64(u64::MAX).unwrap();
        let rand1 = rng.gen_biguint_below(&max);
        let rand2 = rng.gen_biguint_below(&max);
        assert!(rand1.to_str() != rand2.to_str());
    }

    #[test]
    fn test_gen_biguint_range() {
        let mut rng = task_rng();
        let min: BigUint = FromPrimitive::from_u32(u32::MAX).unwrap();
        let max: BigUint = FromPrimitive::from_u64(u64::MAX).unwrap();
        let rand1 = rng.gen_biguint_range(&min, &max);
        let rand2 = rng.gen_biguint_range(&min, &max);
        assert!(rand1.to_str() != rand2.to_str());
    }
}
