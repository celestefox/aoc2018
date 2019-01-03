use std::collections::HashSet;

fn main() {
    let file = include_str!("../../inputs/day1");
    let mut frequency : i64 = 0;
    for line in file.lines() {
        let line = line.trim();
        frequency += line.parse::<i64>().unwrap();
    }
    println!("Frequency: {}", frequency);
    frequency = 0;
    let mut seen_frequencies = HashSet::new();
    for line in file.lines().cycle() {
        let line = line.trim();
        frequency += line.parse::<i64>().unwrap();
        if seen_frequencies.contains(&frequency) {
            break;
        }
        seen_frequencies.insert(frequency);
    }
    println!("First repeated frequency: {}", frequency);
}
