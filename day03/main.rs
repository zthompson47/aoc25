fn main() {
    let input = include_str!("input.txt");
    let batteries: Vec<Vec<u32>> = input.lines().fold(Vec::new(), |mut acc, x| {
        acc.push(x.chars().map(|c| c.to_digit(10).unwrap()).collect());
        acc
    });
    println!("Part 1: {}", part1(batteries));
}

fn part1(batteries: Vec<Vec<u32>>) -> u32 {
    batteries
        .iter()
        .map(|bank| {
            let (tens, mut ones) =
                bank[..bank.len() - 1]
                    .iter()
                    .fold((None, None), |(mut tens, mut ones), x| {
                        if tens.is_none() || x > tens.unwrap() {
                            tens = Some(x);
                            ones = None;
                        } else if ones.is_none() || x > ones.unwrap() {
                            ones = Some(x);
                        }
                        (tens, ones)
                    });
            if ones.is_none() || bank.last().unwrap() > ones.unwrap() {
                ones = bank.last();
            }
            tens.unwrap() * 10 + ones.unwrap()
        })
        .sum()
}
