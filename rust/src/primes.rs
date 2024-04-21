fn get_prime_from_index(n: usize) -> usize {
    (n + 1) * 2 + 1
}

pub fn get_primes_in_range(n: usize) -> Vec<usize> {
    if (n < 2) {
        return vec![];
    }

    let sieve_size = (n - 1) / 2;
    let mut sieve = vec![true; sieve_size];
    let mut result: Vec<usize> = vec![2];

    // TODO make sure to return an empty list when n is less than 2

    for i in 0..sieve_size {
        if !sieve[i] {
            continue;
        }

        let prime = get_prime_from_index(i);
        result.push(prime);

        for j in (i + prime..sieve_size).step_by(prime) {
            sieve[j] = false;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::primes::get_primes_in_range;

    #[test]
    fn it_gets_the_primes_from_2_to_100() {
        assert_eq!(
            get_primes_in_range(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn it_gets_the_last_prime_at_the_end_of_the_range() {
        assert_eq!(get_primes_in_range(2), vec![2]);
        assert_eq!(get_primes_in_range(3), vec![2, 3]);
        assert_eq!(get_primes_in_range(5), vec![2, 3, 5]);
        assert_eq!(get_primes_in_range(7), vec![2, 3, 5, 7]);
    }

    #[test]
    fn it_returns_empty_when_the_range_does_not_include_any_primes() {
        assert_eq!(get_primes_in_range(0), vec![]);
        assert_eq!(get_primes_in_range(1), vec![]);
    }
}
