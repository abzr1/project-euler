fn main() {
    let mut largest = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            if is_palindrome(i * j) && i * j > largest {
                largest = i * j;
            }
        }
    }

    println!("{}", largest);
}

fn is_palindrome(n: i32) -> bool {
    n == n
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}
