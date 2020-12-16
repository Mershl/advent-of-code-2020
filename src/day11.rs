use crate::day11::Position::{Empty, Occupied, Floor};
use std::num::Wrapping;
use std::cmp::max;

#[derive(PartialEq, Clone, Debug)]
enum Position {
    Empty,
    Occupied,
    Floor,
}

struct Plane {
    seat_map: Vec<Vec<Position>>,
    w: usize,
    h: usize,
}

impl Plane {
    pub fn get(&self, x: usize, y: usize) -> &Position {
        if x >= self.w || y >= self.h {
            return &Floor;
        }
        &self.seat_map[y][x]
    }

    fn count_surrounded_by(&self, x: usize, y: usize, surrounded_by: &Position) -> usize {
        let candidates = [self.get((Wrapping(x) - Wrapping(1)).0, (Wrapping(y) - Wrapping(1)).0),
                          self.get(x, (Wrapping(y) - Wrapping(1)).0),
                          self.get(x + 1, (Wrapping(y) - Wrapping(1)).0),
                          self.get(x + 1, y),
                          self.get(x + 1, y + 1),
                          self.get(x, y + 1),
                          self.get((Wrapping(x) - Wrapping(1)).0, y + 1),
                          self.get((Wrapping(x) - Wrapping(1)).0, y)];
        candidates.iter().filter(|&cand| cand == &surrounded_by).count()
    }

    fn first_seen_from_to_direction(&self, x_from: i64, y_from: i64, dir_x: i64, dir_y: i64) -> &Position {
        for multiplier in 1..=max(self.w, self.h) as i64 {
            let next_x = x_from + (dir_x * multiplier);
            let next_y = y_from + (dir_y * multiplier);
            if next_x < 0 || next_x >= self.w as i64 || next_y < 0 || next_y >= self.h as i64 {
                break;
            }
            match self.get(next_x as usize, next_y as usize) {
                &Occupied => { return &Occupied; }
                &Empty => { return &Empty; }
                _ => { }
            }
        }
        &Floor
    }

    fn count_visible_by(&self, x: usize, y: usize, first_visible: &Position) -> usize {
        let candidates = [self.first_seen_from_to_direction(x as i64, y as i64, -1, -1),
                          self.first_seen_from_to_direction(x as i64, y as i64, 0, -1),
                          self.first_seen_from_to_direction(x as i64, y as i64, 1, -1),
                          self.first_seen_from_to_direction(x as i64, y as i64, 1, 0),
                          self.first_seen_from_to_direction(x as i64, y as i64, 1, 1),
                          self.first_seen_from_to_direction(x as i64, y as i64, 0, 1),
                          self.first_seen_from_to_direction(x as i64, y as i64, -1, 1),
                          self.first_seen_from_to_direction(x as i64, y as i64, -1, 0)];
        candidates.iter().filter(|&cand| cand == &first_visible).count()
    }

    pub fn next_seat_map_part1(&self) -> Vec<Vec<Position>> {
        let mut next_seat_map = self.seat_map.clone();
        for y in 0..self.h {
            for x in 0..self.w {
                match self.get(x, y) {
                    Empty => {
                        if self.count_surrounded_by(x, y, &Occupied) == 0 {
                            next_seat_map[y][x] = Occupied;
                        }
                    }
                    Occupied => {
                        if self.count_surrounded_by(x, y, &Occupied) >= 4 {
                            next_seat_map[y][x] = Empty;
                        }
                    }
                    Floor => {}
                }
            }
        }
        next_seat_map
    }

    pub fn next_seat_map_part2(&self) -> Vec<Vec<Position>> {
        let mut next_seat_map = self.seat_map.clone();
        for y in 0..self.h {
            for x in 0..self.w {
                match self.get(x, y) {
                    Empty => {
                        if self.count_visible_by(x, y, &Occupied) == 0 {
                            next_seat_map[y][x] = Occupied;
                        }
                    }
                    Occupied => {
                        if self.count_visible_by(x, y, &Occupied) >= 5 {
                            next_seat_map[y][x] = Empty;
                        }
                    }
                    Floor => {}
                }
            }
        }
        next_seat_map
    }
}

fn input_generator(input: &str) -> Plane {
    let seat_map: Vec<Vec<Position>> = input.lines().map(|l| l.chars().map(|c| match c {
        'L' => { Empty }
        '#' => { Occupied }
        '.' => { Floor }
        _ => panic!("Unsupported sign!")
    }).collect()).collect();
    Plane {
        w: seat_map[0].len(),
        h: seat_map.len(),
        seat_map,
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut plane = input_generator(input);

    loop {
        let next_seat_map = plane.next_seat_map_part1();
        if next_seat_map == plane.seat_map {
            break;
        }
        plane.seat_map = next_seat_map;
    }

    plane.seat_map.iter().flat_map(|l| l.iter().filter(|&seat| seat == &Occupied)).count()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut plane = input_generator(input);

    loop {
        let next_seat_map = plane.next_seat_map_part2();
        if next_seat_map == plane.seat_map {
            break;
        }
        plane.seat_map = next_seat_map;
    }

    plane.seat_map.iter().flat_map(|l| l.iter().filter(|&seat| seat == &Occupied)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
        assert_eq!(part1(input), 37);
    }

    #[test]
    fn example2() {
        let input = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
        assert_eq!(part2(input), 26);
    }
}
