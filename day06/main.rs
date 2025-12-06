fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines().rev();
    // Get operators from last line of input.
    let mut problems = load_problems(lines.next().unwrap());
    // Load operands.
    lines.for_each(|line| {
        line.split_whitespace()
            .enumerate()
            .for_each(|(i, x)| problems[i].values.push(x.parse().unwrap()))
    });
    let part1: u64 = problems.iter().map(solve).sum();
    println!("Part 1: {part1}");

    let mut lines = input.lines().rev();
    let operators = lines.next().unwrap();
    // Get operators from last line of input.
    let mut problems = load_problems(operators);
    // Transpose remaining input to get operands from columns.
    lines
        .rev()
        // Transpose.
        .fold(vec![vec![]; operators.len()], |mut acc, line| {
            line.chars().enumerate().for_each(|(i, c)| acc[i].push(c));
            acc
        })
        .iter()
        // Sets of operands for each problem are separated by blank lines.
        .fold(0, |mut acc, column| {
            let line = column.iter().collect::<String>();
            let line = line.trim();
            if line.is_empty() {
                acc += 1;
            } else {
                problems[acc].values.push(line.parse().unwrap());
            }
            acc
        });
    let part2: u64 = problems.iter().map(solve).sum();
    println!("Part 2: {part2}");
}

fn load_problems(operators: &str) -> Vec<Problem<'_>> {
    operators
        .split_whitespace()
        .map(|x| Problem {
            values: Vec::new(),
            operation: x,
        })
        .collect()
}

#[derive(Debug)]
struct Problem<'a> {
    values: Vec<u64>,
    operation: &'a str,
}

fn solve(problem: &Problem) -> u64 {
    let mut result = match problem.operation {
        "+" => 0,
        "*" => 1,
        _ => panic!(),
    };
    for x in &problem.values {
        match problem.operation {
            "+" => result += x,
            "*" => result *= x,
            _ => panic!(),
        }
    }
    result
}
