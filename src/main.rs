use num::{BigUint, bigint::RandBigInt};
use rand::rngs::ThreadRng;

mod prime;

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate num_iter;

/// Basic help statement.
static HELP: &str = "[rusty-miller/cargo run] <bits> <count=1>\n
\t- bits - the number of bits of the prime number, this must be a multiple of 8, and at least 32 bits.\n
\t- count - the number of prime numbers to generate, defaults to 1\n
\t- k - the number of rounds of the Miller-Rabin primarily test to perform, defaults to 10";

/// Number of bits in a byte.
static BYTE: u64 = 8;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let mut rng: ThreadRng = rand::thread_rng();
  let mut count: u64 = 1;
  let mut k: u64 = 10;
  let bits: u64;
  let l: usize = args.len();

  match l {
    4 | 3 | 2 => {
      if l == 4 {
        k = str::parse::<u64>(&args[3]).expect(&format!("k is not an unsigned number!\n{}", HELP));
      }
      if l == 3 {
        count = str::parse::<u64>(&args[2]).expect(&format!("Count is not an unsigned number!\n{}", HELP));
      }
      bits = str::parse::<u64>(&args[1]).expect(&format!("Bit length is not an unsigned number!\n{}", HELP));
    },

    _ => {
      panic!("{HELP}");
    }
  }

  if (bits % BYTE) != 0 {
    panic!("bit length of {} is not divisible by 8!\n{}", bits, HELP);
  }

  let value: BigUint = rng.gen_biguint(bits / BYTE);

  println!("{}: is prime = {}", &value, prime::miller_rabin(value.clone(), k, &mut rng))
}
