use num::{BigUint, BigInt, bigint::RandBigInt};
use rand::rngs::ThreadRng;

/*thread_local! {
  pub static RNG: ThreadRng = rand::thread_rng();
}*/

lazy_static!(
  pub static ref BZERO: BigUint = create_biguint(0);
  pub static ref BONE: BigUint = create_biguint(1);
  pub static ref BTWO: BigUint = create_biguint(2);
);
pub static ZERO: u32 = 0;
pub static ONE: u32 = 1;
pub static TWO: u32 = 2;



/// Create a BigUint from the provided i32.
fn create_biguint(i: u32) -> BigUint {
  BigUint::from(i)
}


/// Calculates if provided number is probabilistically prime
/// using the Miller-Rabin primality test.
pub fn miller_rabin(value: BigUint, k: u32, rng: &mut ThreadRng) -> bool {
  let mut r: BigUint;
  let mut d: BigUint;
  let mut a: BigUint;
  let x: BigUint;
  let cont: bool = false;

  if (value > create_biguint(3)) && (value.modpow(&BONE, &BTWO) != *BZERO) {
    r = (*BZERO).clone();
    d = &value - ONE;
    loop {
      d /= TWO;
      r += ONE;
      if d.modpow(&BONE, &BTWO) != *BZERO { break; }
    }

    for _ in 0..k {
      loop {
        a = rng.gen_biguint(value.bits() / 8);
        if !(a < *BTWO || a > (value - TWO)) { break; }
      }
    }
  }

  false
}