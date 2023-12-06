use itertools::Itertools;
advent_of_code::solution!(5);

struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Mappings>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let input_linux_line_ending = input.replace('\r', "");
        let blocks = input_linux_line_ending.split("\n\n").collect_vec();
        let seeds = blocks[0]
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect_vec();

        let mappings = blocks
            .iter()
            .dropping(1)
            .map(|block| {
                let first_line_end = block.find('\n').unwrap();
                Mappings::parse(&block[first_line_end + 1..])
            })
            .collect_vec();

        Self { seeds, mappings }
    }
}

struct Mappings {
    mappings: Vec<Mapping>,
}

impl Mappings {
    fn parse(string: &str) -> Self {
        let mappings = string.lines().map(Mapping::parse).collect_vec();

        Self { mappings }
    }

    fn map(&self, n: u64) -> u64 {
        let mapped = self.mappings.iter().find_map(|mapping| mapping.map(n));

        match mapped {
            None => n,
            Some(mapped) => mapped,
        }
    }
}

struct Mapping {
    source_start: u64,
    source_end: u64, // exclusive
    destination_start: u64,
}

impl Mapping {
    fn parse(string: &str) -> Self {
        if let Some((destination_start, source_start, size)) = string
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
        {
            return Self {
                source_start,
                source_end: source_start + size,
                destination_start,
            };
        }

        panic!("Failed to parse mapping")
    }

    fn map(&self, n: u64) -> Option<u64> {
        if self.source_start > n || self.source_end <= n {
            return None;
        }

        let offset = n - self.source_start;
        let mapped_value = self.destination_start + offset;

        Some(mapped_value)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let Almanac { seeds, mappings } = Almanac::parse(input);

    let min_location = seeds
        .iter()
        .map(|seed| {
            let mut result = *seed;
            for mapping in mappings.iter() {
                result = mapping.map(result);
            }
            result
        })
        .min();

    min_location
}

pub fn part_two(input: &str) -> Option<u64> {
    // I could iterate each provided seed range, split it into smaller ranges based on all mappings and check the smallest number in all ranges... but ain't nobody got time for that!
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn mapping_map_in_range() {
        let mapping = Mapping::parse("7 0 5");

        assert_eq!(mapping.map(3), Some(10))
    }

    #[test]
    fn mapping_map_in_out_of_range() {
        let mapping = Mapping::parse("7 0 5");

        assert_eq!(mapping.map(100), None)
    }

    #[test]
    fn mapping_map_bounds() {
        let mapping = Mapping::parse("7 0 5");

        assert_eq!(mapping.map(0), Some(7));
        assert_eq!(mapping.map(4), Some(11));
        assert_eq!(mapping.map(5), None);
    }

    #[test]
    fn mappings_maps_correctly() {
        let mappings = Mappings::parse(indoc! {"\
        15 25 3
        0 10 5
        5 15 10
        100 30 4
        "});

        assert_eq!(mappings.map(0), 0);
        assert_eq!(mappings.map(10), 0);
        assert_eq!(mappings.map(14), 4);
        assert_eq!(mappings.map(15), 5);
        assert_eq!(mappings.map(29), 29);
        assert_eq!(mappings.map(33), 103);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
