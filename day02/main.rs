use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let input = include_str!("input.txt");
    let ranges: Vec<RangeInclusive<usize>> = input
        .trim()
        .split(',')
        .map(|x| x.split_once('-').unwrap())
        .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
        .collect();

    println!("Part 1: {}", part1(ranges.clone()));
    println!("Part 2: {}", part2(ranges));
}

fn part1(ranges: Vec<RangeInclusive<usize>>) -> usize {
    let mut sum = 0;
    for r in ranges {
        for i in r {
            let i_str = i.to_string();
            if i_str.len().is_multiple_of(2) {
                let (left, right) = i_str.split_at(i_str.len() / 2);
                if left == right {
                    sum += i;
                }
            }
        }
    }
    sum
}

fn part2(ranges: Vec<RangeInclusive<usize>>) -> usize {
    let mut invalid: HashSet<usize> = HashSet::new();
    for r in ranges {
        //println!("========== {r:?} ===========");
        for i in r {
            //println!("---- {i} ----");
            let i_str = i.to_string();
            if i_str.len() < 2 {
                continue;
            }
            'each_len: for len in 1..=i_str.len() / 2 {
                //println!("len: {len}");
                if i_str.len().is_multiple_of(len) {
                    let first = &i_str[0..len];
                    //println!(" .. is multiple len: {len}, first: {first}");
                    for j in 1..i_str.len() / len {
                        if first != &i_str[j * len..j * len + len] {
                            continue 'each_len;
                        }
                    }
                    //println!("!!!!!!!!!!!!!!!!!!!!! got one: {i}");
                    invalid.insert(i);
                }
            }
        }
    }
    invalid.iter().sum()
}
