use std::time::{Instant};

use prime_factors::{
    find_factors,
    find_factors_sieve,
    multiply_vector,
};

use sieve::{
    sieve_of_eratosthenes,
    sieve_to_primes,
};


// We'll use primes up to this value.
//const MAX_PRIME: usize = 10_000_000;
const MAX_PRIME: usize = 50_000_000;

fn main() {
    // Build a vector of primes.
    let start0 = Instant::now();
    let sieve = sieve_of_eratosthenes(MAX_PRIME);
    let primes = sieve_to_primes(&sieve);
    let duration0 = start0.elapsed();
    println!("Build sieve: {:?} seconds\n", duration0);

    // Multiply two big primes so we have something interesting to factor.
    //let big_number = 987654103_i64 * 45269999_i64;
    //println!("{} * {} = {}",  987654103_i64, 45269999_i64, big_number);

    // loop {
    {
        // Get the number to factor.
        //let num = get_i64("Number to factor: ");
        let num = 987654103_usize * 45269999_usize;

        // Find the factors the slow way.
        let start1 = Instant::now();
        let factors1 = find_factors(num);
        let duration1 = start1.elapsed();
        println!("find_factors: {:?} seconds", duration1);
        //print_numbers(&mut factors1);
        println!("{:?}", factors1);
        println!("Product: {}", multiply_vector(&factors1));
        println!();

        // Use the Euler's sieve to find the factors.
        let start2 = Instant::now();
        let factors2 = find_factors_sieve(num, &primes);
        let duration2 = start2.elapsed();
        println!("find_factors_sieve: {:?} seconds", duration2);
        println!("{:?}", factors2);
        println!("Product: {}", multiply_vector(&factors2));
        println!();
    }
}

