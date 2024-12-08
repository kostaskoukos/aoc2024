use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn run_day2() -> usize {
    io::BufReader::new(File::open("./input2.txt").unwrap())
        .lines()
        .filter(|line| {
            let vec: Vec<u32> = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            is_valid(&vec)
        })
        .count()
}

fn is_valid(vec: &[u32]) -> bool {
    if !vec.windows(2).all(|w| w[0] <= w[1]) && !vec.windows(2).all(|w| w[0] >= w[1]) {
        return false;
    }
    vec.windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .all(|v| (1..=3).contains(&v))
}
