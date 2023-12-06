use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let lines = read_input_lines_skip_empty(4);

    let cards: Vec<(HashSet<u8>, Vec<u8>)> = lines
        .iter()
        .map(|l| {
            let mut sides = l
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .split('|')
                .map(|numbers| {
                    numbers
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<u8>().unwrap())
                });
            (
                sides.next().unwrap().collect(),
                sides.next().unwrap().collect(),
            )
        })
        .collect_vec();

    let mut sum = 0;

    for (winners, candidates) in &cards {
        let num = candidates
            .iter()
            .copied()
            .filter(|c| winners.contains(c))
            .count();
        if num == 0 {
            continue;
        }
        sum += 1 << (num - 1);
    }

    println!("{sum}");

    let mut i_have_cards = cards.iter().map(|_| 1usize).collect_vec();

    for (index, (winners, candidates)) in cards.iter().enumerate() {
        let num = candidates
            .iter()
            .copied()
            .filter(|c| winners.contains(c))
            .count();

        let add = i_have_cards[index];

        for offset in 0..num {
            i_have_cards[index + offset + 1] += add;
        }
    }

    println!("{}", i_have_cards.into_iter().sum::<usize>())
}
