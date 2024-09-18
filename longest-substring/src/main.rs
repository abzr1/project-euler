// Solution to https://leetcode.com/problems/longest-substring-without-repeating-characters/description/

fn main() {
    println!("{}", length_of_longest_substring("test".to_string()));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut seen: Vec<char> = Vec::new();
    let mut longest = 0;

    if s.len() <= 1 {
        return s.len() as i32;
    }

    for (i, i_char) in s.iter().enumerate() {
        if s.len() != 0 {
            seen.clear();
        }

        seen.push(*i_char);

        for (_j, j_char) in s[i + 1..].iter().enumerate() {
            if seen.contains(j_char) {
                if seen.len() > longest {
                    longest = seen.len();
                }

                seen.clear();
                break;
            }

            seen.push(*j_char);
        }

        if seen.len() > longest {
            longest = seen.len();
        }
    }

    longest as i32
}
