

pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    // use prime number theorem to estimate upper bound
    // sieve of Eular
    // max = (n * (ln n + ln ln n), 13) 
    // bool is_prime[max as usize + 1]; 
    // primes = Vec::new();
    let up_bound = if n < 6 {
        15
    } else {
        let n_f = n as f64;
        (n_f * (n_f.ln() + n_f.ln().ln())).ceil() as u64
    };
    let mut is_prime = vec![true; (up_bound + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    is_prime[2] = true;
    
    let mut primes = Vec::new();
    for i in 2..=up_bound {
        if is_prime[i as usize] {
            primes.push(i as u32);
        }
        for j in primes.iter() {
            if *j as u64 * i > up_bound {
                break;
            }
            is_prime[(*j as u64 * i) as usize] = false;
            if i % *j as u64 == 0 {
                break;
            }
        }
    }
    

    primes[n as usize ]

}
