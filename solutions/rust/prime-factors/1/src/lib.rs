pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut remainder = n;

    let mut divisor: u64 = 2;
    while divisor * divisor <= remainder {
        while remainder % divisor == 0 {
            result.push(divisor);
            remainder /= divisor;
        }
        if divisor == 2 {
            divisor = 3;
        } else {
            divisor += 2;
        }
    }

    if remainder > 1 {
        result.push(remainder);
    }

    result
}
