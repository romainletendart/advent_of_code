use anyhow::{Context, Result, anyhow};
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Position {
    position: u64,
    range: RangeInclusive<u64>,
    ended_at_start_count: u64,
    visited_start_count: u64,
}

impl Position {
    fn new(start: u64, range: RangeInclusive<u64>) -> Self {
        Self {
            position: start,
            range,
            ended_at_start_count: 0,
            visited_start_count: 0,
        }
    }

    fn move_left(&mut self, distance: u64) {
        let mut visited_start = false;
        {
            let mut distance = distance % (self.range.end() - self.range.start() + 1);
            let distance_to_start = self.position - self.range.start();
            if distance > distance_to_start {
                if distance_to_start > 0 {
                    visited_start = true;
                }
                distance -= distance_to_start;
                self.position = self.range.end() - (distance - 1)
            } else {
                self.position -= distance;
                if &self.position == self.range.start() {
                    visited_start = true;
                }
            }
        }
        self.update_counters(distance, visited_start);
    }

    fn move_right(&mut self, distance: u64) {
        let mut visited_start = false;
        {
            let mut distance = distance % (self.range.end() - self.range.start() + 1);
            let distance_to_end = self.range.end() - self.position;
            if distance > distance_to_end {
                visited_start = true;
                distance -= distance_to_end;
                self.position = self.range.start() + (distance - 1)
            } else {
                self.position += distance
            }
        }
        self.update_counters(distance, visited_start);
    }

    fn update_counters(&mut self, distance: u64, visited_start: bool) {
        self.visited_start_count += distance / (self.range.end() - self.range.start() + 1);
        if &self.position == self.range.start() {
            self.ended_at_start_count += 1;
        }
        if visited_start {
            self.visited_start_count += 1;
        }
    }
}

fn solve<R: BufRead>(reader: R) -> Result<(u64, u64)> {
    let mut dial_position = Position::new(50, 0..=99);
    for result in reader.lines() {
        let line = result?;
        let mut word_chars = line.trim().chars();
        let direction = word_chars.next().context("Expected direction")?;
        let distance: u64 = word_chars.collect::<String>().parse()?;

        match direction {
            'L' => dial_position.move_left(distance),
            'R' => dial_position.move_right(distance),
            direction => return Err(anyhow!("Unsupported {direction}")),
        }
    }
    Ok((
        dial_position.ended_at_start_count,
        dial_position.visited_start_count,
    ))
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
mod tests {
    use std::io::Cursor;

    use crate::Position;

    #[test]
    fn test_move_left() {
        let test_cases = [
            (11, 0..=99, 8, 3, 0, 0),
            (11, 0..=99, 108, 3, 0, 1),
            (0, 0..=99, 1, 99, 0, 0),
            (1, 0..=99, 1, 0, 1, 1),
            (6, 3..=10, 2, 4, 0, 0),
            (6, 3..=10, 6, 8, 0, 1),
            (6, 3..=10, 14, 8, 0, 2),
        ];
        for (start, range, distance, new_position, ended_at_start_count, visited_start_count) in
            test_cases
        {
            let mut position = Position::new(start, range.clone());
            position.move_left(distance);
            assert_eq!(
                position.position, new_position,
                "start={start}, range={range:?}, distance={distance}, position != {new_position}"
            );
            assert_eq!(
                position.ended_at_start_count, ended_at_start_count,
                "start={start}, range={range:?}, distance={distance}, ended_at_start_count != {ended_at_start_count}"
            );
            assert_eq!(
                position.visited_start_count, visited_start_count,
                "start={start}, range={range:?}, distance={distance}, visited_start_count != {visited_start_count}"
            );
        }
    }

    #[test]
    fn test_move_right() {
        let test_cases = [
            (11, 0..=99, 8, 19, 0, 0),
            (11, 0..=99, 108, 19, 0, 1),
            (99, 0..=99, 1, 0, 1, 1),
            (6, 3..=10, 2, 8, 0, 0),
            (6, 3..=10, 6, 4, 0, 1),
            (6, 3..=10, 12, 10, 0, 1),
        ];
        for (start, range, distance, new_position, ended_at_start_count, visited_start_count) in
            test_cases
        {
            let mut position = Position::new(start, range.clone());
            position.move_right(distance);
            assert_eq!(
                position.position, new_position,
                "start={start}, range={range:?}, distance={distance}, position != {new_position}"
            );
            assert_eq!(
                position.ended_at_start_count, ended_at_start_count,
                "start={start}, range={range:?}, distance={distance}, ended_at_start_count != {ended_at_start_count}"
            );
            assert_eq!(
                position.visited_start_count, visited_start_count,
                "start={start}, range={range:?}, distance={distance}, visited_start_count != {visited_start_count}"
            );
        }
    }

    #[test]
    fn test_solve_succeeds() {
        use super::*;

        let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        let reader = Cursor::new(test_input);

        let result = solve(reader);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (3, 6));
    }
}
