use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let grid = read_grid(input);
    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid));
}

type Grid = HashMap<(isize, isize), (usize, bool)>;

fn read_grid(input: &str) -> Grid {
    input.lines().enumerate().fold(
        HashMap::<(isize, isize), (usize, bool)>::new(),
        |mut acc, (row, line)| {
            let row = row as isize;
            for (column, c) in line.chars().enumerate() {
                let column = column as isize;
                if c == '@' {
                    acc.entry((row, column))
                        .and_modify(|(_, is_paper)| *is_paper = true)
                        .or_insert((0, true));
                    for coord in adjacent(row, column) {
                        acc.entry(coord)
                            .and_modify(|(adjacent, _)| *adjacent += 1)
                            .or_insert((1, false));
                    }
                }
            }
            acc
        },
    )
}

fn part1(mut grid: Grid) -> usize {
    remove_paper(&mut grid).unwrap()
}

fn part2(mut grid: Grid) -> usize {
    let mut total_removed = 0;
    while let Some(removed) = remove_paper(&mut grid) {
        total_removed += removed;
    }
    total_removed
}

fn adjacent(row: isize, column: isize) -> [(isize, isize); 8] {
    [
        (row - 1, column - 1),
        (row, column - 1),
        (row + 1, column - 1),
        (row - 1, column),
        (row + 1, column),
        (row - 1, column + 1),
        (row, column + 1),
        (row + 1, column + 1),
    ]
}

fn remove_paper(grid: &mut Grid) -> Option<usize> {
    let paper: Vec<(isize, isize)> = grid
        .iter()
        .filter(|((_, _), (adjacent, paper))| *paper && *adjacent < 4)
        .map(|(coord, _)| *coord)
        .collect();
    let len = paper.len();
    for coord in paper {
        for x in adjacent(coord.0, coord.1) {
            grid.get_mut(&x).unwrap().0 -= 1;
        }
        grid.get_mut(&coord).unwrap().1 = false;
    }
    if len > 0 { Some(len) } else { None }
}
