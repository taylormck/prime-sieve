package main

import "core:fmt"
import "core:os"
import "core:strconv"

main :: proc() {

	if len(os.args) != 2 {
		fmt.println("You must pass an integer as the only command line argument!")
		os.exit(1)
	}

	n, ok := strconv.parse_int(os.args[1])

	if !ok {
		fmt.println("You must pass an integer as the only command line argument!")
		os.exit(1)
	}

	primes := get_primes_in_range(n)
	fmt.println(primes)
}

get_prime_from_index :: proc(n: int) -> int {
	return (n + 1) * 2 + 1
}

get_primes_in_range :: proc(n: int) -> (result: [dynamic]int) {
	if n < 2 {
		return result
	}

	sieve_size: int = (n - 1) / 2
	prime_candidates := make([]bool, sieve_size)

	for i in 0 ..< sieve_size {
		prime_candidates[i] = true
	}

	append(&result, 2)

	for is_prime, i in prime_candidates {
		if !is_prime {
			continue
		}

		prime := get_prime_from_index(i)
		append(&result, prime)

		j := i + prime

		for j < sieve_size {
			prime_candidates[j] = false
			j += prime
		}
	}

	return result
}
