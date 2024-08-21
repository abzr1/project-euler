fn main() {
    let sum = (1..1_000).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<i32>();

    println!("{}", sum);
}
