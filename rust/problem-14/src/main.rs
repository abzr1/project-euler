fn main() {
    let mut max_len = 0;
    let mut ans = 0;

    for i in 1..=1_000_000 {
        let mut j: i64 = i;
        let mut chain_len = 1;

        while j != 1 {
            if j % 2 == 0 {
                j /= 2;
                chain_len += 1;
            } else {
                j = 3 * j + 1;
                chain_len += 1;
            }
        }

        if chain_len > max_len {
            max_len = chain_len;
            ans = i;
        }
    }

    println!("{}", ans);
}
