use num::{BigUint, BigInt};


fn create_biguint(i: i32) -> BigUint {
  match BigInt::to_biguint(&BigInt::from(i)) {
    Some(v) => return v,
    None => panic!("Cannot convert {i} to BigUint"),
  }
}

/// Calculates if provided number is probabilistically prime
/// using the Miller-Rabin primality test.
pub fn miller_rabin(value: BigUint, k: u32) -> bool {
  let r: BigUint;
  let d: BigUint;
  let a: BigUint;
  let x: BigUint;
  let cont: bool = false;

  if (value > create_biguint(3)) && (value.modpow(&create_biguint(1), &create_biguint(2)) != create_biguint(0)) {

  }

  false
}