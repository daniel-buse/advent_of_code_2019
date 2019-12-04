// Password facts:
//
// - It is a six-digit number.
// - The value is within the range given in your puzzle input.
// - Two adjacent digits are the same (like 22 in 122345).
// - Going from left to right, the digits never decrease; they only ever
//   increase or stay the same (like 111123 or 135679).
//
// Other than the range rule, the following are true:
//
// - 111111 meets these criteria (double 11, never decreases).
// - 223450 does not meet these criteria (decreasing pair of digits 50).
// - 123789 does not meet these criteria (no double).
//
// Part1:
//
// How many different passwords within the range given in your puzzle input meet
// these criteria?
//
// --- Part Two ---
//
// An Elf just remembered one more important detail: the two adjacent matching
// digits are not part of a larger group of matching digits.
//
// Given this additional criterion, but still ignoring the range rule, the
// following are now true:
//
// - 112233 meets these criteria because the digits never decrease and all
//   repeated digits are exactly two digits long.
// - 123444 no longer meets the criteria (the repeated 44 is part of a larger
//   group of 444).
// - 111122 meets the criteria (even though 1 is repeated more than twice, it
//   still contains a double 22).
//
// How many different passwords within the range given in your puzzle input meet
// all of the criteria?

// Adapted from https://stackoverflow.com/a/41536521 (so for the win ;))
fn to_digits(number: u32) -> Vec<u32> {
    fn inner(number: u32, result: &mut Vec<u32>) {
        if number >= 10 {
            inner(number / 10, result);
        }
        result.push(number % 10);
    }
    let mut result = Vec::new();
    inner(number, &mut result);
    result
}

fn valid_password(number: u32) -> bool {
    let digits = to_digits(number);
    digits.windows(2).all(|digits| digits[0] <= digits[1])
        && digits.windows(2).any(|digits| digits[0] == digits[1])
}

fn part1(input: (u32, u32)) {
    let (lower, upper) = input;
    let mut count = 0;
    for number in lower..=upper {
        if valid_password(number) {
            count += 1;
        }
    }
    println!("Part1: {}", count);
}

fn valid_password_part2(number: u32) -> bool {
    let digits = to_digits(number);
    if !digits.windows(2).all(|digits| digits[0] <= digits[1]) {
        return false;
    }

    for digit in digits.iter() {
        if (digits.iter().filter(|d| *d == digit).count() == 2) {
            return true;
        }
    }
    return false;
}

fn part2(input: (u32, u32)) {
    let (lower, upper) = input;
    let mut count = 0;
    for number in lower..=upper {
        if valid_password_part2(number) {
            count += 1;
        }
    }
    println!("Part2: {}", count);
}

fn main() {
    let input = (193651, 649729);
    part1(input);
    part2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_digits() {
        assert_eq!(to_digits(0), vec![0]);
        assert_eq!(to_digits(5), vec![5]);
        assert_eq!(to_digits(10), vec![1, 0]);
        assert_eq!(to_digits(193651), vec![1, 9, 3, 6, 5, 1]);
    }

    #[test]
    fn test_valid_password() {
        assert!(valid_password(111111));
        assert!(!valid_password(223450));
        assert!(!valid_password(123789));
    }

    #[test]
    fn test_valid_password_part2() {
        assert!(valid_password_part2(112233));
        assert!(!valid_password_part2(123444));
        assert!(valid_password_part2(111122));
    }
}
