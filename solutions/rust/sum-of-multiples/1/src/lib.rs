pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut  set = std::collections::HashSet::new();
    if factors.is_empty() {
        return 0;
    }
    for val in factors {
        if *val == 0 {
            continue;
        }
        let mut multiple = 1;
        while val * multiple < limit {
            set.insert(val * multiple);
            multiple += 1;
        }
    }
    set.iter().sum()
}
