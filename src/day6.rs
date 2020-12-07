use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let parsed_input: Vec<HashSet<char>> = input.split("\n\n")
                                                .map(|s| s.chars()
                                                          .filter(|c| c.is_alphabetic())
                                                          .collect()
                                                ).collect();
    parsed_input.iter().fold(0, |f, set| f + set.len())
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let group_sets: Vec<Vec<Vec<char>>> = input.split("\n\n")
                                               .map(|s| s.split("\n")
                                                         .map(|s| s.chars().collect())
                                                         .collect()
                                               ).collect();
    group_sets.iter().fold(0, |sum, group| {
        let candidate_answers = group.get(0).unwrap();
        sum + candidate_answers.iter().filter(|a| group.iter().skip(1).all(|g| g.contains(a))).count()
    })
}
