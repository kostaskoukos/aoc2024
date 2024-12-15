use regex::Regex;
use std::fs;

pub fn run_day3() -> (u32, u32) {
    (part1(), part2())
}

fn part1() -> u32 {
    Regex::new(r"mul\((?<x>[0-9]+),(?<y>[0-9]+)\)")
        .unwrap()
        .captures_iter(fs::read_to_string("./input3.txt").unwrap().as_str())
        .map(|c| c.extract())
        .map(|(_, [x, y])| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .map(|(x, y)| x * y)
        .sum()
}

fn part2() -> u32 {
    let mut valid = true;
    let mut res = 0u32;
    for c in Regex::new(r"mul\((?<x>[0-9]+),(?<y>[0-9]+)\)|(?<yes>do\(\))|(?<no>don't\(\))")
        .unwrap()
        .captures_iter(fs::read_to_string("./input3.txt").unwrap().as_str())
    {
        if c.name("yes").is_some() {
            valid = true;
        } else if c.name("no").is_some() {
            valid = false;
        } else if valid {
            res += c.name("x").unwrap().as_str().parse::<u32>().unwrap()
                * c.name("y").unwrap().as_str().parse::<u32>().unwrap()
        }
    }
    res
}
