use num::BigUint;
use rand::rngs::ThreadRng;

mod prime;

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate num_iter;

fn main() {
  let k: u32 = 10;
  let mut rng:ThreadRng = rand::thread_rng();
  let value = BigUint::from(65521u64);
  println!("{}", prime::miller_rabin(value, k, &mut rng))
}
