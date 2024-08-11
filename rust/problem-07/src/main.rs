fn main() {
    let mut primes: Vec<i32> = vec![];
    let mut i = 0;

    while primes.len() < 10_001 {
        if is_prime(i) {
            primes.push(i);
        }

        i += 1;
    }

    println!("{}", primes.last().unwrap());
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        } else {
            i += 1;
        }
    }

    true
}
