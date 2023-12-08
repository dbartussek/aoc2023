use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

fn find_time<'lt, E>(
    directions: &[usize],
    map: &'lt HashMap<String, [String; 2]>,
    start: &'lt str,
    end: E,
    cycle_offset: usize,
) -> (usize, &'lt str)
where
    E: Fn(&str) -> bool,
{
    let mut location = start;
    for (step, direction) in directions
        .iter()
        .copied()
        .cycle()
        .skip(cycle_offset)
        .enumerate()
    {
        location = map[location][direction].as_str();

        if end(location) {
            return (step + 1, location);
        }
    }

    unreachable!()
}

fn main() {
    let input = read_input_lines_skip_empty(8);
    let mut input = input.iter();

    let directions: Vec<usize> = input
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unimplemented!(),
        })
        .collect_vec();

    let regex = Regex::new("([A-Z0-9]+)[^A-Z0-9]*([A-Z0-9]+)[^A-Z0-9]*([A-Z0-9]+)").unwrap();
    let map: HashMap<String, [String; 2]> = input
        .map(|line| {
            let mat = regex.captures(line).unwrap();

            (mat[1].to_string(), [mat[2].to_string(), mat[3].to_string()])
        })
        .collect();

    println!(
        "{}",
        find_time(&directions, &map, "AAA", |s| s == "ZZZ", 0).0
    );

    let locations = map.keys().filter(|s| s.ends_with('A')).collect_vec();

    let cycle_start_points = locations
        .iter()
        .copied()
        .map(|start| {
            let (time, end_point) = find_time(&directions, &map, start, |s| s.ends_with('Z'), 0);
            let cycle_offset = time % directions.len();
            (end_point, cycle_offset)
        })
        .collect_vec();

    assert!(cycle_start_points
        .iter()
        .map(|(_, offset)| *offset)
        .all_equal());

    let cycle_step = cycle_start_points
        .iter()
        .copied()
        .map(|(start, offset)| {
            let (time, end) = find_time(&directions, &map, start, |s| s.ends_with('Z'), offset);

            (start, end, time % directions.len(), time)
        })
        .collect_vec();

    assert!(cycle_step
        .iter()
        .copied()
        .all(|(start, end, cycle_offset, _)| start == end && cycle_offset == 0));

    println!(
        "{}",
        cycle_step.iter().map(|(_, _, _, time)| *time).fold(1, lcm)
    );
}
