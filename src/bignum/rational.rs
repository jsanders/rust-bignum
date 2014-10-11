use gmp::Mpq;
use std::num::One;
use super::bigint::BigInt;

#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord, Zero)]
pub struct BigRational {
    data: Mpq
}

impl BigRational {
    pub fn new() -> BigRational {
        BigRational{ data: Mpq::new() }
    }

    pub fn from_integer(t: BigInt) -> BigRational {
        let mut res = BigRational::new();
        res.data.set_z(&t.data);
        res
    }
}

impl One for BigRational {
    fn one() -> BigRational {
        BigRational{ data: One::one() }
    }
}

impl Add<BigRational, BigRational> for BigRational {
    fn add(&self, other: &BigRational) -> BigRational {
        BigRational{ data: self.data.add(&other.data) }
    }
}

impl Mul<BigRational, BigRational> for BigRational {
    fn mul(&self, other: &BigRational) -> BigRational {
        BigRational{ data: self.data.mul(&other.data) }
    }
}

