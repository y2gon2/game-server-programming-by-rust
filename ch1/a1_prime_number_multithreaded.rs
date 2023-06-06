use std::sync::Mutex;
use std::thread;

const MAX_COUNT: i32 = 150000;
const  THREADCOUNT: i32 = 4;

fn is_prime_number (num: i32) -> bool {
    if num == 1 {
        return false;
    }
        
    if num == 2 || num == 3 {
        return true;
    }

    for i in 2..num {
        if (num % i) == 0 {
            return false;
        }
    }

    return true;
}

fn print_numbers (primes: &Vec<i32>) {
    for p in primes.iter() {
        println!("{}", p);
    }
}

fn main() -> Result<()> {
    let mut n = 1;

}