use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn find_invalid_number_for_window_size(list: &[usize], window_size: usize) -> Option<usize> {
    list.windows(window_size + 1).find(|window| {
        window[0..window_size].iter().tuple_combinations().all(|(a, b)| a + b != window[window_size])
    }).map(|w| w[window_size])
}

#[aoc(day9, part1)]
pub fn part1(input: &[usize]) -> usize {
    find_invalid_number_for_window_size(input, 25).unwrap()
}

#[aoc(day9, part2)]
pub fn part2(input: &[usize]) -> usize {
    let target_val = part1(input);
    for window_size in 2..=input.len() {
        if let Some(window) = input.windows(window_size).find(|w| w.iter().sum::<usize>() == target_val) {
            return window.iter().min().unwrap() + window.iter().max().unwrap();
        }
    }
    panic!("No window found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: Vec<usize> = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576].to_vec();
        assert_eq!(find_invalid_number_for_window_size(&*input, 5).unwrap(), 127);
    }
}
