#[crate_id = "bignum#0.1.0-pre"];

#[comment = "Bignum library for Rust"];
#[crate_type = "rlib"];

#[feature(macro_rules)];

extern crate gmp;

use gmp::{Mpz, RandState};
use std::fmt;
use std::from_str::FromStr;
use std::num::{One, Zero, ToStrRadix};
use std::rand::Rng;
use std::libc::c_ulong;

#[deriving(Clone, Eq, Ord, TotalEq, TotalOrd, Zero)]
pub struct BigUint {
    data: Mpz
}

impl BigUint {
    pub fn from_str_radix(s: &str, radix: uint) -> Option<BigUint> {
        let data = Mpz::from_str_radix(s, radix);
        match data {
            Some(data) => Some(BigUint{ data: data }),
            None       => None
        }
    }

    pub fn is_odd(&self) -> bool {
        self.data.tstbit(0)
    }

    pub fn is_even(&self) -> bool {
        !self.is_odd()
    }

    pub fn divides(&self, other: &BigUint) -> bool {
        self.data.divides(&other.data)
    }

    pub fn bits(&self) -> uint {
        self.data.bit_length()
    }

    /// Implements self mod other.
    pub fn modulus(&self, other: &BigUint) -> BigUint {
        BigUint{ data: self.data.modulus(&other.data) }
    }
}


impl One for BigUint {
    fn one() -> BigUint {
        BigUint{ data: One::one() }
    }
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

macro_rules! impl_to_biguint(
    ($T:ty, $from_ty:path) => {
        impl ToBigUint for $T {
            fn to_biguint(&self) -> Option<BigUint> {
                $from_ty(*self)
            }
        }
    }
)

impl_to_biguint!(int,  FromPrimitive::from_int)
impl_to_biguint!(i8,   FromPrimitive::from_i8)
impl_to_biguint!(i16,  FromPrimitive::from_i16)
impl_to_biguint!(i32,  FromPrimitive::from_i32)
impl_to_biguint!(i64,  FromPrimitive::from_i64)
impl_to_biguint!(uint, FromPrimitive::from_uint)
impl_to_biguint!(u8,   FromPrimitive::from_u8)
impl_to_biguint!(u16,  FromPrimitive::from_u16)
impl_to_biguint!(u32,  FromPrimitive::from_u32)
impl_to_biguint!(u64,  FromPrimitive::from_u64)

impl FromStr for BigUint {
    fn from_str(s: &str) -> Option<BigUint> {
        let data: Option<Mpz> = FromStr::from_str(s);
        match data {
            Some(data) => Some(BigUint{ data: data }),
            None       => None
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
        BigUint{ data: self.data.add(&other.data) }
    }
}

impl Sub<BigUint, BigUint> for BigUint {
    fn sub(&self, other: &BigUint) -> BigUint {
        BigUint{ data: self.data.sub(&other.data) }
    }
}

impl Mul<BigUint, BigUint> for BigUint {
    fn mul(&self, other: &BigUint) -> BigUint {
        BigUint{ data: self.data.mul(&other.data) }
    }
}

impl Div<BigUint, BigUint> for BigUint {
    fn div(&self, other: &BigUint) -> BigUint {
        BigUint{ data: self.data.div(&other.data) }
    }
}

impl Rem<BigUint, BigUint> for BigUint {
    fn rem(&self, other: &BigUint) -> BigUint {
        BigUint{ data: self.data.rem(&other.data) }
    }
}

impl BitAnd<BigUint, BigUint> for BigUint {
    fn bitand(&self, other: &BigUint) -> BigUint {
        BigUint{ data: self.data.bitand(&other.data) }
    }
}

impl Shr<uint, BigUint> for BigUint {
    fn shr(&self, rhs: &uint) -> BigUint {
        let shift = *rhs as c_ulong;
        BigUint{ data: self.data.shr(&shift) }
    }
}

#[deriving(Clone, Eq, Ord, TotalEq, TotalOrd, Zero)]
pub struct BigInt {
    data: Mpz
}

impl BigInt {
    /// Implements self mod other.
    pub fn modulus(&self, other: &BigUint) -> BigUint {
        BigUint{ data: self.data.modulus(&other.data) }
    }
}


impl One for BigInt {
    fn one() -> BigInt {
        BigInt{ data: One::one() }
    }
}

impl FromPrimitive for BigInt {
    fn from_u64(other: u64) -> Option<BigInt> {
        let mpz: Option<Mpz> = FromPrimitive::from_u64(other);
        match mpz {
            Some(mpz) => Some(BigInt{ data: mpz }),
            None      => None
        }
    }

