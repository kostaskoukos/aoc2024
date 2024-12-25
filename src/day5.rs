use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{self},
};

pub fn run_day5() -> (usize, usize) {
    let input = fs::read_to_string("input5.txt").unwrap();
    let (order, list) = input.split_once("\n\n").unwrap();

    let order: Vec<(u32, u32)> = order
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect();

    let mut updates: Vec<Vec<u32>> = list
        .lines()
        .map(|l| l.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    (part1(&updates, &order), part2(&mut updates, &order))
}

fn part1(updates: &[Vec<u32>], order: &[(u32, u32)]) -> usize {
    updates
        .iter()
        .filter(|update| {
            let update_map: HashMap<u32, usize> =
                update.iter().enumerate().map(|(i, &v)| (v, i)).collect();
            !order.iter().any(|(a, b)| {
                update_map.contains_key(a)
                    && update_map.contains_key(b)
                    && update_map.get(a).unwrap() > update_map.get(b).unwrap()
            })
        })
        .map(|update| update[update.len() / 2] as usize)
        .sum()
}

fn part2(updates: &mut [Vec<u32>], order: &[(u32, u32)]) -> usize {
    updates
        .iter_mut()
        .filter(|update| {
            let update_map: HashMap<u32, usize> =
                update.iter().enumerate().map(|(i, &v)| (v, i)).collect();
            !order.iter().any(|(a, b)| {
                update_map.contains_key(a)
                    && update_map.contains_key(b)
                    && update_map.get(a).unwrap() > update_map.get(b).unwrap()
            })
        })
        .map(|update| {
            update.sort_unstable_by(|&a, &b| {
                if order.contains(&(a, b)) {
                    Ordering::Less
                } else if order.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update[update.len() / 2] as usize
        })
        .sum()
}
