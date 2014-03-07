#[crate_id = "gmp#0.1.0"];

#[comment = "gmp bindings"];
#[license = "MIT"];
#[crate_type = "lib"];

#[feature(globs)];
#[allow(non_camel_case_types)];

use std::libc::{c_char, c_double, c_int, c_long, c_ulong, c_void, size_t};
use std::num::{One, Zero, ToStrRadix};
use std::mem::{uninit,size_of};
use std::{cmp, vec, fmt};
use std::from_str::FromStr;

struct mpz_struct {
    _mp_alloc: c_int,
    _mp_size: c_int,
    _mp_d: *c_void
}

struct mpq_struct {
    _mp_num: mpz_struct,
    _mp_den: mpz_struct
}

type mp_exp_t = c_long;

struct mpf_struct {
    _mp_prec: c_int,
    _mp_size: c_int,
    _mp_exp: mp_exp_t,
    _mp_d: *c_void
}

struct gmp_randstate_struct {
    _mp_seed: mpz_struct,
    _mp_alg: c_int,
    _mp_algdata: *c_void
}

type mp_bitcnt_t = c_ulong;
type mpz_srcptr = *mpz_struct;
type mpz_ptr = *mut mpz_struct;
type mpq_srcptr = *mpq_struct;
type mpq_ptr = *mut mpq_struct;
type mpf_srcptr = *mpf_struct;
type mpf_ptr = *mut mpf_struct;
type gmp_randstate_t = *mut gmp_randstate_struct;