    fn from_i64(other: i64) -> Option<BigInt> {
        let mpz: Option<Mpz> = FromPrimitive::from_i64(other);
        match mpz {
            Some(mpz) => Some(BigInt{ data: mpz }),
            None      => None
        }
    }
}

impl ToStrRadix for BigInt {
    fn to_str_radix(&self, radix: uint) -> ~str {
        self.data.to_str_radix(radix)
    }
}

impl fmt::Show for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}", self.to_str_radix(10))
    }
}

impl Add<BigInt, BigInt> for BigInt {
    fn add(&self, other: &BigInt) -> BigInt {
        BigInt{ data: self.data.add(&other.data) }
    }
}

impl Sub<BigInt, BigInt> for BigInt {
    fn sub(&self, other: &BigInt) -> BigInt {
        BigInt{ data: self.data.sub(&other.data) }
    }
}

impl Mul<BigInt, BigInt> for BigInt {
    fn mul(&self, other: &BigInt) -> BigInt {
        BigInt{ data: self.data.mul(&other.data) }
    }
}

impl Div<BigInt, BigInt> for BigInt {
    fn div(&self, other: &BigInt) -> BigInt {
        BigInt{ data: self.data.div(&other.data) }
    }
}

impl Rem<BigInt, BigInt> for BigInt {
    fn rem(&self, other: &BigInt) -> BigInt {
        BigInt{ data: self.data.rem(&other.data) }
    }
}

impl ToBigUint for BigInt {
    fn to_biguint(&self) -> Option<BigUint> {
        if *self >= Zero::zero() {
            Some(BigUint{ data: self.data.clone() })
        } else {
            None
        }
    }
}

pub trait ToBigInt {
    fn to_bigint(&self) -> Option<BigInt>;
}

impl ToBigInt for BigUint {
    fn to_bigint(&self) -> Option<BigInt> {
       Some(BigInt{ data: self.data.clone() }) 
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
mod test_biguint {
    use super::{BigUint, RandBigInt, ToBigUint, ToBigInt};
    use std::{u32,u64};
    use std::from_str::FromStr;
    use std::num::{Zero, One};
    use std::rand::task_rng;

    #[test]
    fn test_clone() {
        let two = 2u.to_biguint().unwrap();
        let also_two = two.clone();
        assert_eq!(two.to_str(), also_two.to_str());
    }

    #[test]
    fn test_from_primitive() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        assert_eq!(two.to_str(), ~"2");
    }

    #[test]
    fn test_from_str() {
        let two: BigUint = FromStr::from_str("2").unwrap();
        assert_eq!(two.to_str(), ~"2");
    }

    #[test]
    fn test_from_str_radix() {
        let two = BigUint::from_str_radix("1a", 16).unwrap();
        assert_eq!(two.to_str(), ~"26");
    }

    #[test]
    fn test_to_biguint() {
        let three = 3u.to_biguint().unwrap();
        assert_eq!(three.to_str(), ~"3");
    }

    #[test]
    fn test_to_bigint() {
        let three = 3u.to_biguint().unwrap();
        let four = 4u.to_biguint().unwrap();
        assert!(three.to_bigint().unwrap() - four.to_bigint().unwrap() < Zero::zero())
    }

    #[test]
    fn test_comparisons() {
        let two = 2u.to_biguint().unwrap();
        let also_two = 2u.to_biguint().unwrap();
        let three = 3u.to_biguint().unwrap();
        assert!(two == also_two);
        assert!(two >= also_two);
        assert!(two <= also_two);
        assert!(three > two);
        assert!(three >= two);
        assert!(two < three);
        assert!(two <= three);
    }

    #[test]
    fn test_zero_and_one() {
        let zero: BigUint = Zero::zero();
        let one: BigUint = One::one();
        assert_eq!(zero.to_str(), ~"0");
        assert_eq!(one.to_str(), ~"1");
    }

    #[test]
    fn test_printing() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        println!("{} == 2!", two); // Doesn't fail
    }

    #[test]
    fn test_bits() {
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();
        assert_eq!(three.bits(), 2);

        let max64: BigUint = FromPrimitive::from_u64(u64::MAX).unwrap();
        assert_eq!(max64.bits(), 64);
        assert_eq!((max64 + three).bits(), 65);
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
    fn test_div() {
        let one: BigUint = FromPrimitive::from_uint(1).unwrap();
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();

        assert_eq!(two / two, one);
        assert_eq!(two.div(&two), one);
    }

    #[test]
    fn test_rem() {
        let one: BigUint = FromPrimitive::from_uint(1).unwrap();
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(three % two, one);
        assert_eq!(three.rem(&two), one);
    }

    #[test]
    fn test_bitand() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(two & three, two);
        assert_eq!(two.bitand(&three), two);
    }

