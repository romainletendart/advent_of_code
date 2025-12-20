use std::env::args;
use std::fs::File;
use std::{io::BufRead, io::BufReader};

use anyhow::Context;
use anyhow::Result;

struct Bank {
    inner: Vec<u8>,
}

impl Bank {
    fn new(inner: Vec<u8>) -> Self {
        Self { inner }
    }

    fn get_max_joltage(&self) -> u8 {
        let (ten_index, ten_value) = self.inner[..self.inner.len() - 1]
            .iter()
            .enumerate()
            .reduce(|acc, e| {
                if e.1 > acc.1 {
                    return e;
                }
                acc
            })
            .expect("Bank should have at least 2 cells");
        let unit = self.inner[ten_index + 1..]
            .iter()
            .max()
            .expect("Bank should have at least 2 cells");
        *ten_value * 10 + unit
    }

    fn get_full_max_joltage(&self) -> u64 {
        let output_len = 12;
        let mut window_len = self.inner.len() - (output_len - 1);
        let mut window_start = 0;
        let mut max_joltage = 0;
        for position in 0..output_len {
            let (in_window_position, max_digit) = self.inner
                [window_start..window_start + window_len]
                .iter()
                .enumerate()
                .reduce(|(max_position, max_digit), (position, digit)| {
                    if digit > max_digit {
                        return (position, digit);
                    }
                    (max_position, max_digit)
                })
                .expect("Bank should have at least 2 cells");
            window_start += 1 + in_window_position;
            window_len -= in_window_position;
            max_joltage += 10_u64.pow((output_len - position - 1) as u32) * (*max_digit as u64);
        }
        max_joltage
    }
}

struct Banks<R: BufRead> {
    reader: R,
}

impl<R: BufRead> Banks<R> {
    fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BufRead> Iterator for Banks<R> {
    type Item = Bank;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line_buf = String::new();
        let num_bytes = self.reader.read_line(&mut line_buf).ok()?;
        if num_bytes == 0 {
            return None;
        }
        let bank: Vec<u8> = line_buf
            .trim_end()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .expect("Only accepting digits")
                    .try_into()
                    .expect("1-char digit should fit in u8")
            })
            .collect();
        if !bank.is_empty() {
            return Some(Bank::new(bank));
        }
        None
    }
}

fn solve_part_1<R: BufRead>(reader: R) -> Result<u64> {
    let total_output_joltage = Banks::new(reader)
        .map(|bank| bank.get_max_joltage() as u64)
        .sum();
    Ok(total_output_joltage)
}

fn solve_part_2<R: BufRead>(reader: R) -> Result<u64> {
    let total_output_joltage = Banks::new(reader)
        .map(|bank| bank.get_full_max_joltage())
        .sum();
    Ok(total_output_joltage)
}

fn main() -> Result<()> {
    let input_path = args().nth(1).context("Missing input path")?;
    let input_file = File::open(&input_path).context("Couldn't open input path")?;
    let reader = BufReader::new(input_file);

    let part1 = solve_part_1(reader).context("Couldn't solve input")?;
    println!("Solution (part#1): {part1}");

    let input_file = File::open(&input_path).context("Couldn't open input path")?;
    let reader = BufReader::new(input_file);

    let part2 = solve_part_2(reader).context("Couldn't solve input")?;
    println!("Solution (part#2): {part2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    use rstest::rstest;

    #[rstest]
    #[case(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 98)]
    #[case(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 89)]
    #[case(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 78)]
    #[case(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 92)]
    #[case(vec![8, 1, 8, 1, 8, 1, 9, 6, 1, 1, 9, 2, 1, 1, 1], 99)]
    fn test_get_max_joltage(#[case] input: Vec<u8>, #[case] expected: u8) {
        assert_eq!(Bank::new(input).get_max_joltage(), expected)
    }

    #[rstest]
    #[case(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 987654321111)]
    #[case(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 811111111119)]
    #[case(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 434234234278)]
    #[case(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 888911112111)]
    fn test_get_full_max_joltage(#[case] input: Vec<u8>, #[case] expected: u64) {
        assert_eq!(Bank::new(input).get_full_max_joltage(), expected)
    }

    #[rstest]
    fn test_solve_part_1_succeeds() {
        let input_data = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
        let reader = Cursor::new(input_data);

        let result = solve_part_1(reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 357);
    }

    #[rstest]
    fn test_solve_part_2_succeeds() {
        let input_data = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
        let reader = Cursor::new(input_data);

        let result = solve_part_2(reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3121910778619);
    }
}
