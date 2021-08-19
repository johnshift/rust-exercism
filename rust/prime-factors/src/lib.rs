pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();

    let mut current_number: u64 = n;
    'outer: while current_number != 1 {
        for i in 2..current_number + 1 {
            if current_number % i == 0 {
                prime_factors.push(i);
                current_number /= i;
                continue 'outer;
            }
        }
    }

    prime_factors
}
