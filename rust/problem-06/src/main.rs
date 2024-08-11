fn main() {
    let sum_of_sq = (1i32..=100).map(|x| x.pow(2)).sum::<i32>();
    let sq_of_sum = (1..=100).sum::<i32>().pow(2);
    println!("{}", sq_of_sum - sum_of_sq);
}
