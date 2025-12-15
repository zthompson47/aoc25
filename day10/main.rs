fn main() {
    let machines: Vec<Machine> = include_str!("test.txt")
        .lines()
        .map(Machine::from)
        .collect();
    println!("Part 1: {}", part1(&machines));
}

fn part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| {
            let mut i = 0;
            'out: loop {
                i += 1;
                for buttons in choose(i, &machine.buttons) {
                    if machine.is_solution(buttons) {
                        break 'out i;
                    }
                }
            }
        })
        .sum()
}

fn choose<T>(number: usize, list: &[T]) -> Vec<&[T]> {
    let result = Vec::new();
    for i in number..list.len() {

    }
    result
}

#[derive(Debug, Default)]
struct Machine {
    solution: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    #[allow(unused)]
    joltage: Vec<usize>,
}

impl Machine {
    fn is_solution(&self, buttons: &[Vec<usize>]) -> bool {
        let mut solution = vec![false; self.solution.len()];
        for button in buttons {
            for i in button {
                solution[*i] = !solution[*i];
            }
        }
        solution == self.solution
    }
}

impl From<&str> for Machine {
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    fn from(line: &str) -> Self {
        let mut parts = line.split(' ');
        let solution = parts
            .next()
            .unwrap()
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();
        let buttons = parts
            .clone()
            .take_while(|l| l.starts_with('('))
            .map(|l| {
                l.strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|c| c.parse().unwrap())
                    .collect()
            })
            .collect();
        let joltage = parts
            .find(|l| l.starts_with('{'))
            .unwrap()
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        Machine {
            solution,
            buttons,
            joltage,
        }
    }
}
