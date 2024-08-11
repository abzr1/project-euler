fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 0;

    while b < 4_000_000 {
        if b % 2 == 0 {
            c += b;
        }

        let temp = a;
        a = b;
        b += temp;
    }

    println!("{}", c);
}
