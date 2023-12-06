use std::collections::BTreeMap;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_opt};
use nom::multi::{many1, many_till};
use nom::{
    character::complete::multispace0, error::ParseError, sequence::delimited, Finish, IResult,
};

advent_of_code::solution!(4);

#[derive(Debug)]
struct ScratchCard {
    id: usize,
    winning_numbers: Vec<u8>,
    have_numbers: Vec<u8>,
}

impl ScratchCard {
    fn from_string(input: &str) -> ScratchCard {
        let (_, sc) = ScratchCard::parse(input).finish().unwrap();
        sc
    }

    fn parse(input: &str) -> IResult<&str, ScratchCard> {
        let number = || map_opt(ws(digit1), |n| n.parse::<u8>().ok());

        let (input, id) = delimited(tag("Card "), number(), char(':'))(input)?;

        let (input, winning_numbers) =
            map(many_till(number(), char('|')), |(numbers, _)| numbers)(input)?;
        let (input, have_numbers) = many1(number())(input)?;

        Ok((
            input,
            ScratchCard {
                id: id as usize,
                winning_numbers,
                have_numbers,
            },
        ))
    }

    fn power(&self) -> u32 {
        let wins = self.wins();

        if wins == 0 {
            return 0;
        }

        2u32.pow((wins - 1) as u32)
    }

    fn wins(&self) -> usize {
        self.have_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(ScratchCard::from_string)
        .map(|sc| sc.power())
        .sum();
    Some(result)
}

struct CardCount {
    count: u32,
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines().map(ScratchCard::from_string).collect_vec();

    let mut state = cards
        .iter()
        .map(|sc| (sc.id, CardCount { count: 1 }))
        .collect::<BTreeMap<_, _>>();

    for sc in &cards {
        let cc = state.get(&sc.id).unwrap();
        if cc.count == 0 {
            continue;
        }

        let wins = sc.wins();
        if wins > 0 {
            let cards_won = cc.count;
            for i in sc.id + 1..=sc.id + wins {
                state.entry(i).and_modify(|cc| cc.count += cards_won);
            }
        }
    }

    let result = state.into_values().map(|cc| cc.count).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
