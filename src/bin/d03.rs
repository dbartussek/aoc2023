use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;
use range_set::RangeSet;
use std::{
    collections::{BTreeSet, HashSet},
    iter::once,
    ops::RangeInclusive,
};

fn flood_numbers<IN, R>(
    num_lines: usize,
    is_number: IN,
    root_set: R,
) -> Vec<RangeSet<[RangeInclusive<usize>; 1]>>
where
    IN: Fn(isize, isize) -> bool,
    R: IntoIterator<Item = (isize, isize)>,
{
    let mut has_number = (0..num_lines)
        .map(|_| RangeSet::<[RangeInclusive<usize>; 1]>::new())
        .collect_vec();

    let mut seen = HashSet::new();
    let mut todo = BTreeSet::new();

    for (x, y) in root_set {
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

    has_number
}

fn main() {
    let lines = read_input_lines_skip_empty(3);

    let symbols = lines.iter().enumerate().flat_map(|(y, line)| {
        line.char_indices()
            .filter(|(_, c)| !c.is_numeric() && *c != '.')
            .map(move |(x, _)| (x as isize, y as isize))
    });

    let is_number = |x: isize, y: isize| {
        if x < 0 || y < 0 || x as usize >= lines[0].len() || y as usize >= lines.len() {
            false
        } else {
            (lines[y as usize].as_bytes()[x as usize] as char).is_numeric()
        }
    };

    let has_number = flood_numbers(lines.len(), is_number, symbols);

    let mut sum = 0;

    for (y, numbers) in has_number.into_iter().enumerate() {
        for r in numbers.into_smallvec() {
            sum += lines[y][r.clone()].parse::<usize>().unwrap();
        }
    }

    println!("{sum}");

    let symbols = lines.iter().enumerate().flat_map(|(y, line)| {
        line.char_indices()
            .filter(|(_, c)| *c == '*')
            .map(move |(x, _)| (x as isize, y as isize))
    });

    let mut sum = 0;

    for s in symbols {
        let mut ratio = 1;
        let mut count = 0;

        for (y, numbers) in flood_numbers(lines.len(), is_number, once(s))
            .into_iter()
            .enumerate()
        {
            for r in numbers.into_smallvec() {
                ratio *= lines[y][r.clone()].parse::<usize>().unwrap();
                count += 1;
            }
        }

        if count == 2 {
            sum += ratio;
        }
    }

    println!("{sum}");
}
