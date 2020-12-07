use std::collections::HashMap;
use regex::Regex;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<&str, HashMap<&str, usize>> {
    let mut rules: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    let contents_regex = Regex::new(r"(\d+) (.*?) bag").unwrap();
    input.lines().for_each(|l| {
        let mut splits = l.split(" bags contain ");
        let bag_color = splits.next().unwrap();
        let contents = splits.next().unwrap();
        let contents: HashMap<&str, usize> = contents_regex.captures_iter(contents)
                                                           .map(|c| (
                                                               c.get(2).unwrap().as_str(),
                                                               c.get(1).unwrap().as_str().parse().unwrap(),
                                                           ))
                                                           .collect();
        rules.insert(bag_color, contents);
    });
    rules
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<&str, HashMap<&str, usize>>) -> usize {
    println!("{:?}", input);
    20
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(part1(&*input_generator(input)), 4);
    }
}
