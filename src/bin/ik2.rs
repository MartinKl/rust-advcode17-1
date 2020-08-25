#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_5195() {
        let seq = "5 1 9 5";
        assert_eq!(8, row_checksum(&seq));
    }
    #[test]
    fn test_7534() {
        let seq = "7 5 3 4";
        assert_eq!(4, row_checksum(&seq));
    }
    #[test]
    fn test_2468() {
        let seq = "2\t4\t6\t8";
        assert_eq!(6, row_checksum(&seq));
    }
}

fn row_checksum(seq: &str) -> i32 {
    let mut numbers = seq.split(char::is_whitespace).map(|n| n.parse().unwrap());
    let mut min: i32 = numbers.next().unwrap();
    let mut max = min;
    for number in numbers {
        if number < min {
            min = number
        }
        if number > max {
            max = number
        }
    }
    max - min
}

use std::fs;

fn main() {
    println!(
        "{:?}",
        fs::read_to_string("./inputs/day2.txt")
            .unwrap()
            .lines()
            .map(row_checksum)
            .sum::<i32>()
    );
}
