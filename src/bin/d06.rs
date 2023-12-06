use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = read_input_lines_skip_empty(6);
    let mut numbers = input.into_iter().map(|l| {
        l.split(':')
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec()
    });

    let times = numbers.next().unwrap();
    let distances = numbers.next().unwrap();

    let results: Vec<_> = times
        .par_iter()
        .copied()
        .zip(distances.par_iter().copied())
        .map(|(time, distance)| {
            (0..time)
                .into_par_iter()
                .map(|charge_time| (time - charge_time) * charge_time)
                .filter(|this_distance| *this_distance > distance)
                .count()
        })
        .collect();

    println!(
        "{results:?}: {}",
        results.iter().copied().fold(1, |a, b| a * b)
    );

    let input = read_input_lines_skip_empty(6);
    let mut numbers = input.into_iter().map(|l| {
        l.split(':')
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .replace(' ', "")
            .parse::<usize>()
            .unwrap()
    });
    let time = numbers.next().unwrap();
    let distance = numbers.next().unwrap();

    let results = (0..time)
        .into_par_iter()
        .map(|charge_time| (time - charge_time) * charge_time)
        .filter(|this_distance| *this_distance > distance)
        .count();

    println!("{results}",);
}
