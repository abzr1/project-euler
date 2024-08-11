fn main() {
    println!("{}", compute());
}

fn compute() -> i32 {
    for i in 1..998 {
        for j in 1..998 {
            for k in 1..998 {
                if i + j + k == 1000 && (i * i) + (j * j) == k * k {
                    return i * j * k;
                }
            }
        }
    }

    0
}
