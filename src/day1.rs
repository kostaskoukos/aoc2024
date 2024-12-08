use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn run_day1() -> u32 {
    let (mut v1, mut v2) = read_input();
    v1.sort();
    v2.sort();
    v1.iter().zip(v2).map(|(a, b)| a.abs_diff(b)).sum()
}

fn read_input() -> (Vec<u32>, Vec<u32>) {
    let reader = io::BufReader::new(File::open("./input1.txt").unwrap());
    reader
        .lines()
        .map(|line| {
            let vec = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (
                vec.first().unwrap().to_owned(),
                vec.get(1).unwrap().to_owned(),
            )
        })
        .unzip()
}
