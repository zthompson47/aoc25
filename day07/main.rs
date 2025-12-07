use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines();
    // Map each beam to number of paths that got to that beam.
    let mut beams = HashMap::from([(lines.next().unwrap().find("S").unwrap(), 1)]);
    let mut splits = 0;
    for line in lines {
        let mut new_beams = HashMap::new();
        for (beam, paths) in beams {
            // Sum paths of beams that merge.
            let mut merge = |i: usize| {
                new_beams
                    .entry(i)
                    .and_modify(|x| *x += paths)
                    .or_insert(paths);
            };
            match line.chars().collect::<Vec<char>>()[beam] {
                '.' => merge(beam),
                '^' => {
                    splits += 1;
                    merge(beam - 1);
                    merge(beam + 1);
                }
                _ => panic!(),
            }
        }
        beams = new_beams;
    }
    let total_paths: usize = beams.values().sum();
    println!("Part 1: {}", splits);
    println!("Part 2: {}", total_paths);
}
