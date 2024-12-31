use std::fs;

struct Entry {
    target: u64,
    nums: Vec<u64>,
}

impl Entry {
    fn can_be_calculated1(&self, curr: u64, n: usize) -> bool {
        if n == self.nums.len() {
            return self.target == curr;
        }
        self.can_be_calculated1(curr + self.nums[n], n + 1)
            || self.can_be_calculated1(curr * self.nums[n], n + 1)
    }
    fn can_be_calculated2(&self, curr: u64, n: usize) -> bool {
        if n == self.nums.len() {
            return self.target == curr;
        }
        self.can_be_calculated2(curr + self.nums[n], n + 1)
            || self.can_be_calculated2(curr * self.nums[n], n + 1)
            || self.can_be_calculated2(
                curr * 10u64.pow(self.nums[n].ilog10() + 1) + self.nums[n],
                n + 1,
            )
    }
}

pub fn run_day7() -> (u64, u64) {
    let input = fs::read_to_string("./input7.txt")
        .unwrap()
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(a, b)| Entry {
            target: a.parse::<u64>().unwrap(),
            nums: b
                .split(" ")
                .map(|num| num.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        })
        .collect::<Vec<Entry>>();
    (part1(&input), part2(&input))
}

fn part1(input: &[Entry]) -> u64 {
    input
        .iter()
        .filter(|e| e.can_be_calculated1(e.nums[0], 1))
        .map(|e| e.target)
        .sum()
}

fn part2(input: &[Entry]) -> u64 {
    input
        .iter()
        .filter(|e| e.can_be_calculated2(e.nums[0], 1))
        .map(|e| e.target)
        .sum()
}
