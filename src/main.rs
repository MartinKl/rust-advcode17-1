use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1122() {
        let seq = "1122";
        assert_eq!(3, check(&seq));
    }

    #[test]
    fn test_1111() {
        let seq = "1111";
        assert_eq!(4, check(&seq));
    }

    #[test]
    fn test_1234() {
        let seq = "1234";
        assert_eq!(0, check(&seq));
    }

    #[test]
    fn test_91212129() {
        let seq = "91212129";
        assert_eq!(9, check(&seq));
    }
}

const BASE: u32 = 10;


fn check(sequence: &str) -> u32 {
    let mut s: u32 = 0;
    let chars: Vec<_> = sequence.chars().collect();
    let chars_ = [&chars[1..chars.len()], &chars[0..1]].concat();
    for i in 0..chars.len() {
        s += match chars[i] == chars_[i] {
            false => 0,
            true => match chars[i].to_digit(BASE) {
                Some(u) => u,
                None => panic!("{:?} not a number of base {}", chars[i], BASE)
            }
        }
    } 
    s
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        let v = check(arg.as_str());
        println!("{} has checksum {}", &arg, &v);
    }
}

/// An attempt to solve the same puzzle with just iterators.
fn run_iter(seq: &str) -> u32 {
    seq.chars()
        .zip(seq.chars().cycle().skip(1))
        .filter_map(|pair: (char, char)| -> Option<u32> {
            let (c1, c2) = pair;
            match c1 == c2 {
                true => c2.to_digit(10),
                false => None,
            }
        })
        .sum()
}
