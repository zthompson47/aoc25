fn main() {
    let input = include_str!("input.txt");
    let batteries: Vec<Vec<u64>> = input.lines().fold(Vec::new(), |mut acc, x| {
        acc.push(x.chars().map(|c| c.to_digit(10).unwrap() as u64).collect());
        acc
    });
    println!("Part 1: {}", part1(batteries.clone()));
    println!("Part 2: {}", part2(batteries));
}

fn part1(batteries: Vec<Vec<u64>>) -> u64 {
    batteries
        .iter()
        .map(|bank| {
            let (tens, mut ones) =
                bank[..bank.len() - 1]
                    .iter()
                    .fold((0, 0), |(mut tens, mut ones), &x| {
                        if x > tens {
                            tens = x;
                            ones = 0;
                        } else if x > ones {
                            ones = x;
                        }
                        (tens, ones)
                    });
            if *bank.last().unwrap() > ones {
                ones = *bank.last().unwrap();
            }
            tens * 10 + ones
        })
        .sum()
}

fn part2(batteries: Vec<Vec<u64>>) -> u64 {
    batteries
        .iter()
        .map(|bank| {
            let mut joltage = [0; 12];
            for i in 0..bank.len() {
                // Try replacing furthest left digit in joltage as the remaining bank allows.
                for j in 12usize.saturating_sub(bank.len() - i)..12 {
                    if bank[i] > joltage[j] {
                        joltage[j] = bank[i];
                        // Reset digits to right of new digit.  They must come from after new
                        // digit in bank to keep order.
                        if j < joltage.len() {
                            joltage[j + 1..].iter_mut().for_each(|x| *x = 0);
                        }
                        break;
                    }
                }
            }
            joltage
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    x * 10u64.pow(11 - i as u32)
                })
                .sum::<u64>()
        })
        .sum()
}
