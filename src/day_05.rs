pub struct Day05a;
pub struct Day05b;

impl crate::Solution for Day05a {
    fn solve(&self) -> String {
        format!("{}", solve_a(include_str!("../inputs/day05")))
    }
}

impl crate::Solution for Day05b {
    fn solve(&self) -> String {
        format!("{}", solve_b(include_str!("../inputs/day05")))
    }
}

use std::{collections::HashSet, ops::Range};

use nom::{
    bytes::complete::{tag, take_while1},
    character::{complete::*, is_space},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn solve_a(input: &str) -> u64 {
    let (_, game) = parse_game_a(input).expect("failed to parse game");
    return game.min_location();
}

fn solve_b(input: &str) -> u64 {
    let (_, game) = parse_game_b(input).expect("failed to parse game");
    return game.min_location();
}

#[derive(Debug, PartialEq)]
struct Game {
    seeds: Vec<std::ops::Range<u64>>,
    almanac: Almanac,
}

impl Game {
    fn min_location(&self) -> u64 {
        let mut min = u64::MAX;

        for seed_range in &self.seeds {
            let locations = self.almanac.transform(seed_range.clone());
            let min_location = locations.iter().map(|r| r.start).min().unwrap();
            if min_location < min {
                min = min_location;
            }
        }

        min
    }
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn transform(&self, seed_ids: Range<u64>) -> Vec<Range<u64>> {
        let soil: Vec<Range<u64>> = self.seed_to_soil.transform(seed_ids);

        let mut fertilizer = vec![];
        for s in soil {
            let f = self.soil_to_fertilizer.transform(s);
            fertilizer.extend(f)
        }

        let water = fertilizer
            .into_iter()
            .map(|f| self.fertilizer_to_water.transform(f))
            .fold(Vec::new(), |mut acc, w| {
                acc.extend(w);
                acc
            });
        let light = water
            .into_iter()
            .map(|w| self.water_to_light.transform(w))
            .fold(Vec::new(), |mut acc, l| {
                acc.extend(l);
                acc
            });
        let temperature = light
            .into_iter()
            .map(|l| self.light_to_temperature.transform(l))
            .fold(Vec::new(), |mut acc, t| {
                acc.extend(t);
                acc
            });
        let humidity = temperature
            .into_iter()
            .map(|t| self.temperature_to_humidity.transform(t))
            .fold(Vec::new(), |mut acc, h| {
                acc.extend(h);
                acc
            });
        let location = humidity
            .into_iter()
            .map(|h| self.humidity_to_location.transform(h))
            .fold(Vec::new(), |mut acc, l| {
                acc.extend(l);
                acc
            });

        location
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    name: String,
    ranges: Vec<SeedRange>,
}

impl Map {
    /// Transforms a range of seed ids into a range of locations
    fn transform(&self, range: Range<u64>) -> Vec<Range<u64>> {
        // Break up the range into smaller ranges that are fully contained in the mappings
        let mut sub_ranges = HashSet::new();
        for r in &self.ranges {
            if r.source.end <= range.start || r.source.start >= range.end {
                // no overlap
                continue;
            } else if r.source.start >= range.start && r.source.end <= range.end {
                // fully contained
                sub_ranges.insert(r.source.clone());
            } else if r.source.start >= range.start {
                // starts in range
                let new_range = r.source.start..range.end;
                sub_ranges.insert(new_range);
            } else if r.source.end <= range.end {
                // ends in range
                let new_range = range.start..r.source.end;
                sub_ranges.insert(new_range);
            } else {
                // overlaps
                let new_range = range.start..range.end;
                sub_ranges.insert(new_range);
            }
        }

        // If there are no sub ranges, return the original range
        if sub_ranges.is_empty() {
            return vec![range];
        }

        let mut res = vec![];

        // Transform the sub ranges or leave them if they arent contained in the mappings
        for sub in sub_ranges {
            if let Some(matching_range) = self.ranges.iter().find(|r| r.contains(&sub)) {
                res.push(matching_range.transform(sub));
            } else {
                res.push(sub)
            }
        }

        res
    }
}

#[derive(Debug, PartialEq)]
struct SeedRange {
    source: Range<u64>,
    destination: Range<u64>,
}

impl SeedRange {
    fn transform(&self, range: Range<u64>) -> Range<u64> {
        let start = range.start - self.source.start + self.destination.start;
        let end = range.end - self.source.start + self.destination.start;
        start..end
    }

    fn contains(&self, range: &Range<u64>) -> bool {
        self.source.start <= range.start && self.source.end >= range.end
    }
}

fn parse_game_a(input: &str) -> IResult<&str, Game> {
    let (input, seeds) = preceded(
        multispace0,
        preceded(tag("seeds: "), separated_list1(multispace1, u64)),
    )(input)?;

    let (input, almanac) = delimited(multispace0, parse_almanac, multispace0)(input)?;

    let ranges = seeds
        .iter()
        .map(|seed_id| *seed_id..(*seed_id + 1))
        .collect();

    Ok((
        input,
        Game {
            seeds: ranges,
            almanac,
        },
    ))
}

fn parse_game_b(input: &str) -> IResult<&str, Game> {
    let seed_range = separated_pair(u64, space1, u64);
    let (input, seeds) = preceded(
        multispace0,
        preceded(tag("seeds: "), separated_list1(space1, seed_range)),
    )(input)?;

    let seeds = seeds
        .iter()
        .map(|(start, length)| *start..(*start + *length))
        .collect();

    let (input, almanac) = delimited(multispace0, parse_almanac, multispace0)(input)?;

    Ok((input, Game { seeds, almanac }))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seed_to_soil) = parse_map(input)?;
    let (input, soil_to_fertilizer) = parse_map(input)?;
    let (input, fertilizer_to_water) = parse_map(input)?;
    let (input, water_to_light) = parse_map(input)?;
    let (input, light_to_temperature) = parse_map(input)?;
    let (input, temperature_to_humidity) = parse_map(input)?;
    let (input, humidity_to_location) = parse_map(input)?;

    Ok((
        input,
        Almanac {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, name) = delimited(
        multispace0,
        terminated(take_while1(|c| !is_space(c as u8)), tag(" map:")),
        multispace0,
    )(input)?;

    let (input, ranges) = separated_list1(line_ending, parse_range)(input)?;

    Ok((
        input,
        Map {
            name: name.to_string(),
            ranges,
        },
    ))
}

fn parse_range(input: &str) -> IResult<&str, SeedRange> {
    let (input, (destination_range_start, source_range_start, range_length)) = delimited(
        space0,
        tuple((terminated(u64, space1), terminated(u64, space1), u64)),
        space0,
    )(input)?;

    Ok((
        input,
        SeedRange {
            source: source_range_start..(source_range_start + range_length),
            destination: destination_range_start..(destination_range_start + range_length),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 35);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 46);
    }

    #[test]
    fn range() {
        let range = SeedRange {
            source: 98..100,
            destination: 50..52,
        };

        assert_eq!(range.transform(98..100), 50..52);
    }

    #[test]
    fn parse_range() {
        assert_eq!(
            super::parse_range("50 98 2"),
            Ok((
                "",
                SeedRange {
                    source: 98..100,
                    destination: 50..52,
                }
            ))
        );
    }

    #[test]
    fn parse_map() {
        assert_eq!(
            super::parse_map("seed-to-soil map:\n50 98 2\n52 50 48"),
            Ok((
                "",
                Map {
                    name: "seed-to-soil".to_string(),
                    ranges: vec![
                        SeedRange {
                            source: 98..100,
                            destination: 50..52,
                        },
                        SeedRange {
                            source: 50..98,
                            destination: 52..100,
                        },
                    ],
                }
            ))
        );
    }

    #[test]
    fn parse_example() {
        let game = parse_game_a(EXAMPLE);
        assert!(game.is_ok());
        dbg!(game.unwrap());
    }
}
