use aoc2023::read_input_lines_skip_empty;
use maplit::hashmap;
use std::collections::HashMap;

fn main() {
    let lines = read_input_lines_skip_empty(2);

    let counts: HashMap<&str, usize> = hashmap! {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
    };

    let mut sum = 0;

    'game: for (index, line) in lines.iter().enumerate() {
        let index = index + 1;

        let line = line.split(':').skip(1).next().unwrap_or_default();
        for hand in line.split(';') {
            for cube in hand.split(',').map(str::trim) {
                let mut it = cube.split(' ');
                let count = it.next().unwrap().parse::<usize>().unwrap();
                let color = it.next().unwrap();

                if counts.get(color).copied().unwrap() < count {
                    continue 'game;
                }
            }
        }

        sum += index;
    }

    println!("{sum}");


    let mut sum = 0;

    for line in lines.iter() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let line = line.split(':').skip(1).next().unwrap_or_default();
        for hand in line.split(';') {
            for cube in hand.split(',').map(str::trim) {
                let mut it = cube.split(' ');
                let count = it.next().unwrap().parse::<usize>().unwrap();
                let color = it.next().unwrap();

                let compare = match color {
                    "red" => &mut red,
                    "green" => &mut green,
                    "blue" => &mut blue,
                    _ => unreachable!(),
                };

                *compare = (*compare).max(count);
            }
        }

        sum += red * green * blue;
    }

    println!("{sum}");
}
