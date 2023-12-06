use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;
use rangemap::{RangeMap, RangeSet};
use rayon::prelude::*;
use std::ops::Range;

fn main() {
    let inputs = read_input_lines_skip_empty(5);
    let mut inputs = inputs.into_iter();

    let seeds = inputs
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let groups = inputs
        .group_by(|line| line.chars().next().unwrap().is_numeric())
        .into_iter()
        .filter_map(|(b, g)| if b { Some(g) } else { None })
        .map(|g| {
            let mut range_map = RangeMap::new();

            for line in g {
                let (dest, source, length) = line
                    .split(' ')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                range_map.insert(source..(source + length), (dest, source));
            }

            range_map
        })
        .collect_vec();

    let do_mapping = |mut seed| {
        for mapping in groups.iter() {
            if let Some((dest, source)) = mapping.get(&seed).copied() {
                seed = seed - source + dest;
            }
        }
        seed
    };
    let do_map_range = |range: Range<u64>| {
        let mut ranges = RangeSet::new();
        ranges.insert(range);

        for mapping in groups.iter() {
            ranges = ranges
                .iter()
                .cloned()
                .flat_map(|range| {
                    let mut unmapped = RangeSet::new();
                    unmapped.insert(range.clone());

                    let mapped = mapping
                        .overlapping(&range)
                        .map(|(key, (dest, source))| {
                            let map_range = key.start.max(range.start)..key.end.min(range.end);
                            let mapped_range = (map_range.start - *source + *dest)
                                ..(map_range.end - *source + *dest);

                            unmapped.remove(map_range);

                            mapped_range
                        })
                        .collect_vec();

                    mapped.into_iter().chain(unmapped)
                })
                .collect();
        }

        ranges
    };

    let mapped = seeds.iter().copied().map(do_mapping).collect_vec();

    println!("{}", mapped.iter().copied().min().unwrap());

    let minimum = seeds
        .chunks(2)
        .collect_vec()
        .into_par_iter()
        .filter_map(|chunk| {
            let base = chunk[0];
            let length = chunk[1];

            let ranges = do_map_range(base..(base + length));
            ranges.into_iter().map(|r| r.start).min()
        })
        .min()
        .unwrap();
    println!("{minimum}");
}
