use std::{collections::HashSet, fs};

pub fn run_day6() -> (usize, usize) {
    let map: Vec<Vec<char>> = fs::read_to_string("input6.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut start = (0u32, 0u32);
    for i in 0..map.len() - 1 {
        for j in 0..map[0].len() - 1 {
            if map[i][j] == '^' {
                start = (i as u32, j as u32);
            }
        }
    }
    (part1(&map, start), 0)
}

fn part1(map: &[Vec<char>], start: (u32, u32)) -> usize {
    let mut guard = start;

    let mut dir: (i8, i8) = (-1, 0);
    let mut res = 0;
    let mut visited = HashSet::new();

    loop {
        res += if visited.insert(guard) { 1 } else { 0 };
        let mut new_y = guard.0 as i32 + dir.0 as i32;
        let mut new_x = guard.1 as i32 + dir.1 as i32;

        if new_y < 0 || new_y >= map.len() as i32 || new_x < 0 || new_x >= map[0].len() as i32 {
            break res;
        }
        if map[new_y as usize][new_x as usize] == '#' {
            dir = (dir.1, -dir.0);
            new_y = guard.0 as i32 + dir.0 as i32;
            new_x = guard.1 as i32 + dir.1 as i32;
        }
        guard = (new_y as u32, new_x as u32);
    }
}
