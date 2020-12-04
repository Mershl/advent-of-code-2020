
pub struct TreeMap {
    width: usize,
    height: usize,
    lines: Vec<Vec<char>>
}

pub struct Pos {
    x: usize,
    y: usize
}

impl TreeMap {
    fn get_pos(&self, x: usize, y: usize) -> &char {
        let x = x % self.width;
        let y = y % self.height;
        self.lines.get(y).unwrap().get(x).unwrap()
    }
}

fn trees_for_slope(treemap: &TreeMap, slope_x: usize, slope_y: usize) -> usize {
    let mut current_pos = Pos { x: 0, y: 0 };
    let mut tree_count = 0;
    while current_pos.y < treemap.height {
        match treemap.get_pos(current_pos.x, current_pos.y) {
            '#' => { tree_count += 1 }
            &_ => {}
        }
        current_pos = Pos { x: current_pos.x + slope_x, y: current_pos.y + slope_y }
    }
    tree_count
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> TreeMap {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    TreeMap {
        width: lines.first().unwrap().len(),
        height: lines.len(),
        lines
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &TreeMap) -> usize {
    trees_for_slope(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &TreeMap) -> usize {
      trees_for_slope(input, 1, 1)
    * trees_for_slope(input, 3, 1)
    * trees_for_slope(input, 5, 1)
    * trees_for_slope(input, 7, 1)
    * trees_for_slope(input, 1, 2)
}
