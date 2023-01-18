use num::{BigUint, bigint::RandBigInt, Integer};
use rand::{rngs::ThreadRng};

use tracing::{event, span, Level};

lazy_static!(
  pub static ref BZERO: BigUint = BigUint::from(0 as u32);
  pub static ref BONE: BigUint = BigUint::from(1 as u32);
  pub static ref BTWO: BigUint = BigUint::from(2 as u32);
  pub static ref BTHREE: BigUint = BigUint::from(3 as u32);
);
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
  let mut cont: bool;
  let span = span!(Level::TRACE, "miller rabin");
  let _guard = span.enter();


  if *value > *BTHREE && !value.is_even() {
    r = (*BZERO).clone();
    d = (value) - ONE;
    loop {
      d /= TWO;
      r += ONE;
      if !value.is_even() { break; }
    }

    for _ in 0..k {
      cont = false;
      loop {
        event!(Level::TRACE, "calculating a");
        a = rng.gen_biguint(bits);
        if !(a < *BTWO || a > ((value) - TWO)) { break; }
      }

      x = a.modpow(&d, &value);
      if (x == *BONE) || (x == ((value) - ONE)) {
        continue;
      }

      event!(Level::TRACE, "calculating x");
      for _i in num_iter::range_inclusive((*BZERO).clone(), &r - ONE) {
        x = x.modpow(&BTWO, value);
        if x == ((value) - ONE) {
          cont = true;
          break;
        }
      }

      if cont {
        event!(Level::TRACE, "continuing");
        continue;
      }

      event!(Level::TRACE, "x failed");
      return false;
    }
    return true;

  } else if *value == *BONE || *value == *BTWO || *value == *BTHREE {
    return true;

  } else {
    event!(Level::TRACE, "even");
    return false;
  }
}
