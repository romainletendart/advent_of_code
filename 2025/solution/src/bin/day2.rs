use std::env::args;
use std::fs::File;
use std::{io::BufRead, io::BufReader, ops::RangeInclusive};

use anyhow::Context;
use anyhow::Result;

fn count_digits(number: u64) -> usize {
    let mut number = number;
    let mut digit_count = 0;
    while number >= 1 {
        digit_count += 1;
        number /= 10;
    }
    digit_count
}

/// Returns true if `number` has a sequence of digits that is repeacted exactly twice.
fn is_invalid_id_v1(number: u64) -> bool {
    let digit_count = count_digits(number);
    if !digit_count.is_multiple_of(2) {
        // We neeed to equally-sized sequences of digits.
        return false;
    }

    // E.g. 9898 has 4 digits thus we divide by 10² to get 98 as a quotient and 98 as a remainder.
    let divider = 10_u64.pow((digit_count / 2) as u32);
    let quotient = number / divider;
    let remainder = number % divider;
    quotient == remainder
}

/// Returns true if `number` has a sequence of digits that is repeated at least twice.
fn is_invalid_id_v2(number: u64) -> bool {
    let digit_count = count_digits(number);

    // Not all sequence lengths are to be evaluated, only those that are a multiple of digit_count.
    let possible_seq_lengths = (1..=(digit_count / 2)).filter(|l| digit_count.is_multiple_of(*l));

    for seq_len in possible_seq_lengths {
        let divider = 10_u64.pow(seq_len as u32);
        let mut number = number;
        let mut previous_remainder: Option<u64> = None;
        let mut all_remainders_are_equal = true;
        // We loop hoping to get the same remainder for all subsequent divisions by the same
        // divider.
        for _ in 0..(digit_count / seq_len) {
            let quotient = number / divider;
            let remainder = number % divider;
            if let Some(prev_remainder) = previous_remainder
                && prev_remainder != remainder
            {
                all_remainders_are_equal = false;
                break;
            }
            number = quotient;
            previous_remainder = Some(remainder);
        }
        if all_remainders_are_equal {
            return true;
        }
    }
    false
}

struct Ranges<R: BufRead> {
    reader: R,
}

impl<R: BufRead> Ranges<R> {
    fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BufRead> Iterator for Ranges<R> {
    type Item = RangeInclusive<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec![];
        if let Ok(num_bytes) = self.reader.read_until(b',', &mut buf) {
            if num_bytes == 0 {
                return None;
            }

            let mut iter = buf.into_iter();
            let start: String =
                String::from_utf8(iter.by_ref().take_while(|c| c != &b'-').collect())
                    .expect("Failed to read start");
            let end: String = String::from_utf8(
                iter.by_ref()
                    .take_while(|c| c != &b',' && c != &b'\n')
                    .collect(),
            )
            .expect("Failed to read end");

            let start: u64 = start.parse().expect("Couldn't parse start as u64");
            let end: u64 = end.parse().expect("Couldn't parse end as u64");

            return Some(RangeInclusive::new(start, end));
        }
        None
    }
}

fn solve<R: BufRead>(reader: R) -> Result<(u64, u64)> {
    let ranges = Ranges::new(reader);
    let all_ids: Vec<u64> = ranges.flatten().collect();
    let sum_of_invalid_ids_v1: u64 = all_ids
        .iter()
        .filter(|number| is_invalid_id_v1(**number))
        .sum();
    let sum_of_invalid_ids_v2: u64 = all_ids
        .iter()
        .filter(|number| is_invalid_id_v2(**number))
        .sum();
    Ok((sum_of_invalid_ids_v1, sum_of_invalid_ids_v2))
}

fn main() -> Result<()> {
    let input_path = args().nth(1).context("Missing input path")?;
    let input_file = File::open(input_path).context("Couldn't open input path")?;
    let reader = BufReader::new(input_file);

    let (part1, part2) = solve(reader).context("Couldn't solve input")?;
    println!("Solution (part#1): {part1}");
    println!("Solution (part#2): {part2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    use rstest::rstest;

    #[rstest]
    #[case(11, true)]
    #[case(99, true)]
    #[case(1010, true)]
    #[case(8888, true)]
    #[case(3, false)]
    #[case(10, false)]
    #[case(101, false)]
    #[case(1001, false)]
    fn test_is_invalid_id_v1(#[case] input: u64, #[case] expected: bool) {
        assert_eq!(is_invalid_id_v1(input), expected)
    }

    #[rstest]
    #[case(11, true)]
    #[case(99, true)]
    #[case(1010, true)]
    #[case(8888, true)]
    #[case(123123123, true)]
    #[case(1212121212, true)]
    #[case(1111111, true)]
    #[case(3, false)]
    #[case(10, false)]
    #[case(101, false)]
    #[case(1001, false)]
    fn test_is_invalid_id_v2(#[case] input: u64, #[case] expected: bool) {
        assert_eq!(is_invalid_id_v2(input), expected)
    }

    #[rstest]
    fn test_solve_succeeds() {
        let input_data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let reader = Cursor::new(input_data);

        let result = solve(reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (1227775554, 4174379265));
    }
}
