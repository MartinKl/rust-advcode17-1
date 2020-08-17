#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1122() {
        let seq = "1122";
        assert_eq!(3, run(&seq));
    }

    #[test]
    fn test_1111() {
        let seq = "1111";
        assert_eq!(4, run(&seq));
    }

    #[test]
    fn test_1234() {
        let seq = "1234";
        assert_eq!(0, run(&seq));
    }

    #[test]
    fn test_91212129() {
        let seq = "91212129";
        assert_eq!(9, run(&seq));
    }
}

fn run(sequence: &str) -> u32 {
    let mut digits = sequence.chars();
    let mut s: u32 = 0;
    let mut c: char = match digits.next() {
        Some(ch) => ch,
        _ => panic!("Unexpected!"),
    };
    for d in digits {
        s += match d == c {
            true => match d.to_digit(10) {
                None => panic!("I didn't see that coming!"),
                Some(u) => u,
            },
            false => 0,
        };
        c = d;
    }
    s += match sequence.chars().nth(0) {
        Some(cc) => match cc == c {
            true => match c.to_digit(10) {
                Some(u) => u,
                None => panic!("AH!"),
            },
            false => 0,
        },
        None => panic!("..."),
    };
    s
}

fn main() {
    println!("{}", run_iter(&String::from("1122")));
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
