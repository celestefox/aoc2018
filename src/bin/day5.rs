use std::collections::{HashMap, VecDeque};

fn engine(chain: VecDeque<char>) -> VecDeque<char> {
    chain.iter().fold(VecDeque::new(), |mut acc, &x| {
        if let Some(&y) = acc.back() {
            if x != y && x.to_uppercase().to_string() == y.to_uppercase().to_string() {
                acc.pop_back();
            } else {
                acc.push_back(x);
            }
        } else {
            acc.push_back(x);
        }
        acc
    })
}

fn imbue(chain: VecDeque<char>) -> HashMap<char, usize> {
    "qwertyuiopasdfghjklzxcvbnm".chars().fold(
        HashMap::new(),
        |mut acc: HashMap<char, usize>, x: char| {
            acc.insert(
                x,
                engine(chain.iter().filter(|&y| x.to_uppercase().to_string() != y.to_uppercase().to_string()).cloned().collect()).len(),
            );
            acc
        },
    )
}

pub fn main() {
    let file = include_str!("../../inputs/day5").trim();
    let processed_chain = engine(file.chars().collect());
    //println!("{:?}", processed_chain);
    println!(
        "Number of components in processed chain: {}",
        processed_chain.len()
    );
    let polymers = imbue(processed_chain);
    let canidate = polymers.iter().min_by_key(|poly| poly.1).unwrap();
    println!("Shortest canidate polymer: {:?}", canidate);
}
