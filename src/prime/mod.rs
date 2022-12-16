use num::{BigUint, bigint::RandBigInt};
use rand::rngs::ThreadRng;

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
pub fn miller_rabin(value: BigUint, k: u64, rng: &mut ThreadRng) -> bool {
  let mut r: BigUint;
  let mut d: BigUint;
  let mut a: BigUint;
  let mut x: BigUint;
  let mut cont: bool;

  if (value > create_biguint(3)) && (value.modpow(&BONE, &BTWO) != *BZERO) {
    r = (*BZERO).clone();
    d = (&value) - ONE;
    loop {
      d /= TWO;
      r += ONE;
      if d.modpow(&BONE, &BTWO) != *BZERO { break; }
    }

    for _ in 0..k {
      cont = false;
      loop {
        a = rng.gen_biguint(value.bits() / 8);
        if !(a < *BTWO || a > ((&value) - TWO)) { break; }
      }

      x = a.modpow(&d, &value);
      if (x == *BONE) || (x == ((&value) - ONE)) {
        continue;
      }

      for _i in num_iter::range_inclusive((*BZERO).clone(), &r - ONE) {
        x = x.modpow(&BTWO, &value);
        if x == ((&value) - ONE) {
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

  } else if value <= *BZERO {
    return false;

  } else {  // 0 < value <= 3
    return true;
  }
}