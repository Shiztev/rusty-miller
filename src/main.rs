mod prime;
mod seq_mr;

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate num_iter;
extern crate stopwatch;

use std::{time::Duration, thread, sync::{Arc, Mutex, mpsc::{self, Sender}}};

use num::{BigUint, bigint::RandBigInt};
use rand::{rngs::ThreadRng, SeedableRng};
use stopwatch::Stopwatch;

/// Basic help statement.
static HELP: &str = "[rusty-miller/cargo run] <bits> <count=1> <k=10>\n
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

  gen_primes(k, count, bits);
  //threaded_gen_primes(k, count, bits);
}


/// Generate primes concurrently.
fn threaded_gen_primes(k: u64, count: u64, bits: u64) {
  let mut curr_time: Duration;
  let mut prev_time: Duration;
  let sw: Stopwatch = Stopwatch::start_new();
  // TODO: PROBLEM <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< StdRng is SUPER SLOWWWWWWWWWWWW
  let rng: Arc<Mutex<rand::rngs::StdRng>> = Arc::new(Mutex::new(rand::rngs::StdRng::from_entropy()));
  let (sender, receiver) = mpsc::channel();
  let mut v: BigUint;
  let mut n: u64 = 1;

  // generate threads
  for _ in 0..20 {
    let r: Arc<Mutex<rand::rngs::StdRng>> = Arc::clone(&rng);
    let s = sender.clone();

    thread::spawn(move || {
      let mut value: BigUint;
      loop {
        value = r.lock().unwrap().gen_biguint(bits);  // TODO: handle error
        if prime::miller_rabin(&value, k, &r) {
          s.send(value.clone()).unwrap();  // TODO: handle error
        }
      }
    });
  }

  while n <= count {  // while needing more primes
    // msg passing
    prev_time = sw.elapsed();
    v = receiver.recv().unwrap();
    curr_time = sw.elapsed() - prev_time;
    println!("[{}ms]\n{}: {}\n", curr_time.as_millis(), n, v);
    n += 1;
  }
  println!("Net generation time: {}s", sw.elapsed().as_secs_f64());

  // terminate all threads
}


/// Generate primes sequentially.
fn gen_primes(k: u64, count: u64, bits: u64) {
  let mut rng: ThreadRng = rand::thread_rng();
  let mut value: BigUint;
  let mut n: u64 = 1;
  let mut curr_time: Duration;
  let mut prev_time: Duration;
  let sw: Stopwatch = Stopwatch::start_new();


  for _ in 0..count {
    prev_time = sw.elapsed();
    loop {
      value = rng.gen_biguint(bits);
      if seq_mr::miller_rabin(&value.clone(), k, &mut rng) {
        curr_time = sw.elapsed() - prev_time;
        println!("[{}ms]\n{}: {}\n", curr_time.as_millis(), n, value);
        n += 1;
        break;
      }
    }
  }
  println!("Net generation time: {}s", sw.elapsed().as_secs_f64());
}