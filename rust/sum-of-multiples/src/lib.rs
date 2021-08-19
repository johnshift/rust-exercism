pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();

    for i in 1..limit {
        for factor in factors {
            if *factor == 0 {
                continue;
            }
            if i % factor == 0 {
                multiples.push(i);
            }
        }
    }

    multiples.dedup();
    multiples.into_iter().sum()
}
