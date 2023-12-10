use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;

fn main() {
    let input = read_input_lines_skip_empty(9);
    let numbers = input
        .into_iter()
        .map(|l| {
            l.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let chain = numbers
        .iter()
        .map(|series| {
            let mut series = series.clone();
            let mut pyramid = vec![series.clone()];

            while series.iter().any(|v| *v != 0) {
                series = series
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect_vec();
                pyramid.push(series.clone());
            }
            pyramid
        })
        .collect_vec();

    let total = chain
        .iter()
        .map(|series_and_derivatives| {
            series_and_derivatives
                .iter()
                .rev()
                .map(|s| *s.last().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{total}");

    let total = chain
        .iter()
        .map(|series_and_derivatives| {
            series_and_derivatives
                .iter()
                .rev()
                .fold(0, |a, b| b.first().unwrap() - a)
        })
        .sum::<i32>();
    println!("{total}");
}
