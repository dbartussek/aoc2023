pub fn read_input(day: u8) -> String {
    std::fs::read_to_string(format!("inputs/d{day:02}.txt")).unwrap()
}

pub fn read_input_lines(day: u8) -> Vec<String> {
    let raw = read_input(day);
    raw.lines().map(|s| s.to_string()).collect()
}

pub fn read_input_lines_skip_empty(day: u8) -> Vec<String> {
    let mut l = read_input_lines(day);
    l.retain(|s| !s.is_empty());
    l
}
