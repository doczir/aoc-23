use std::collections::{HashMap, HashSet};
use std::slice::Iter;

use itertools::Itertools;

advent_of_code::solution!(3);

struct Schematic {
    data: Vec<Vec<char>>,
    columns: usize,
    rows: usize,
}

impl Schematic {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        let columns = data.first().unwrap().len();
        let rows = data.len();
        Schematic {
            data,
            columns,
            rows,
        }
    }

    pub fn is_symbol_at(&self, line_index: usize, column_index: usize) -> bool {
        if line_index >= self.rows || column_index >= self.columns {
            return false;
        }
        let line = &self.data[line_index];
        let ch = line[column_index];

        is_symbol(ch)
    }

    pub fn is_gear_at(&self, line_index: usize, column_index: usize) -> bool {
        if line_index >= self.rows || column_index >= self.columns {
            return false;
        }
        let line = &self.data[line_index];
        let ch = line[column_index];

        is_gear(ch)
    }

    pub fn is_neighbor_symbol(&self, line_index: usize, column_index: usize) -> bool {
        self.is_symbol_at(line_index.saturating_sub(1), column_index)
            || self.is_symbol_at(line_index.saturating_sub(1), column_index.saturating_sub(1))
            || self.is_symbol_at(line_index.saturating_sub(1), column_index.saturating_add(1))
            || self.is_symbol_at(line_index, column_index.saturating_sub(1))
            || self.is_symbol_at(line_index, column_index.saturating_add(1))
            || self.is_symbol_at(line_index.saturating_add(1), column_index)
            || self.is_symbol_at(line_index.saturating_add(1), column_index.saturating_sub(1))
            || self.is_symbol_at(line_index.saturating_add(1), column_index.saturating_add(1))
    }

    pub fn adjacent_gears(&self, line_index: usize, column_index: usize) -> Vec<Gear> {
        let results = vec![
            self.gear_at(line_index.saturating_sub(1), column_index.saturating_sub(1)),
            self.gear_at(line_index.saturating_sub(1), column_index),
            self.gear_at(line_index.saturating_sub(1), column_index.saturating_add(1)),
            self.gear_at(line_index, column_index.saturating_sub(1)),
            self.gear_at(line_index, column_index.saturating_add(1)),
            self.gear_at(line_index.saturating_add(1), column_index.saturating_sub(1)),
            self.gear_at(line_index.saturating_add(1), column_index),
            self.gear_at(line_index.saturating_add(1), column_index.saturating_add(1)),
        ];

        results.into_iter().flatten().collect_vec()
    }

    pub fn gear_at(&self, line_index: usize, column_index: usize) -> Option<Gear> {
        if self.is_gear_at(line_index, column_index) {
            Some(Gear {
                row_index: line_index,
                column_index,
            })
        } else {
            None
        }
    }

    pub fn enumerate(&self) -> SchematicEnumerator {
        let mut line_iter = self.data.iter();
        let row_iter = line_iter.next().map(|x| x.iter());
        SchematicEnumerator {
            line_iter,
            row_iter,
            line_index: 0,
            column_index: 0,
        }
    }
}

fn is_symbol(ch: char) -> bool {
    !ch.is_ascii_digit() && ch != '.'
}

fn is_gear(ch: char) -> bool {
    ch == '*'
}

struct SchematicEnumerator<'a> {
    line_iter: Iter<'a, Vec<char>>,
    row_iter: Option<Iter<'a, char>>,
    line_index: usize,
    column_index: usize,
}

impl Iterator for SchematicEnumerator<'_> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        match self.row_iter {
            None => None,
            Some(ref mut row_iter) => {
                let next_char = row_iter.next();

                match next_char {
                    None => {
                        self.row_iter = self.line_iter.next().map(|x| x.iter());
                        self.line_index += 1;
                        self.column_index = 0;
                        self.next()
                    }
                    Some(_) => {
                        let result = next_char.map(|ch| (self.line_index, self.column_index, *ch));
                        self.column_index += 1;
                        result
                    }
                }
            }
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct Gear {
    row_index: usize,
    column_index: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let schematic = Schematic::new(lines);

    let mut part_numbers: Vec<u32> = Vec::new();
    let mut parsed_number = String::new();
    let mut is_part_number = false;

    for (line_index, char_index, ch) in schematic.enumerate() {
        if ch.is_ascii_digit() {
            if !is_part_number {
                is_part_number = schematic.is_neighbor_symbol(line_index, char_index)
            }
            parsed_number.push(ch);
        } else if !parsed_number.is_empty() {
            if is_part_number {
                part_numbers.push(parsed_number.parse().unwrap());
                is_part_number = false;
            }
            parsed_number.clear();
        }
    }

    Some(part_numbers.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let schematic = Schematic::new(lines);

    let mut found_gears: HashMap<Gear, Vec<u32>> = HashMap::new();
    let mut parsed_number = String::new();
    let mut adjacent_gears = HashSet::new();

    for (line_index, char_index, ch) in schematic.enumerate() {
        if ch.is_ascii_digit() {
            let gears = schematic.adjacent_gears(line_index, char_index);
            if !gears.is_empty() {
                adjacent_gears.extend(gears)
            }
            parsed_number.push(ch);
        } else if !parsed_number.is_empty() {
            if !adjacent_gears.is_empty() {
                let part_number = parsed_number.parse::<u32>().unwrap();
                for gear in &adjacent_gears {
                    found_gears
                        .entry(*gear)
                        .and_modify(|e| e.push(part_number))
                        .or_insert(vec![part_number]);
                }
                adjacent_gears.clear();
            }
            parsed_number.clear();
        }
    }

    let result = found_gears
        .into_iter()
        .filter(|(_, part_numbers)| part_numbers.len() == 2)
        .map(|(_, part_numbers)| part_numbers.iter().product::<u32>())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
