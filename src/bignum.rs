use std::libc::{c_int,c_char,c_void,c_uint,size_t};
use std::{str,vec,fmt};
use std::mem::uninit;

struct Mpz {
    alloc: c_int,
    size: c_int,
    data: *c_void // Not sure if unsigned int, unsigned long, or unsigned long long
}

type MpzPtr = *Mpz;

#[link(name = "gmp")]
extern "C" {
    // Initializing
    fn __gmpz_init(rop: MpzPtr);
    fn __gmpz_init_set_ui(rop: MpzPtr, op: c_uint);

    // Arithmetic
    fn __gmpz_add(rop: MpzPtr, op1: MpzPtr, op2: MpzPtr);
    fn __gmpz_sub(rop: MpzPtr, op1: MpzPtr, op2: MpzPtr);
    fn __gmpz_mul(rop: MpzPtr, op1: MpzPtr, op2: MpzPtr);

    // Conversion
    fn __gmpz_sizeinbase(op: MpzPtr, base: c_int) -> size_t;
    fn __gmpz_get_str(outstr: *c_char, base: c_int, op: MpzPtr) -> *c_char;
}

struct BigUint {
    data: Mpz
}

pub trait ToBigUint {
    fn to_biguint(&self) -> Option<BigUint>;
}

impl ToBigUint for uint {
    fn to_biguint(&self) -> Option<BigUint> {
        unsafe {
            let data = uninit();
            __gmpz_init_set_ui(&data, *self as c_uint);
            Some(BigUint { data: data })
        }
    }
}

impl ToStrRadix for BigUint {
    fn to_str_radix(&self, radix: uint) -> ~str {
        unsafe {
            let len = __gmpz_sizeinbase(&self.data, radix as c_int) as uint;
            let dstptr = vec::from_elem(len, 0 as c_char).as_ptr();
            let cstr = __gmpz_get_str(dstptr as *c_char, radix as c_int, &self.data);
            str::raw::from_c_str(cstr)
        }
    }
}

impl ToStr for BigUint {
    fn to_str(&self) -> ~str { self.to_str_radix(10) }
}

impl fmt::Show for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}", self.to_str_radix(10))
    }
}

impl Add<BigUint, BigUint> for BigUint {
    fn add(&self, other: &BigUint) -> BigUint {
        unsafe {
            let dstdata = uninit();
            __gmpz_init(&dstdata);
            __gmpz_add(&dstdata, &self.data, &other.data);
            BigUint { data: dstdata }
        }
    }
}

impl Sub<BigUint, BigUint> for BigUint {
    fn sub(&self, other: &BigUint) -> BigUint {
        unsafe {
            let dstdata = uninit();
            __gmpz_init(&dstdata);
            __gmpz_sub(&dstdata, &self.data, &other.data);
            BigUint { data: dstdata }
        }
    }
}

impl Mul<BigUint, BigUint> for BigUint {
    fn mul(&self, other: &BigUint) -> BigUint {
        unsafe {
            let dstdata = uninit();
            __gmpz_init(&dstdata);
            __gmpz_mul(&dstdata, &self.data, &other.data);
            BigUint { data: dstdata }
        }
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
