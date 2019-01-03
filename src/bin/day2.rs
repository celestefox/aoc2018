use std::collections::HashMap;

fn checksum(lines: &str) -> usize {
    let mut two_repeats = 0;
    let mut three_repeats = 0;
    for line in lines.lines() {
        let mut map: HashMap<char, usize> = HashMap::new();
        for character in line.chars() {
            let entry = map.entry(character).or_insert(0);
            *entry += 1;
        }
        if map.values().filter(|&v| *v == 2).count() > 0 {
            two_repeats += 1;
        }
        if map.values().filter(|&v| *v == 3).count() > 0 {
            three_repeats += 1;
        }
    }
    two_repeats * three_repeats
}

fn find_right_ids(lines: &str) -> (&str, &str) {
    let ids: Vec<&str> = lines.lines().map(|v| v.trim()).collect();
    ids.iter().filter_map(|&id| {
        let other = ids.iter().filter(|&oid| compare_ids(id, oid)).nth(0)?;
        Some((id, *other))
    }).nth(0).unwrap()
}

fn compare_ids(id: &str, other: &str) -> bool {
    id.chars().zip(other.chars()).filter(|(id_char, other_char)| id_char != other_char).count() == 1
}

fn shared_chars((sx, sy): (&str, &str)) -> String {
    sx.chars().zip(sy.chars()).filter_map(|(cx, cy)| if cx == cy { Some(cx) } else { None }).collect()
}

pub fn main() {
    let file = include_str!("../../inputs/day2");
    println!(
        "Simple checksum: {}, shared chars between right ids: {:?}",
        checksum(file),
        shared_chars(find_right_ids(file))
    );
}
