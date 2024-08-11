fn main() {
    let mut sum: i64 = 0;

    for i in 0..2_000_000 {
        if is_prime(i) {
            sum += i as i64;
        }
    }

    println!("{}", sum);
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
