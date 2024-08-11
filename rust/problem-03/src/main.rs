fn main() {
    let mut n: i64 = 600851475143;

    let mut i = 2;

    let mut factors: Vec<i64> = vec![];

    while i * i <= n {
        if n % i == 0 {
            n /= i;
            factors.push(i);
        } else {
            i += 1;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    println!("{}", factors.iter().max().unwrap());
}
