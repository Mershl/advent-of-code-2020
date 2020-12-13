
#[aoc_generator(day0)]
pub fn input_generator(input: &str) -> Vec<Lines> {
    input.lines().collect()
}

#[aoc(day0, part1)]
pub fn part1(input: &str) -> usize {
    1337
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1("Hello World"), 1);
    }
}
