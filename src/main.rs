mod prime;

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate num_iter;
extern crate stopwatch;

use std::{time::Duration, thread, sync::{mpsc::{self, Sender}, Arc, Mutex}};

use tracing::{event, span, Level};

use num::{BigUint, bigint::RandBigInt};
use rand::{rngs::ThreadRng};
use stopwatch::Stopwatch;

/// Basic help statement.
static HELP: &str = "[rusty-miller/cargo run] <run-type> <bits> <count=1> <k=10>\n
\t- run-type - determines if program run sequentially (s), in parallel (p), or both (b)
\t- bits - the number of bits of the prime number, this must be a multiple of 8, and at least 32 bits.\n
\t- count - the number of prime numbers to generate, defaults to 1\n
\t- k - the number of rounds of the Miller-Rabin primarily test to perform, defaults to 10";

/// Number of bits in a byte.
static BYTE: u64 = 8;


fn main() {
  let args: Vec<String> = std::env::args().collect();
  let mut count: u64 = 1;
  let mut k: u64 = 10;
  let bits: u64;
  let l: usize = args.len();
  let mut s: bool = false;
  let mut p: bool = false;
  let s_check = "s";
  let p_check = "p";
  let b_check = "b";

  match l {
    5 | 4 | 3 | 2 => {
      if l >= 5 {
        k = str::parse::<u64>(&args[4]).expect(&format!("k is not an unsigned number!\n{}", HELP));
      }
      if l >= 4 {
        count = str::parse::<u64>(&args[3]).expect(&format!("Count is not an unsigned number!\n{}", HELP));
      }
      if l >= 3 {
        let b = args[2] == b_check;
        s = (args[2] == s_check) || b;
        p = (args[2] == p_check) || b;
        if !s && !p {
          panic!("run-type is not valid, please specify sequential (s), parallel (p), or both (b)");
        }
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

  if count < 1 {
    panic!("count must be greater than 0!");
  }

  if p {
    println!("PARALLEL");
    threaded_gen_primes(k, count, bits);
  }

  if s {
    println!("SEQUENTIAL");
    gen_primes(k, count, bits);
  }
}


/// Generate primes concurrently.
pub fn threaded_gen_primes(k: u64, count: u64, bits: u64) {

  let sw: Stopwatch = Stopwatch::start_new();
  let (sender, receiver) = mpsc::channel();
  let gen_count: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

  // generate threads
  for _ in 0..20 {
    let s = sender.clone();
    let c = Arc::clone(&gen_count);

    thread::spawn(move || {
      let mut value: BigUint;
      let mut r: ThreadRng = rand::thread_rng();
      let span = span!(Level::TRACE, "generating prime");
      let _guard = span.enter();

      loop {
        value = r.gen_biguint(bits);  // TODO: handle error
        if prime::miller_rabin(&value, k, &mut r) {
          event!(Level::TRACE, "found a prime");
          {
            let mut curr_count = c.lock().unwrap();
            if *curr_count < count {
              println!("{}", value);
              *curr_count += 1;
            }

            if *curr_count == count {
              s.send(true).unwrap();
            }
          }
          //s.send(value.clone()).unwrap();  // TODO: handle error
        }
      }
    });
  }

  receiver.recv().unwrap();
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
      if prime::miller_rabin(&value.clone(), k, &mut rng) {
        curr_time = sw.elapsed() - prev_time;
        println!("[{}ms]\n{}: {}\n", curr_time.as_millis(), n, value);
        n += 1;
        break;
      }
    }
  }
  println!("Net generation time: {}s", sw.elapsed().as_secs_f64());
}

#[cfg(test)]
mod parallel_tests {
  use crate::threaded_gen_primes;
  use tracing::Level;
use tracing_subscriber::FmtSubscriber;

  #[test]
  pub fn the_test() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let k = 10;
    let count = 3;
    let bits = 1000;
    threaded_gen_primes(k, count, bits);
  }
}