    #[test]
    fn test_shr() {
        let one: BigUint = FromPrimitive::from_uint(1).unwrap();
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();

        assert_eq!(two >> 1, one);
        assert_eq!(two.shr(&1), one);
    }

    #[test]
    fn test_is_odd() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert!(!two.is_odd());
        assert!(three.is_odd());
    }

    #[test]
    fn test_is_even() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert!(two.is_even());
        assert!(!three.is_even());
    }

    #[test]
    fn test_divides() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();
        let six: BigUint = FromPrimitive::from_uint(6).unwrap();

        assert!(two.divides(&six));
        assert!(three.divides(&six));
        assert!(!two.divides(&three));
    }

    #[test]
    fn test_modulus() {
        let two: BigUint = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(two.modulus(&three), two);
    }

    #[test]
    fn test_rand_gen_biguint() {
        let mut rng = task_rng();
        let rand1 = rng.gen_biguint(128);
        let rand2 = rng.gen_biguint(128);
        assert!(rand1.to_str() != rand2.to_str());
    }

    #[test]
    fn test_rand_gen_biguint_below() {
        let mut rng = task_rng();
        let max: BigUint = FromPrimitive::from_u64(u64::MAX).unwrap();
        let rand1 = rng.gen_biguint_below(&max);
        let rand2 = rng.gen_biguint_below(&max);
        assert!(rand1.to_str() != rand2.to_str());
    }

    #[test]
    fn test_rand_gen_biguint_range() {
        let mut rng = task_rng();
        let min: BigUint = FromPrimitive::from_u32(u32::MAX).unwrap();
        let max: BigUint = FromPrimitive::from_u64(u64::MAX).unwrap();
        let rand1 = rng.gen_biguint_range(&min, &max);
        let rand2 = rng.gen_biguint_range(&min, &max);
        assert!(rand1.to_str() != rand2.to_str());
    }
}

#[cfg(test)]
mod test_bigint {
    use super::{BigInt, BigUint, ToBigUint};
    use std::num::{Zero, One};

    #[test]
    fn test_zero_and_one() {
        let zero: BigInt = Zero::zero();
        let one: BigInt = One::one();
        assert_eq!(zero.to_str(), ~"0");
        assert_eq!(one.to_str(), ~"1");
    }

    #[test]
    fn test_to_biguint() {
        let three: BigInt = FromPrimitive::from_int(3).unwrap();
        let minusthree: BigInt = FromPrimitive::from_int(-3).unwrap();
        assert_eq!(three.to_biguint().unwrap().to_str(), ~"3");
        assert_eq!(minusthree.to_biguint(), None);
    }

    #[test]
    fn test_add() {
        let two: BigInt = FromPrimitive::from_uint(2).unwrap();
        let three: BigInt = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(two.add(&three).to_str(), ~"5");
        assert_eq!((two + three).to_str(), ~"5");
    }

    #[test]
    fn test_simple_sub() {
        let two: BigInt = FromPrimitive::from_uint(2).unwrap();
        let three: BigInt = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(three.sub(&two).to_str(), ~"1");
        assert_eq!((three - two).to_str(), ~"1");
    }

    #[test]
    fn test_mul() {
        let two: BigInt = FromPrimitive::from_uint(2).unwrap();
        let three: BigInt = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(two.mul(&three).to_str(), ~"6");
        assert_eq!((two * three).to_str(), ~"6");
    }

    #[test]
    fn test_div() {
        let one: BigInt = FromPrimitive::from_uint(1).unwrap();
        let two: BigInt = FromPrimitive::from_uint(2).unwrap();

        assert_eq!(two / two, one);
        assert_eq!(two.div(&two), one);
    }

    #[test]
    fn test_rem() {
        let one: BigInt = FromPrimitive::from_uint(1).unwrap();
        let two: BigInt = FromPrimitive::from_uint(2).unwrap();
        let three: BigInt = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(three % two, one);
        assert_eq!(three.rem(&two), one);
    }

    #[test]
    fn test_modulus() {
        let minusone: BigInt = FromPrimitive::from_int(-1).unwrap();
        let two: BigInt = FromPrimitive::from_uint(2).unwrap();
        let three: BigUint = FromPrimitive::from_uint(3).unwrap();

        assert_eq!(two.modulus(&three), two.to_biguint().unwrap());
        assert_eq!(minusone.modulus(&three), two.to_biguint().unwrap());
    }

}
