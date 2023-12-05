use aoc2023::read_input_lines_skip_empty;
use itertools::Itertools;

fn read_number(bytes: &[u8]) -> Option<u32> {
    const MAPPINGS: &[(&[u8], u32)] = &[
        (b"0", 0),
        (b"1", 1),
        (b"2", 2),
        (b"3", 3),
        (b"4", 4),
        (b"5", 5),
        (b"6", 6),
        (b"7", 7),
        (b"8", 8),
        (b"9", 9),
        // Text
        (b"zero", 0),
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
    ];

    for (key, v) in MAPPINGS.iter().copied() {
        if bytes.starts_with(key) {
            return Some(v);
        }
    }

    None
}

fn main() {
    let input = read_input_lines_skip_empty(1);

    let mut total = 0;

    for line in &input {
        let numbers = line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| (c as u32) - ('0' as u32))
            .collect_vec();
        total += numbers.first().copied().unwrap_or_default() * 10
            + numbers.last().copied().unwrap_or_default();
    }

    println!("{}", total);

    let mut total = 0;
    for line in &input {
        let numbers = (0..line.as_bytes().len())
            .filter_map(|start| read_number(&line.as_bytes()[start..]))
            .collect_vec();

        total += numbers.first().copied().unwrap_or_default() * 10
            + numbers.last().copied().unwrap_or_default();
    }

    println!("{}", total);
}
