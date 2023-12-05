use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;
use range_set::RangeSet;
use std::{
    collections::{BTreeSet, HashSet},
    ops::RangeInclusive,
};

fn main() {
    let lines = read_input_lines_skip_empty(3);

    let symbols = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| !c.is_numeric() && *c != '.')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect_vec();

    let mut has_number = lines
        .iter()
        .map(|_| RangeSet::<[RangeInclusive<usize>; 1]>::new())
        .collect_vec();

    let is_number = |x: isize, y: isize| {
        if x < 0 || y < 0 || x as usize >= lines[0].len() || y as usize >= lines.len() {
            false
        } else {
            (lines[y as usize].as_bytes()[x as usize] as char).is_numeric()
        }
    };

    let mut seen = HashSet::new();
    let mut todo = BTreeSet::new();

    for (x, y) in symbols.iter().copied() {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let x = x + dx;
                let y = y + dy;
                if is_number(x, y) {
                    todo.insert((x, y));
                    seen.insert((x, y));
                }
            }
        }
    }

    while let Some((x, y)) = todo.pop_first() {
        let mut maybe_insert = |x, y| {
            if !seen.contains(&(x, y)) && is_number(x, y) {
                todo.insert((x, y));
                seen.insert((x, y));
            }
        };

        maybe_insert(x - 1, y);
        maybe_insert(x + 1, y);

        has_number[y as usize].insert(x as usize);
    }

    let mut sum = 0;

    for (y, numbers) in has_number.iter().enumerate() {
        for r in numbers.clone().into_smallvec() {
            sum += lines[y][r].parse::<usize>().unwrap();
        }
    }

    println!("{sum}");
}
