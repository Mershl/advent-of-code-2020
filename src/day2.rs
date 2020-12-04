
#[derive(Debug)]
pub struct PasswordEntry {
    condition_a: usize,
    condition_b: usize,
    letter: char,
    password: String
}

impl PasswordEntry {
    fn is_occurence_valid(&self) -> bool {
        let occurences = self.password.chars().filter(|&c| c == self.letter).count();
        occurences >= self.condition_a && occurences <= self.condition_b
    }

    fn is_exactly_one_at_positions_valid(&self) -> bool {
        let match_a = self.password.chars().nth(self.condition_a - 1).unwrap() == self.letter;
        let match_b = self.password.chars().nth(self.condition_b - 1).unwrap() == self.letter;
        match_a ^ match_b
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordEntry> {
    input.lines().map(|line| {
        let mut splits = line.split([':', '-', ' '].as_ref()).filter(|s| !s.is_empty());
        PasswordEntry {
            condition_a: splits.next().unwrap().parse().unwrap(),
            condition_b: splits.next().unwrap().parse().unwrap(),
            letter: splits.next().unwrap().parse().unwrap(),
            password: splits.next().unwrap().to_string()
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[PasswordEntry]) -> usize {
    input.iter().filter(|&entry| entry.is_occurence_valid()).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[PasswordEntry]) -> usize {
    input.iter().filter(|&entry| entry.is_exactly_one_at_positions_valid()).count()
}
