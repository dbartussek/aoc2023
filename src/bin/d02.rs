use std::collections::HashMap;
use maplit::hashmap;
use aoc2023::read_input_lines_skip_empty;

fn main() {
    let lines = read_input_lines_skip_empty(2);

    let counts: HashMap<&str, usize> = hashmap!{
        "red" => 12,
        "green" => 13,
        "blue" => 14,
    };

    let mut sum = 0;

    'game:
    for (index, line) in lines.iter().enumerate() {
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
}
