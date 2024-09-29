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
    let mut m = n;
    let mut l = 0;

    while m > 0 {
        l *= 10;
        l += m % 10;
        m /= 10;
    }

    l == n
}