#[link(name = "gmp")]
extern "C" {
    fn __gmpz_init(x: mpz_ptr);
    fn __gmpz_init2(x: mpz_ptr, n: mp_bitcnt_t);
    fn __gmpz_init_set(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_init_set_ui(rop: mpz_ptr, op: c_ulong);
    fn __gmpz_init_set_str(rop: mpz_ptr, str: *c_char, base: c_int) -> c_int;
    fn __gmpz_clear(x: mpz_ptr);
    fn __gmpz_realloc2(x: mpz_ptr, n: mp_bitcnt_t);
    fn __gmpz_set(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_set_str(rop: mpz_ptr, str: *c_char, base: c_int) -> c_int;
    fn __gmpz_get_str(str: *mut c_char, base: c_int, op: mpz_srcptr) -> *c_char;
    fn __gmpz_sizeinbase(op: mpz_srcptr, base: c_int) -> size_t;
    fn __gmpz_cmp(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    fn __gmpz_cmp_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
    fn __gmpz_add(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_sub(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_mul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_mul_2exp(rop: mpz_ptr, op1: mpz_srcptr, op2: mp_bitcnt_t);
    fn __gmpz_neg(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_abs(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_tdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    fn __gmpz_tdiv_r(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    fn __gmpz_fdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);
    fn __gmpz_mod(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    fn __gmpz_divisible_p(n: mpz_srcptr, d: mpz_srcptr) -> c_int;
    fn __gmpz_and(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_ior(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_xor(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_com(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_popcount(op: mpz_srcptr) -> mp_bitcnt_t;
    fn __gmpz_hamdist(op1: mpz_srcptr, op2: mpz_srcptr) -> mp_bitcnt_t;
    fn __gmpz_setbit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    fn __gmpz_clrbit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    fn __gmpz_combit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    fn __gmpz_tstbit(rop: mpz_srcptr, bit_index: mp_bitcnt_t) -> c_int;
    fn __gmpz_gcd(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_lcm(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_invert(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    fn __gmpz_import(rop: mpz_ptr, count: size_t, order: c_int, size: size_t,
                     endian: c_int, nails: size_t, op: *c_void);
    fn __gmp_randinit_default(state: gmp_randstate_t);
    fn __gmp_randinit_mt(state: gmp_randstate_t);
    fn __gmp_randinit_lc_2exp(state: gmp_randstate_t, a: mpz_srcptr, c: c_ulong, m2exp: mp_bitcnt_t);
    fn __gmp_randinit_lc_2exp_size(state: gmp_randstate_t, size: mp_bitcnt_t);
    fn __gmp_randinit_set(state: gmp_randstate_t, op: *gmp_randstate_struct);
    fn __gmp_randclear(state: gmp_randstate_t);
    fn __gmp_randseed(state: gmp_randstate_t, seed: mpz_srcptr);
    fn __gmp_randseed_ui(state: gmp_randstate_t, seed: c_ulong);
    fn __gmpz_urandomb(rop: mpz_ptr, state: gmp_randstate_t, n: mp_bitcnt_t);
    fn __gmpz_urandomm(rop: mpz_ptr, state: gmp_randstate_t, n: mpz_srcptr);
    fn __gmpq_init(x: mpq_ptr);
    fn __gmpq_clear(x: mpq_ptr);
    fn __gmpq_set(rop: mpq_ptr, op: mpq_srcptr);
    fn __gmpq_set_z(rop: mpq_ptr, op: mpz_srcptr);
    fn __gmpq_set_ui(rop: mpq_ptr, op1: c_ulong, op2: c_ulong);
    fn __gmpq_set_d(rop: mpq_ptr, op: c_double);
    fn __gmpq_set_f(rop: mpq_ptr, op: mpf_srcptr);
    fn __gmpq_cmp(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    fn __gmpq_cmp_ui(op1: mpq_srcptr, num2: c_ulong, den2: c_ulong) -> c_int;
    fn __gmpq_equal(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    fn __gmpq_add(sum: mpq_ptr, addend1: mpq_srcptr, addend2: mpq_srcptr);
    fn __gmpq_sub(difference: mpq_ptr, minuend: mpq_srcptr, subtrahend: mpq_srcptr);
    fn __gmpq_mul(product: mpq_ptr, multiplier: mpq_srcptr, multiplicand: mpq_srcptr);
    fn __gmpq_div(product: mpq_ptr, multiplier: mpq_srcptr, multiplicand: mpq_srcptr);
    fn __gmpq_neg(negated_operand: mpq_ptr, operand: mpq_srcptr);
    fn __gmpq_abs(rop: mpq_ptr, op: mpq_srcptr);
    fn __gmpq_inv(inverted_number: mpq_ptr, number: mpq_srcptr);
    fn __gmpq_get_num(numerator: mpz_ptr, rational: mpq_srcptr);
    fn __gmpq_get_den(denominator: mpz_ptr, rational: mpq_srcptr);
    fn __gmpf_init2(x: mpf_ptr, prec: mp_bitcnt_t);
    fn __gmpf_init_set(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_clear(x: mpf_ptr);
    fn __gmpf_get_prec(op: mpf_srcptr) -> mp_bitcnt_t;
    fn __gmpf_set_prec(rop: mpf_ptr, prec: mp_bitcnt_t);
    fn __gmpf_set(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_cmp(op1: mpf_srcptr, op2: mpf_srcptr) -> c_int;
    fn __gmpf_cmp_d(op1: mpf_srcptr, op2: c_double) -> c_int;
    fn __gmpf_cmp_ui(op1: mpf_srcptr, op2: c_ulong) -> c_int;
    fn __gmpf_reldiff(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_add(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_sub(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_mul(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_div(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_neg(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_abs(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_ceil(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_floor(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_trunc(rop: mpf_ptr, op: mpf_srcptr);
}

pub struct Mpz {
    priv mpz: mpz_struct,
}

impl Drop for Mpz {
    fn drop(&mut self) { unsafe { __gmpz_clear(&mut self.mpz) } }
}

impl Mpz {
    pub fn new() -> Mpz {
        unsafe {
            let mut mpz = uninit();
            __gmpz_init(&mut mpz);
            Mpz { mpz: mpz }
        }
    }

    pub fn new_reserve(n: c_ulong) -> Mpz {
        unsafe {
            let mut mpz = uninit();
            __gmpz_init2(&mut mpz, n);
            Mpz { mpz: mpz }
        }
    }

    pub fn reserve(&mut self, n: c_ulong) {
        if (self.bit_length() as c_ulong) < n {
            unsafe { __gmpz_realloc2(&mut self.mpz, n) }
        }
    }

    pub fn from_str_radix(s: &str, base: uint) -> Option<Mpz> {
        unsafe {
            assert!(base == 0 || (base >= 2 && base <= 62));
            let mut mpz = uninit();
            let r = s.with_c_str(|s| {
                __gmpz_init_set_str(&mut mpz, s, base as c_int)
            });
            if r == 0 {
                Some(Mpz { mpz: mpz })
            } else {
                __gmpz_clear(&mut mpz);
                None
            }
        }
    }

    pub fn set(&mut self, other: &Mpz) {
        unsafe { __gmpz_set(&mut self.mpz, &other.mpz) }
    }

    // TODO: too easy to forget to check this return value - rename?
    pub fn set_from_str_radix(&mut self, s: &str, base: uint) -> bool {
        assert!(base == 0 || (base >= 2 && base <= 62));
        s.with_c_str(|s| {
            unsafe { __gmpz_set_str(&mut self.mpz, s, base as c_int) == 0 }
        })
    }

    pub fn bit_length(&self) -> uint {
        unsafe { __gmpz_sizeinbase(&self.mpz, 2) as uint }
    }

    pub fn compl(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_com(&mut res.mpz, &self.mpz);
            res
        }
    }

    pub fn abs(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_abs(&mut res.mpz, &self.mpz);
            res
        }
    }

    pub fn gcd(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_gcd(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }

    pub fn lcm(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_lcm(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }

    pub fn divides(&self, other: &Mpz) -> bool {
        unsafe {
            __gmpz_divisible_p(&other.mpz, &self.mpz) != 0
        }
    }

    pub fn modulus(&self, modulo: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_mod(&mut res.mpz, &self.mpz, &modulo.mpz);
            res
        }
    }

    // TODO: handle a zero modulo
    pub fn invert(&self, modulo: &Mpz) -> Option<Mpz> {
        unsafe {
            let mut res = Mpz::new();
            if __gmpz_invert(&mut res.mpz, &self.mpz, &modulo.mpz) == 0 {
                None
            } else {
                Some(res)
            }
        }
    }

    pub fn popcount(&self) -> uint {
        unsafe { __gmpz_popcount(&self.mpz) as uint }
    }

    pub fn hamdist(&self, other: &Mpz) -> uint {
        unsafe { __gmpz_hamdist(&self.mpz, &other.mpz) as uint }
    }

    pub fn setbit(&mut self, bit_index: c_ulong) {
        unsafe { __gmpz_setbit(&mut self.mpz, bit_index) }
    }

    pub fn clrbit(&mut self, bit_index: c_ulong) {
        unsafe { __gmpz_clrbit(&mut self.mpz, bit_index) }
    }

    pub fn combit(&mut self, bit_index: c_ulong) {
        unsafe { __gmpz_combit(&mut self.mpz, bit_index) }
    }

    pub fn tstbit(&self, bit_index: c_ulong) -> bool {
        unsafe { __gmpz_tstbit(&self.mpz, bit_index) == 1 }
    }
}

impl Clone for Mpz {
    fn clone(&self) -> Mpz {
        unsafe {
            let mut mpz = uninit();
            __gmpz_init_set(&mut mpz, &self.mpz);
            Mpz { mpz: mpz }
        }
    }
}

impl TotalEq for Mpz {
    fn equals(&self, other: &Mpz) -> bool {
        let cmp = unsafe { __gmpz_cmp(&self.mpz, &other.mpz) };
        cmp == 0
    }
}

impl Eq for Mpz {
    fn eq(&self, other: &Mpz) -> bool {
        self.equals(other)
    }
}

impl TotalOrd for Mpz {
    fn cmp(&self, other: &Mpz) -> Ordering {
        let cmp = unsafe { __gmpz_cmp(&self.mpz, &other.mpz) };

        match cmp {
            x if x < 0 => Less,
            x if x > 0 => Greater,
            _ => Equal
        }
    }
}

impl Ord for Mpz {
    fn lt(&self, other: &Mpz) -> bool {
        self.cmp(other) == Less
    }
}

impl Add<Mpz, Mpz> for Mpz {
    fn add(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_add(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl Sub<Mpz, Mpz> for Mpz {
    fn sub(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_sub(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl Mul<Mpz, Mpz> for Mpz {
    fn mul(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_mul(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl Div<Mpz, Mpz> for Mpz {
    fn div(&self, other: &Mpz) -> Mpz {
        unsafe {
            if self.is_zero() {
                fail!(~"divide by zero")
            }

            let mut res = Mpz::new();
            __gmpz_tdiv_q(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl Rem<Mpz, Mpz> for Mpz {
    fn rem(&self, other: &Mpz) -> Mpz {
        unsafe {
            if self.is_zero() {
                fail!(~"divide by zero")
            }

            let mut res = Mpz::new();
            __gmpz_tdiv_r(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl Neg<Mpz> for Mpz {
    fn neg(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_neg(&mut res.mpz, &self.mpz);
            res
        }
    }
}

impl ToPrimitive for Mpz {
    fn to_i64(&self) -> Option<i64> {
        fail!(~"not implemented")
    }
    fn to_u64(&self) -> Option<u64> {
        fail!(~"not implemented")
    }
}

impl FromPrimitive for Mpz {
    fn from_u64(other: u64) -> Option<Mpz> {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_import(&mut res.mpz, 1, 1, size_of::<u64>() as size_t, 0, 0,
                          &other as *u64 as *c_void);
            Some(res)
        }
    }
    fn from_i64(other: i64) -> Option<Mpz> {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_import(&mut res.mpz, 1, 1, size_of::<i64>() as size_t, 0, 0,
                          &other.abs() as *i64 as *c_void);
            if other.is_negative() {
                __gmpz_neg(&mut res.mpz, &res.mpz)
            }
            Some(res)
        }
    }
}

impl One for Mpz {
    fn one() -> Mpz {
        unsafe {
            let mut mpz = uninit();
            __gmpz_init_set_ui(&mut mpz, 1);
            Mpz { mpz: mpz }
        }
    }
}

impl Zero for Mpz {
    fn zero() -> Mpz { Mpz::new() }
    fn is_zero(&self) -> bool {
        unsafe { __gmpz_cmp_ui(&self.mpz, 0) == 0 }
    }
}

impl BitAnd<Mpz, Mpz> for Mpz {
    fn bitand(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_and(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl BitOr<Mpz, Mpz> for Mpz {
    fn bitor(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_ior(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl BitXor<Mpz, Mpz> for Mpz {
    fn bitxor(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_xor(&mut res.mpz, &self.mpz, &other.mpz);
            res
        }
    }
}

impl Shl<c_ulong, Mpz> for Mpz {
    fn shl(&self, other: &c_ulong) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_mul_2exp(&mut res.mpz, &self.mpz, *other);
            res
        }
    }
}

impl Shr<c_ulong, Mpz> for Mpz {
    fn shr(&self, other: &c_ulong) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_fdiv_q_2exp(&mut res.mpz, &self.mpz, *other);
            res
        }
    }
}

impl FromStr for Mpz {
    fn from_str(s: &str) -> Option<Mpz> {
        Mpz::from_str_radix(s, 10)
    }
}

impl ToStrRadix for Mpz {
    // TODO: fail on an invalid base
    fn to_str_radix(&self, base: uint) -> ~str {
        unsafe {
            // Extra two bytes are for possible minus sign and null terminator
            let len = __gmpz_sizeinbase(&self.mpz, base as c_int) as uint + 2;

            // Allocate and write into a raw *c_char of the correct length
            let mut vector: ~[u8] = vec::with_capacity(len);
            vector.set_len(len);

            let mut cstr = vector.to_c_str_unchecked();
            cstr.with_mut_ref(|raw| -> () {
                __gmpz_get_str(raw, base as c_int, &self.mpz);
            });

            match cstr.as_str() {
                Some(slice) => slice.to_owned(),
                None        => fail!("GMP returned invalid UTF-8!")
            }
        }
    }
}

impl fmt::Show for Mpz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}", self.to_str_radix(10))
    }
}


pub struct RandState {
    priv state: gmp_randstate_struct,
}

impl Drop for RandState {
    fn drop(&mut self) {
        unsafe { __gmp_randclear(&mut self.state) }
    }
}

impl RandState {
    pub fn new() -> RandState {
        unsafe {
            let mut state: gmp_randstate_struct = uninit();
            __gmp_randinit_default(&mut state);
            RandState { state: state }
        }
    }

    pub fn new_mt() -> RandState {
        unsafe {
            let mut state: gmp_randstate_struct = uninit();
            __gmp_randinit_mt(&mut state);
            RandState { state: state }
        }
    }

    pub fn new_lc_2exp(a: Mpz, c: c_ulong, m2exp: c_ulong) -> RandState {
        unsafe {
            let mut state: gmp_randstate_struct = uninit();
            __gmp_randinit_lc_2exp(&mut state, &a.mpz, c, m2exp);
            RandState { state: state }
        }
    }

    pub fn new_lc_2exp_size(size: c_ulong) -> RandState {
        unsafe {
            let mut state: gmp_randstate_struct = uninit();
            __gmp_randinit_lc_2exp_size(&mut state, size);
            RandState { state: state }
        }
    }

    pub fn seed(&mut self, seed: Mpz) {
        unsafe { __gmp_randseed(&mut self.state, &seed.mpz) }
    }

    pub fn seed_ui(&mut self, seed: c_ulong) {
        unsafe { __gmp_randseed_ui(&mut self.state, seed) }
    }

    /// Generate a uniform random integer in the range 0 to n-1, inclusive
    pub fn urandom(&mut self, n: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_urandomm(&mut res.mpz, &mut self.state, &n.mpz);
            res
        }
    }

    /// Generate a uniformly distributed random integer in the range 0 to 2^n−1, inclusive.
    pub fn urandom_2exp(&mut self, n: c_ulong) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_urandomb(&mut res.mpz, &mut self.state, n);
            res
        }
    }
}

impl Clone for RandState {
    fn clone(&self) -> RandState {
        unsafe {
            let mut state: gmp_randstate_struct = uninit();
            __gmp_randinit_set(&mut state, &self.state);
            RandState { state: state }
        }
    }
}

pub struct Mpq {
    priv mpq: mpq_struct,
}

impl Drop for Mpq {
    fn drop(&mut self) { unsafe { __gmpq_clear(&mut self.mpq) } }
}

impl Mpq {
    pub fn new() -> Mpq {
        unsafe {
            let mut mpq = uninit();
            __gmpq_init(&mut mpq);
            Mpq { mpq: mpq }
        }
    }

    pub fn set(&mut self, other: &Mpq) {
        unsafe { __gmpq_set(&mut self.mpq, &other.mpq) }
    }

    pub fn set_z(&mut self, other: &Mpz) {
        unsafe { __gmpq_set_z(&mut self.mpq, &other.mpz) }
    }

    pub fn set_d(&mut self, other: f64) {
        unsafe { __gmpq_set_d(&mut self.mpq, other) }
    }

    pub fn set_f(&mut self, other: &Mpf) {
        unsafe { __gmpq_set_f(&mut self.mpq, &other.mpf) }
    }

    pub fn get_num(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpq_get_num(&mut res.mpz, &self.mpq);
            res
        }
    }

    pub fn get_den(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpq_get_den(&mut res.mpz, &self.mpq);
            res
        }
    }

    pub fn abs(&self) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_abs(&mut res.mpq, &self.mpq);
            res
        }
    }

    pub fn invert(&self) -> Mpq {
        unsafe {
            if self.is_zero() {
                fail!(~"divide by zero")
            }

            let mut res = Mpq::new();
            __gmpq_inv(&mut res.mpq, &self.mpq);
            res
        }
    }
}

impl Clone for Mpq {
    fn clone(&self) -> Mpq {
        let mut res = Mpq::new();
        res.set(self);
        res
    }
}

impl cmp::Eq for Mpq {
    fn eq(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_equal(&self.mpq, &other.mpq) != 0 }
    }
    fn ne(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_equal(&self.mpq, &other.mpq) == 0 }
    }
}

impl cmp::Ord for Mpq {
    fn lt(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(&self.mpq, &other.mpq) < 0 }
    }
    fn le(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(&self.mpq, &other.mpq) <= 0 }
    }
    fn gt(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(&self.mpq, &other.mpq) > 0 }
    }
    fn ge(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(&self.mpq, &other.mpq) >= 0 }
    }
}

impl Add<Mpq, Mpq> for Mpq {
    fn add(&self, other: &Mpq) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_add(&mut res.mpq, &self.mpq, &other.mpq);
            res
        }
    }
}

impl Sub<Mpq, Mpq> for Mpq {
    fn sub(&self, other: &Mpq) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_sub(&mut res.mpq, &self.mpq, &other.mpq);
            res
        }
    }
}

impl Mul<Mpq, Mpq> for Mpq {
    fn mul(&self, other: &Mpq) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_mul(&mut res.mpq, &self.mpq, &other.mpq);
            res
        }
    }
}

impl Div<Mpq, Mpq> for Mpq {
    fn div(&self, other: &Mpq) -> Mpq {
        unsafe {
            if self.is_zero() {
                fail!(~"divide by zero")
            }

            let mut res = Mpq::new();
            __gmpq_div(&mut res.mpq, &self.mpq, &other.mpq);
            res
        }
    }
}

impl Neg<Mpq> for Mpq {
    fn neg(&self) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_neg(&mut res.mpq, &self.mpq);
            res
        }
    }
}

impl ToPrimitive for Mpq {
    fn to_i64(&self) -> Option<i64> {
        fail!(~"not implemented")
    }
    fn to_u64(&self) -> Option<u64> {
        fail!(~"not implemented")
    }
}

impl FromPrimitive for Mpq {
    fn from_i64(other: i64) -> Option<Mpq> {
        let mut res = Mpq::new();
        res.set_z(&FromPrimitive::from_i64(other).unwrap());
        Some(res)
    }
    fn from_u64(other: u64) -> Option<Mpq> {
        let mut res = Mpq::new();
        res.set_z(&FromPrimitive::from_u64(other).unwrap());
        Some(res)
    }
}

impl One for Mpq {
    fn one() -> Mpq {
        let mut res = Mpq::new();
        unsafe { __gmpq_set_ui(&mut res.mpq, 1, 1) }
        res
    }
}

impl Zero for Mpq {
    fn zero() -> Mpq { Mpq::new() }
    fn is_zero(&self) -> bool {
        unsafe { __gmpq_cmp_ui(&self.mpq, 0, 1) == 0 }
    }
}

pub struct Mpf {
    priv mpf: mpf_struct,
}

impl Drop for Mpf {
    fn drop(&mut self) { unsafe { __gmpf_clear(&mut self.mpf) } }
}

impl Mpf {
    pub fn new(precision: c_ulong) -> Mpf {
        unsafe {
            let mut mpf = uninit();
            __gmpf_init2(&mut mpf, precision);
            Mpf { mpf: mpf }
        }
    }

    pub fn set(&mut self, other: &Mpf) {
        unsafe { __gmpf_set(&mut self.mpf, &other.mpf) }
    }

    pub fn get_prec(&self) -> c_ulong {
        unsafe { __gmpf_get_prec(&self.mpf) }
    }

    pub fn set_prec(&mut self, precision: c_ulong) {
        unsafe { __gmpf_set_prec(&mut self.mpf, precision) }
    }

    pub fn abs(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_abs(&mut res.mpf, &self.mpf);
            res
        }
    }

    pub fn ceil(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_ceil(&mut res.mpf, &self.mpf);
            res
        }
    }

    pub fn floor(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_floor(&mut res.mpf, &self.mpf);
            res
        }
    }

    pub fn trunc(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_trunc(&mut res.mpf, &self.mpf);
            res
        }
    }

    pub fn reldiff(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(cmp::max(self.get_prec() as uint,
                                         other.get_prec() as uint) as c_ulong);
            __gmpf_reldiff(&mut res.mpf, &self.mpf, &other.mpf);
            res
        }
    }
}

impl Clone for Mpf {
    fn clone(&self) -> Mpf {
        unsafe {
            let mut mpf = uninit();
            __gmpf_init_set(&mut mpf, &self.mpf);
            Mpf { mpf: mpf }
        }
    }
}

impl cmp::Eq for Mpf {
    fn eq(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(&self.mpf, &other.mpf) == 0 }
    }
    fn ne(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(&self.mpf, &other.mpf) != 0 }
    }
}

impl cmp::Ord for Mpf {
    fn lt(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(&self.mpf, &other.mpf) < 0 }
    }
    fn le(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(&self.mpf, &other.mpf) <= 0 }
    }
    fn gt(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(&self.mpf, &other.mpf) > 0 }
    }
    fn ge(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(&self.mpf, &other.mpf) >= 0 }
    }
}

impl Add<Mpf, Mpf> for Mpf {
    fn add(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(cmp::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_add(&mut res.mpf, &self.mpf, &other.mpf);
            res
        }
    }
}

impl Sub<Mpf, Mpf> for Mpf {
    fn sub(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(cmp::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_sub(&mut res.mpf, &self.mpf, &other.mpf);
            res
        }
    }
}

impl Mul<Mpf, Mpf> for Mpf {
    fn mul(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(cmp::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_mul(&mut res.mpf, &self.mpf, &other.mpf);
            res
        }
    }
}

impl Div<Mpf, Mpf> for Mpf {
    fn div(&self, other: &Mpf) -> Mpf {
        unsafe {
            if __gmpf_cmp_ui(&self.mpf, 0) == 0 {
                fail!(~"divide by zero")
            }

            let mut res = Mpf::new(cmp::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_div(&mut res.mpf, &self.mpf, &other.mpf);
            res
        }
    }
}

impl Neg<Mpf> for Mpf {
    fn neg(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_neg(&mut res.mpf, &self.mpf);
            res
        }
    }
}

#[cfg(test)]
mod test_mpz {
    use super::*;
    use std::from_str::FromStr;
    use std::num::ToStrRadix;
    use std::num::One;
    use std::libc::c_ulong;

    #[test]
    fn test_set() {
        let mut x: Mpz = FromPrimitive::from_int(1000).unwrap();
        let y: Mpz = FromPrimitive::from_int(5000).unwrap();
        assert!(x != y);
        x.set(&y);
        assert!(x == y);
    }

    #[test]
    fn test_set_from_str_radix() {
        let mut x: Mpz = FromPrimitive::from_int(1000).unwrap();
        let y: Mpz = FromPrimitive::from_int(5000).unwrap();
        assert!(x != y);
        assert!(x.set_from_str_radix("5000", 10));
        assert!(x == y);
        assert!(!x.set_from_str_radix("aaaa", 2));
    }

    #[test]
    #[should_fail]
    fn test_from_str_radix_lower_bound() {
        Mpz::from_str_radix("", 1);
    }

    #[test]
    #[should_fail]
    fn test_from_str_radix_upper_bound() {
        Mpz::from_str_radix("", 63);
    }

    #[test]
    #[should_fail]
    fn test_set_from_str_radix_lower_bound() {
        let mut x = Mpz::new();
        x.set_from_str_radix("", 1);
    }

    #[test]
    #[should_fail]
    fn test_set_from_str_radix_upper_bound() {
        let mut x = Mpz::new();
        x.set_from_str_radix("", 63);
    }

    #[test]
    fn test_eq() {
        let x: Mpz = FromPrimitive::from_int(4242142195).unwrap();
        let y: Mpz = FromPrimitive::from_int(4242142195).unwrap();
        let z: Mpz = FromPrimitive::from_int(4242142196).unwrap();

        assert!(x == y);
        assert!(x != z);
        assert!(y != z);
    }

    #[test]
    fn test_ord() {
        let x: Mpz = FromStr::from_str("40000000000000000000000").unwrap();
        let y: Mpz = FromStr::from_str("45000000000000000000000").unwrap();
        let z: Mpz = FromStr::from_str("50000000000000000000000").unwrap();

        assert!(x < y && x < z && y < z);
        assert!(x <= x && x <= y && x <= z && y <= z);
        assert!(z > y && z > x && y > x);
        assert!(z >= z && z >= y && z >= x && y >= x);
    }

    #[test]
    #[should_fail]
    fn test_div_zero() {
        let x = Mpz::new();
        x / x;
    }

    #[test]
    #[should_fail]
    fn test_rem_zero() {
        let x = Mpz::new();
        x % x;
    }

    #[test]
    fn test_div_round() {
        let x: Mpz = FromPrimitive::from_int(2).unwrap();
        let y: Mpz = FromPrimitive::from_int(3).unwrap();
        assert!((x / y).to_str() == (2i / 3).to_str());
        assert!((x / -y).to_str() == (2i / -3).to_str());
    }

    #[test]
    fn test_rem() {
        let x: Mpz = FromPrimitive::from_int(20).unwrap();
        let y: Mpz = FromPrimitive::from_int(3).unwrap();
        assert!((x % y).to_str() == (20i % 3).to_str());
        assert!((x % -y).to_str() == (20i % -3).to_str());
        assert!((-x % y).to_str() == (-20i % 3).to_str());
    }

    #[test]
    fn test_to_str_radix() {
        let x: Mpz = FromPrimitive::from_int(255).unwrap();
        assert!(x.to_str_radix(16) == ~"ff");
    }

    #[test]
    fn test_to_str() {
        let x: Mpz = FromStr::from_str("1234567890").unwrap();
        assert!(x.to_str() == ~"1234567890");
    }

    #[test]
    fn test_invalid_str() {
        let x: Option<Mpz> = FromStr::from_str("foobar");
        assert!(x.is_none());
    }

    #[test]
    fn test_clone() {
        let a: Mpz = FromPrimitive::from_int(100).unwrap();
        let b = a.clone();
        let aplusb: Mpz = FromPrimitive::from_int(200).unwrap();
        assert!(b == a);
        assert!(a + b == aplusb);
    }

    #[test]
    fn test_from_int() {
        let x: Mpz = FromPrimitive::from_int(150).unwrap();
        assert!(x.to_str() == ~"150");
        assert!(x == FromStr::from_str("150").unwrap());
    }

    #[test]
    fn test_abs() {
        let x: Mpz = FromPrimitive::from_int(1000).unwrap();
        let y: Mpz = FromPrimitive::from_int(-1000).unwrap();
        assert!(-x == y);
        assert!(x == -y);
        assert!(x == y.abs());
        assert!(x.abs() == y.abs());
    }

    #[test]
    fn test_bitand() {
        let a = 0b1001_0111;
        let b = 0b1100_0100;
        let mpza: Mpz = FromPrimitive::from_int(a).unwrap();
        let mpzb: Mpz = FromPrimitive::from_int(b).unwrap();
        let mpzres: Mpz = FromPrimitive::from_int(a & b).unwrap();
        assert!(mpza & mpzb == mpzres);
    }

    #[test]
    fn test_bitor() {
        let a = 0b1001_0111;
        let b = 0b1100_0100;
        let mpza: Mpz = FromPrimitive::from_int(a).unwrap();
        let mpzb: Mpz = FromPrimitive::from_int(b).unwrap();
        let mpzres: Mpz = FromPrimitive::from_int(a | b).unwrap();
        assert!(mpza | mpzb == mpzres);
    }

    #[test]
    fn test_bitxor() {
        let a = 0b1001_0111;
        let b = 0b1100_0100;
        let mpza: Mpz = FromPrimitive::from_int(a).unwrap();
        let mpzb: Mpz = FromPrimitive::from_int(b).unwrap();
        let mpzres: Mpz = FromPrimitive::from_int(a ^ b).unwrap();
        assert!(mpza ^ mpzb == mpzres);
    }

    #[test]
    fn test_shifts() {
        let i = 227;
        let j: Mpz = FromPrimitive::from_int(i).unwrap();
        assert!((i << 4).to_str() == (j << 4).to_str());
        assert!((-i << 4).to_str() == (-j << 4).to_str());
        assert!((i >> 4).to_str() == (j >> 4).to_str());
        assert!((-i >> 4).to_str() == (-j >> 4).to_str());
    }

    #[test]
    fn test_compl() {
        let a: Mpz = FromPrimitive::from_int(13).unwrap();
        let b: Mpz = FromPrimitive::from_int(-442).unwrap();
        assert!(a.compl().to_str() == (!13i).to_str());
        assert!(b.compl().to_str() == (!-442i).to_str());
    }

    #[test]
    fn test_popcount() {
        Mpz::from_str_radix("1010010011", 2).unwrap().popcount() == 5;
    }

    #[test]
    fn test_hamdist() {
        let a: Mpz = FromPrimitive::from_int(0b1011_0001).unwrap();
        let b: Mpz = FromPrimitive::from_int(0b0010_1011).unwrap();
        assert!(a.hamdist(&b) == 4);
    }

    #[test]
    fn test_bit_length() {
        let a: Mpz = FromPrimitive::from_int(0b1011_0000_0001_0000).unwrap();
        let b: Mpz = FromPrimitive::from_int(0b101).unwrap();
        assert!(a.bit_length() == 16);
        assert!(b.bit_length() == 3);
    }

    #[test]
    fn test_gcd() {
        let zero: Mpz = FromPrimitive::from_int(0).unwrap();
        let three: Mpz = FromPrimitive::from_int(3).unwrap();
        let six: Mpz = FromPrimitive::from_int(6).unwrap();
        let eighteen: Mpz = FromPrimitive::from_int(18).unwrap();
        let twentyfour: Mpz = FromPrimitive::from_int(24).unwrap();
        assert!(zero.gcd(&zero) == zero);
        assert!(three.gcd(&six) == three);
        assert!(eighteen.gcd(&twentyfour) == six);
    }

    #[test]
    fn test_lcm() {
        let zero: Mpz = FromPrimitive::from_int(0).unwrap();
        let three: Mpz = FromPrimitive::from_int(3).unwrap();
        let five: Mpz = FromPrimitive::from_int(5).unwrap();
        let six: Mpz = FromPrimitive::from_int(6).unwrap();
        let eighteen: Mpz = FromPrimitive::from_int(18).unwrap();
        let twentyfour: Mpz = FromPrimitive::from_int(24).unwrap();
        let seventytwo: Mpz = FromPrimitive::from_int(72).unwrap();
        assert!(zero.lcm(&five) == zero);
        assert!(five.lcm(&zero) == zero);
        assert!(three.lcm(&six) == six);
        assert!(eighteen.lcm(&twentyfour) == seventytwo);
    }

    #[test]
    fn test_divides() {
        let two: Mpz = FromPrimitive::from_int(2).unwrap();
        let three: Mpz = FromPrimitive::from_int(3).unwrap();
        let six: Mpz = FromPrimitive::from_int(6).unwrap();
        assert!(two.divides(&six));
        assert!(three.divides(&six));
        assert!(!two.divides(&three));
    }

    #[test]
    fn test_modulus() {
        let minusone: Mpz = FromPrimitive::from_int(-1).unwrap();
        let two: Mpz = FromPrimitive::from_int(2).unwrap();
        let three: Mpz = FromPrimitive::from_int(3).unwrap();
        assert_eq!(two.modulus(&three), two);
        assert_eq!(minusone.modulus(&three), two);
    }

    #[test]
    fn test_invert() {
        let two: Mpz = FromPrimitive::from_int(2).unwrap();
        let three: Mpz = FromPrimitive::from_int(3).unwrap();
        let four: Mpz = FromPrimitive::from_int(4).unwrap();
        let five: Mpz = FromPrimitive::from_int(5).unwrap();
        let eleven: Mpz = FromPrimitive::from_int(11).unwrap();
        assert!(three.invert(&eleven) == Some(four.clone()));
        assert!(four.invert(&eleven) == Some(three.clone()));
        assert!(two.invert(&five) == Some(three.clone()));
        assert!(three.invert(&five) == Some(two.clone()));
        assert!(two.invert(&four).is_none());
    }

    #[test]
    fn test_one() {
        let onea: Mpz = One::one();
        let oneb: Mpz = FromPrimitive::from_int(1).unwrap();
        assert!(onea == oneb);
    }

    #[test]
    fn test_bit_fiddling() {
        let mut xs: Mpz = FromPrimitive::from_int(0b1010_1000_0010_0011).unwrap();
        assert!(xs.bit_length() == 16);
        let mut ys = [true, false, true, false,
                      true, false, false, false,
                      false, false, true, false,
                      false, false, true, true];
        ys.reverse();
        for i in range(0, xs.bit_length()) {
            assert!(xs.tstbit(i as c_ulong) == ys[i]);
        }
        xs.setbit(0);
        ys[0] = true;
        xs.setbit(3);
        ys[3] = true;
        xs.clrbit(1);
        ys[1] = false;
        xs.clrbit(5);
        ys[5] = false;
        xs.combit(14);
        ys[14] = !ys[14];
        xs.combit(15);
        ys[15] = !ys[15];
        for i in range(0, xs.bit_length()) {
            assert!(xs.tstbit(i as c_ulong) == ys[i]);
        }
    }
}

#[cfg(test)]
mod test_rand {
    use super::*;

    #[test]
    fn test_randstate() {
        let mut state = RandState::new();
        state.seed_ui(42);
        for _ in range(1u, 1000) {
            for x in range(1i, 10) {
                let upper: Mpz = FromPrimitive::from_int(x).unwrap();
                assert!(state.urandom(&upper) < upper);
            }
        }
    }
}

#[cfg(test)]
mod test_mpq {
    use super::*;
    use std::num::One;

    #[test]
    fn test_one() {
        let onea: Mpq = One::one();
        let oneb: Mpq = FromPrimitive::from_int(1).unwrap();
        assert!(onea == oneb);
    }

    #[test]
    #[should_fail]
    fn test_div_zero() {
        let x = Mpq::new();
        x / x;
    }

    #[test]
    #[should_fail]
    fn test_invert_zero() {
        Mpq::new().invert();
    }
}

#[cfg(test)]
mod test_mpf {
    use super::*;

    #[test]
    #[should_fail]
    fn test_div_zero() {
        let x = Mpf::new(100);
        x / x;
    }
}

