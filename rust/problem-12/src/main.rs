fn main() {
    for i in 1.. {
        if divisors(triangle_num(i)) > 500 {
            println!("{}", triangle_num(i));
            break;
        }
    }
}

fn triangle_num(nth: i32) -> i32 {
    (0..=nth).sum()
}

fn divisors(n: i32) -> i32 {
    (1..=n / 2)
        .filter(|x| n % x == 0)
        .chain(vec![n])
        .collect::<Vec<_>>()
        .len()
        .try_into()
        .unwrap()
}
