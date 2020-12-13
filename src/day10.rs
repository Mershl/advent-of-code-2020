use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut r: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    r.push(0);
    r.sort_unstable();
    r.push(r.last().unwrap() + 3);
    r
}

#[aoc(day10, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut diffs: HashMap<usize, usize> = HashMap::new();

    input.windows(2).for_each(|w| {
        let diff = w[1] - w[0];
        let entry = diffs.entry(diff).or_insert(0);
        *entry += 1;
    });

    diffs.get(&1).unwrap_or(&0) * diffs.get(&3).unwrap_or(&0)
}

#[aoc(day10, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    cache.insert(0, 1);
    input.iter().skip(1).for_each(|&i| {
        let ways_to_jump =
                  cache.get(&(i - 3)).unwrap_or(&0)
                + cache.get(&(i - 2)).unwrap_or(&0)
                + cache.get(&(i - 1)).unwrap_or(&0);
        cache.insert(i, ways_to_jump);
    });
    cache[input.last().unwrap()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        assert_eq!(part1(&*input_generator(input)), 220);
    }

    #[test]
    fn example2() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        assert_eq!(part2(&*input_generator(input)), 19208);
    }
}
