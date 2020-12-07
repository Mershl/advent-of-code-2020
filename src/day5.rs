use std::collections::BTreeSet;

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

fn seat_id_from_string(input_string: &str) -> usize {
    let row = binary_step(&input_string[..7], 'F', 'B', 128);
    let column = binary_step(&input_string[7..], 'L', 'R', 8);
    row * 8 + column
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> BTreeSet<usize> {
    input.lines().map(seat_id_from_string).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &BTreeSet<usize>) -> usize {
    input.iter().max().unwrap().to_owned()
}

#[aoc(day5, part2)]
pub fn part2(input: &BTreeSet<usize>) -> usize {
    input.iter().skip(1).find(|&seat_id| !input.contains(&(seat_id + 1))).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: &str = "BFFFBBFRRR";
        let seat_id = seat_id_from_string(input);
        assert_eq!(seat_id, 567);
    }
}
