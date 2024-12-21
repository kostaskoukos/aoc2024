use std::fs;

pub fn run_day4() -> (usize, usize) {
    let map = fs::read_to_string("./input4.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                'X' => p1 += find_xmas(&map, (i, j)),
                'A' => p2 += find_mas(&map, (i, j)),
                _ => continue,
            }
        }
    }

    (p1, p2)
}

fn find_mas(map: &[Vec<char>], (i, j): (usize, usize)) -> usize {
    if i == 0 || i == map.len() - 1 || j == 0 || j == map[0].len() - 1 {
        return 0;
    }
    if (map[i - 1][j - 1] == 'M' && map[i + 1][j + 1] == 'S'
        || map[i - 1][j - 1] == 'S' && map[i + 1][j + 1] == 'M')
        && (map[i - 1][j + 1] == 'M' && map[i + 1][j - 1] == 'S'
            || map[i - 1][j + 1] == 'S' && map[i + 1][j - 1] == 'M')
    {
        1
    } else {
        0
    }
}

fn find_xmas(map: &[Vec<char>], (i, j): (usize, usize)) -> usize {
    let dp = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let pattern = ['M', 'A', 'S'];
    dp.iter()
        .filter(|&&(dx, dy)| {
            pattern.iter().enumerate().all(|(k, &c)| {
                let new_i = i as isize + dy * (k as isize + 1);
                let new_j = j as isize + dx * (k as isize + 1);
                new_i >= 0
                    && new_i < map.len() as isize
                    && new_j >= 0
                    && new_j < map[0].len() as isize
                    && map[new_i as usize][new_j as usize] == c
            })
        })
        .count()
}
