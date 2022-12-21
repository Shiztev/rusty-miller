use std::sync::{Arc, Mutex};

use num::{BigUint, bigint::RandBigInt};
use rand::rngs::StdRng;

lazy_static!(
  pub static ref BZERO: BigUint = create_biguint(0);
  pub static ref BONE: BigUint = create_biguint(1);
  pub static ref BTWO: BigUint = create_biguint(2);
);
pub static ONE: u32 = 1;
pub static TWO: u32 = 2;



/// Create a BigUint from the provided i32.
fn create_biguint(i: u32) -> BigUint {
  BigUint::from(i)
}


/// Calculates if provided number is probabilistically prime
/// using the Miller-Rabin primality test.
pub fn miller_rabin(value: &BigUint, k: u64, rng: &Arc<Mutex<rand::rngs::StdRng>>) -> bool {
  let bits: u64 = value.bits();
  let mut r: BigUint;
  let mut d: BigUint;
  let mut a: BigUint;
  let mut x: BigUint;
  let mut cont: bool;

  if *value <= *BZERO {
    return false;

  } else if !(*value > create_biguint(3)) {  // 0 < value <= 3
    return true;

  } else if value.modpow(&BONE, &BTWO) != *BZERO {  // implicitly value > create_biguint(3)
    r = (*BZERO).clone();
    d = (value) - ONE;
    loop {
      d /= TWO;
      r += ONE;
      if d.modpow(&BONE, &BTWO) != *BZERO { break; }
    }

    for _ in 0..k {
      cont = false;
      loop {
        a = rng.lock().unwrap().gen_biguint(bits);
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
  }
  false
}