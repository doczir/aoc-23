use std::cmp::max;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn power(&self) -> u32 {
        let mut max_pulls = HashMap::new();

        for reveals_summed_by_color in self.sets.iter().map(|set| set.reveals_summed_by_colors()) {
            for reveal in reveals_summed_by_color {
                max_pulls
                    .entry(reveal.color)
                    .and_modify(|e| *e = max(*e, reveal.count))
                    .or_insert(reveal.count);
            }
        }

        max_pulls
            .into_iter()
            .map(|(_, count)| count)
            .reduce(|a, b| a * b)
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct Set {
    cube_reveals: Vec<Reveal>,
}

impl Set {
    fn reveals_summed_by_colors(&self) -> Vec<Reveal> {
        let mut result = HashMap::new();

        for reveal in &self.cube_reveals {
            result
                .entry(reveal.color.clone())
                .and_modify(|e| *e += reveal.count)
                .or_insert(reveal.count);
        }

        result
            .into_iter()
            .map(|(color, count)| Reveal { color, count })
            .collect_vec()
    }
}

#[derive(Debug)]
struct Reveal {
    count: u32,
    color: String,
}

fn parse_line(line: &str) -> Game {
    let mut line = line.strip_prefix("Game ").unwrap();
    let colon_idx = line.find(':').unwrap();

    let id: u32 = line[..colon_idx].parse().unwrap();

    line = &line[colon_idx + 1..];

    let set_definitions: Vec<_> = line.split(';').collect();

    let mut sets = Vec::new();
    for set_definition in set_definitions {
        let reveal_definitions: Vec<_> = set_definition.split(',').map(|x| x.trim()).collect();

        let mut cube_reveals = Vec::new();
        for reveal_definition in reveal_definitions {
            let space_idx = reveal_definition.find(' ').unwrap();

            let count: u32 = reveal_definition[..space_idx].parse().unwrap();
            let color = reveal_definition[space_idx + 1..].to_string();

            cube_reveals.push(Reveal { count, color })
        }

        sets.push(Set { cube_reveals });
    }

    Game { id, sets }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<_> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .collect();

    let available_cubes: [(_, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    let result = games
        .iter()
        .filter(|game| {
            !game.sets.iter().any(|set| {
                for Reveal { color, count } in &set.reveals_summed_by_colors() {
                    let &(_, available) = available_cubes.iter().find(|ac| ac.0 == *color).unwrap();

                    if *count > available {
                        return true;
                    }
                }
                false
            })
        })
        .map(|game| game.id)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<_> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .collect();

    let result = games.iter().map(|game| game.power()).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
