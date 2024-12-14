use regex::Regex;
use std::fs;

pub fn run_day3() -> u32 {
    Regex::new(r"mul\((?<x>[0-9]+),(?<y>[0-9]+)\)")
        .unwrap()
        .captures_iter(fs::read_to_string("./input3.txt").unwrap().as_str())
        .map(|c| c.extract())
        .map(|(_, [x, y])| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .map(|(x, y)| x * y)
        .sum()
}
