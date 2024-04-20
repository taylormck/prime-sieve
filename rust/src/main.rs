mod primes;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("You must pass an integer as the first parameter");
        std::process::exit(1);
    }

    let n = &args[1];

    let n: usize = match n.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("You must pass an integer as the first parameter");
            std::process::exit(1);
        }
    };

    let result = primes::get_primes_in_range(n);

    println!("{:?}", result);
}
