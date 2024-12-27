struct Solution;
const ANY_CHAR: char = '.';
const UNIQUE_CHAR: char = '*';
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() && !s.is_empty() {
            return false;
        }
        let mut s_iter = s.chars();
        let mut p_iter = p.chars();

        let mut current_pattern: char = p_iter.next().unwrap();
        let mut current_remembered: char = s_iter.next().unwrap();
        let mut is_repeating = false;

        loop {
            println!("is repating {is_repeating}");
            if is_repeating {
                println!("is_repeating");
                if current_pattern != ANY_CHAR && current_pattern != current_remembered {
                    let Some(next_pattern) = p_iter.next() else {
                        println!("return false");
                        return false;
                    };

                    println!("stop repeating");
                    is_repeating = false;
                    // NOTE: if there is **, this is illegal, so I do not consider that
                    current_pattern = next_pattern;
                }
                let Some(next_remembered) = s_iter.next() else {
                    println!("stop repating return false");
                    return p_iter.next().is_none();
                };
                current_remembered = next_remembered;
                continue;
            }
            if current_pattern != ANY_CHAR && current_pattern != current_remembered {
                println!("stop repating return false");
                return false;
            }
            println!("{current_remembered},{current_pattern}");
            match (p_iter.next(), s_iter.next()) {
                (Some(UNIQUE_CHAR), Some(next_remembered)) => {
                    is_repeating = true;
                    current_remembered = next_remembered;
                }
                (Some(next_pattern), Some(next_remembered)) => {
                    current_remembered = next_remembered;
                    current_pattern = next_pattern;
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (None, None) => {
                    return true;
                }
            }
        }
    }
}

#[test]
fn check() {
    assert!(Solution::is_match("aaaa".to_string(), "a*".to_string()));
    assert!(Solution::is_match(
        "aaaabbb".to_string(),
        "a*b*".to_string()
    ));
    assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    assert!(Solution::is_match("abcd".to_string(), "ab.*".to_string()));
}
