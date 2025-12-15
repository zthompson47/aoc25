fn main() {
    let machines: Vec<Machine> = include_str!("input.txt")
        .lines()
        .map(Machine::from)
        .collect();
    println!("Part 1: {}", part1(&machines));
}

fn part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| {
            for count in 1..=machine.buttons.len() {
                for sequence in choose(count, &machine.buttons) {
                    if machine.is_solution(sequence) {
                        return count;
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

fn choose<T>(number: usize, list: &[T]) -> Vec<Vec<&T>> {
    assert_ne!(number, 0);
    assert!(number <= list.len());
    if number == 1 {
        list.iter().map(|x| Vec::from([x])).collect()
    } else {
        list[..=list.len() - number]
            .iter()
            .enumerate()
            .flat_map(|(idx, v)| {
                choose(number - 1, &list[idx + 1..])
                    .into_iter()
                    .map(|mut sequence| {
                        sequence.push(v);
                        sequence
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[derive(Debug, Default)]
struct Machine {
    solution: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    #[allow(unused)]
    joltage: Vec<usize>,
}

impl Machine {
    fn is_solution(&self, buttons: Vec<&Vec<usize>>) -> bool {
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
