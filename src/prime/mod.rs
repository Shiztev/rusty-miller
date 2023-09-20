use num::{BigUint, bigint::RandBigInt, Integer};
use rand::rngs::ThreadRng;
use num::Zero;

use crate::primelist;

lazy_static!(
  pub static ref BZERO: BigUint = BigUint::from(0 as u32);
  pub static ref BONE: BigUint = BigUint::from(1 as u32);
  pub static ref BTWO: BigUint = BigUint::from(2 as u32);
  pub static ref BTHREE: BigUint = BigUint::from(3 as u32);
);
pub static ZERO: u32 = 0;
pub static ONE: u32 = 1;
pub static TWO: u32 = 2;


/// Calculates if provided number is probabilistically prime
/// using the Miller-Rabin primality test.
pub fn miller_rabin(value: &BigUint, k: u64, rng: &mut ThreadRng) -> bool {
  let bits: u64 = value.bits();
  let mut r: BigUint;
  let mut d: BigUint;
  let mut a: BigUint;
  let mut x: BigUint;
  let mut cont: bool = false;


  if value > &BTHREE && !value.is_even() {
    // check known prime cases
    for u in primelist::PRIMES {
      if (value % u).is_zero() {
        return false;
      }
    }

    r = BigUint::from(ZERO);
    d = (value) - ONE;
    loop {
      d /= TWO;
      r += ONE;
      if !d.is_even() { break; }
    }

    for _ in 0..k {
      loop {
        a = rng.gen_biguint(bits);
        if !(a < *BTWO || a > ((value) - TWO)) { break; }
      }

      x = a.modpow(&d, &value);
      if (x == *BONE) || (x == ((value) - ONE)) {
        continue;
      }

      for _i in num_iter::range_inclusive((*BZERO).clone(), &r - ONE) {
        x = x.modpow(&BTWO, value);
        if x == ((value) - ONE) {
          cont = true;
          break;
        }
      }

      if cont {
        continue;
      }

      return false;
    }
    return true;

  } else if *value == *BONE || *value == *BTWO || *value == *BTHREE {
    return true;

  } else {
    return false;
  }
}
