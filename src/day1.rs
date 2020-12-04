
const TARGET_VALUE: u32 = 2020;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut values: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    values.sort_unstable();
    values.retain(|&x| x <= TARGET_VALUE);
    values
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    for (idx1, &v1) in input.iter().enumerate() {
        for v2 in input.iter().skip(idx1 + 1) {
            if v1 + v2 == TARGET_VALUE {
                return v1 * v2;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    for (idx1, &v1) in input.iter().enumerate() {
        for (idx2, &v2) in input.iter().enumerate().skip(idx1 + 1) {
            for v3 in input.iter().skip(idx2 + 1) {
                if v1 + v2 + v3 == TARGET_VALUE {
                    return v1 * v2 * v3;
                }
            }
        }
    }
    unreachable!()
}
