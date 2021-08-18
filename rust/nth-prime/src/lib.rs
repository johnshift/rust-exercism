pub fn nth(n: u32) -> u32 {
    // bootstrap primes and next number
    let mut primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13];
    let mut current_number = 14; // next number

    // if n is less than or equal 6 (our bootstrapped primes), we return it.
    if n as usize <= 6 {
        return primes[n as usize];
    }

    // from here on out we only determine the last prime found
    // primes.len() - 1 because we need to run loop one more time
    // to determine last prime to be found
    while n as usize != primes.len() - 1 {
        // search for primes
        let mut is_prime = true;

        'primes_loop: for p in &primes {
            if current_number % *p == 0 {
                is_prime = false;
                break 'primes_loop;
            }
        }

        if is_prime {
            primes.push(current_number);
        }

        current_number += 1
    }

    // we return the last prime included
    primes[primes.len() - 1]
}
