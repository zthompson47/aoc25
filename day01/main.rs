fn main() {
    let input = include_str!("input.txt");
    let (part1, part2) = run(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn run(input: &str) -> (u32, u32) {
    let mut position = 50;
    let mut zeroes_landed = 0;
    let mut zeroes_clicked = 0;

    for (direction, delta) in input.lines().map(|line| {
        let (direction, delta) = line.split_at(1);
        let delta = delta.parse::<u32>().unwrap();
        (direction, delta)
    }) {
        match direction {
            "R" => {
                let distance = position + delta;
                position = distance % 100;
                zeroes_clicked += distance / 100;
            }
            "L" => {
                let distance = ((100 - position) % 100) + delta;
                position = (100 - (distance % 100)) % 100;
                zeroes_clicked += distance / 100;
            }
            _ => unreachable!(),
        }

        if position == 0 {
            zeroes_landed += 1;
        }
    }

    (zeroes_landed, zeroes_clicked)
}
