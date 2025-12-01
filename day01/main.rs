fn main() {
    let input = include_str!("input.txt");
    let (part1, part2) = run(input);
    println!("Part 1: {}", part1);
    println!("Part 1: {}", part2);
}

fn run(input: &str) -> (u32, u32) {
    let mut pos = 50;
    let mut zeroes_landed = 0;
    let mut zeroes_clicked = 0;

    for (dir, val) in input.lines().map(|line| {
        let (dir, val) = line.split_at(1);
        let val = val.parse::<u32>().unwrap();
        (dir, val)
    }) {
        match dir {
            "R" => {
                let tot = pos + val;
                pos = tot % 100;
                zeroes_clicked += tot / 100;
            }
            "L" => {
                let tot = ((100 - pos) % 100) + val;
                pos = (100 - (tot % 100)) % 100;
                zeroes_clicked += tot / 100;
            }
            _ => unreachable!(),
        }

        if pos == 0 {
            zeroes_landed += 1;
        }
    }

    (zeroes_landed, zeroes_clicked)
}
