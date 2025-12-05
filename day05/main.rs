use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("input.txt");
    let (mut ranges, ingredients) = read_input(input);
    let part1 = ingredients
        .iter()
        .filter(|x| {
            for r in &ranges {
                if r.contains(x) {
                    return true;
                }
            }
            false
        })
        .count();
    println!("Part 1: {part1}");

    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut ranges = ranges.into_iter();
    let mut canonical_ranges = vec![ranges.next().unwrap()];
    for range in ranges {
        if range.start() >= canonical_ranges.last().unwrap().start()
            && range.start() <= canonical_ranges.last().unwrap().end()
        {
            *canonical_ranges.last_mut().unwrap() =
                *(range.start().min(canonical_ranges.last().unwrap().start()))
                    ..=*(range.end().max(canonical_ranges.last().unwrap().end()));
        } else {
            canonical_ranges.push(range);
        }
    }
    let part2: usize = canonical_ranges.into_iter().map(|x| x.count()).sum();
    println!("Part 2: {part2}");
}

fn read_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (mut ranges, mut ingredients) = (Vec::new(), Vec::new());
    let mut lines = input.lines();
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (l, r) = line.split_once('-').unwrap();
        ranges.push(l.parse().unwrap()..=r.parse().unwrap());
    }
    for line in lines {
        ingredients.push(line.parse().unwrap());
    }
    (ranges, ingredients)
}
