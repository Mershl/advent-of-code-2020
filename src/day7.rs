use std::collections::HashMap;
use regex::Regex;

type RulesMap<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

pub fn input_generator(input: &str) -> RulesMap {
    let mut rules: RulesMap = HashMap::new();
    let contents_regex = Regex::new(r"(\d+) (.*?) bag").unwrap();
    input.lines().for_each(|l| {
        let mut splits = l.split(" bags contain ");
        let bag_color = splits.next().unwrap();
        let contents = splits.next().unwrap();
        let contents: Vec<(&str, usize)> = contents_regex.captures_iter(contents)
                                                           .map(|c| (
                                                               c.get(2).unwrap().as_str(),
                                                               c.get(1).unwrap().as_str().parse().unwrap(),
                                                           ))
                                                           .collect();
        rules.insert(bag_color, contents);
    });
    rules
}

fn reaches_shiny_gold<'a>(
    cache: &mut HashMap<&'a str, bool>,
    rules_map: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    search: &'a str
) -> bool {
    if !cache.contains_key(search) {
        let reaches = rules_map[search].iter().any(|(next_hop,_)| reaches_shiny_gold(cache, rules_map, next_hop));
        cache.insert(search, reaches);
    }
    cache[search]
}

fn bags_needed(rules_map: &RulesMap, search: &str) -> usize {
    1 + rules_map[search].iter().map(|(needed_bag, count)| count * bags_needed(rules_map, needed_bag)).sum::<usize>()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let rules_map = input_generator(input);
    // use cache to remember branches that are already known to reach target
    let mut cache = HashMap::new();
    cache.insert("shiny gold", true);
    // -1 because shiny gold rule itself doesn't count
    rules_map.keys().filter(|rule| reaches_shiny_gold(&mut cache, &rules_map, rule)).count() - 1
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let rules_map = input_generator(input);
    // -1 because shiny gold bag itself doesn't count
    bags_needed(&rules_map, "shiny gold") - 1
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
        assert_eq!(part1(input), 4);
    }
}
