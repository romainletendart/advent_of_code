use std::env::args;
use std::fs::File;
use std::{io::BufRead, io::BufReader};

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.height + x]
    }

    fn is_paper_roll(&self, x: usize, y: usize) -> bool {
        self.get(x, y) == b'@'
    }

    fn is_accessible(&self, x: usize, y: usize) -> bool {
        let relative_positions: [(i8, i8); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let paper_roll_count = relative_positions
            .iter()
            .filter(|&rel_pos| {
                if (x == 0 && rel_pos.0 < 0) || (x == (self.width - 1) && rel_pos.0 > 0) {
                    return false;
                }
                if (y == 0 && rel_pos.1 < 0) || (y == (self.height - 1) && rel_pos.1 > 0) {
                    return false;
                }
                true
            })
            .filter(|&rel_pos| {
                let abs_rel_x = rel_pos.0.unsigned_abs() as usize;
                let x_to_check = if rel_pos.0 >= 0 {
                    x + abs_rel_x
                } else {
                    x - abs_rel_x
                };
                let abs_rel_y = rel_pos.1.unsigned_abs() as usize;
                let y_to_check = if rel_pos.1 >= 0 {
                    y + abs_rel_y
                } else {
                    y - abs_rel_y
                };
                self.is_paper_roll(x_to_check, y_to_check)
            })
            .count();
        paper_roll_count < 4
    }

    fn from_reader<R: BufRead>(mut reader: R) -> Result<Self> {
        let mut buf: Vec<u8> = Vec::new();

        let num_bytes = reader.read_until(b'\n', &mut buf)?;
        if num_bytes == 0 {
            return Err(anyhow!("Found empty first line"));
        }
        if buf[buf.len() - 1] == b'\n' {
            buf.remove(buf.len() - 1);
        }
        let width = buf.len();

        let mut height = 1;
        loop {
            let num_bytes = reader.read_until(b'\n', &mut buf)?;
            if num_bytes == 0 {
                break;
            }
            if num_bytes != (width + 1) {
                return Err(anyhow!("Found line with inconsistent width"));
            }
            if buf[buf.len() - 1] == b'\n' {
                buf.remove(buf.len() - 1);
            }
            height += 1;
        }

        Ok(Self {
            data: buf,
            width,
            height,
        })
    }
}

fn solve_part_1<R: BufRead>(reader: R) -> Result<u64> {
    let grid = Grid::from_reader(reader)?;
    let candidate_positions = (0..grid.width).flat_map(|x| (0..grid.height).map(move |y| (x, y)));
    let accessible_position_count = candidate_positions
        .filter(|pos| grid.is_paper_roll(pos.0, pos.1))
        .filter(|pos| grid.is_accessible(pos.0, pos.1))
        .count();
    Ok(accessible_position_count as u64)
}

fn main() -> Result<()> {
    let input_path = args().nth(1).context("Missing input path")?;
    let input_file = File::open(&input_path).context("Couldn't open input path")?;
    let reader = BufReader::new(input_file);

    let part1 = solve_part_1(reader).context("Couldn't solve input")?;
    println!("Solution (part#1): {part1}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use std::io::Cursor;

    const EXAMPLE_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[rstest]
    fn test_grid_from_reader() {
        let input_data = ".@.\n...\n@@@\n";
        let reader = Cursor::new(input_data);

        let result = Grid::from_reader(reader);
        assert!(result.is_ok());

        let grid = result.unwrap();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(
            grid.data,
            vec![b'.', b'@', b'.', b'.', b'.', b'.', b'@', b'@', b'@']
        );
    }

    #[rstest]
    fn test_solve_part_1() {
        let reader = Cursor::new(EXAMPLE_INPUT);

        let result = solve_part_1(reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 13);
    }
}
