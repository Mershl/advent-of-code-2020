
pub struct BoardingPass {
    row: usize,
    column: usize,
    seat_id: usize
}

impl BoardingPass {
    fn from_string(input_string: &str) -> BoardingPass {
        let row = BoardingPass::binary_step(&input_string[..7], 'F', 'B', 128);
        let column = BoardingPass::binary_step(&input_string[7..], 'L', 'R', 8);
        BoardingPass {
            row,
            column,
            seat_id: row * 8 + column
        }
    }

    fn binary_step(binary_string: &str, left_char: char, right_char: char, range: usize) -> usize {
        let mut lower = 0;
        let mut upper = range;

        binary_string.chars().for_each(|c| {
            match c {
                _ if (c == left_char) => { upper -= (upper - lower) / 2; },
                _ if (c == right_char) => { lower += (upper - lower) / 2; },
                _ => unreachable!()
            }
        });
        lower
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|l| BoardingPass::from_string(l)).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[BoardingPass]) -> usize {
    input.iter().max_by(|a, b| a.seat_id.cmp(&b.seat_id)).unwrap().seat_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: &str = "BFFFBBFRRR";
        let boarding_pass = BoardingPass::from_string(input);
        assert_eq!(boarding_pass.row, 70);
        assert_eq!(boarding_pass.column, 7);
        assert_eq!(boarding_pass.seat_id, 567);
    }
}
