fn main() {
    let input = include_str!("input.txt");
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .map(|x| {
            x.split_once(',')
                .map(|(r, c)| (r.parse().unwrap(), c.parse().unwrap()))
                .unwrap()
        })
        .collect();
    let mut max_area = 0;
    for (a, b) in tiles.pairs() {
        let area = ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs();
        max_area = max_area.max(area);
    }
    println!("Part 1: {max_area}");
}

trait Pairs<T> {
    fn pairs(&self) -> Vec<(T, T)>
    where
        T: Clone + Copy;
}

impl<T> Pairs<T> for Vec<T> {
    fn pairs(&self) -> Vec<(T, T)>
    where
        T: Clone + Copy,
    {
        (0..self.len() - 1)
            .flat_map(|x| {
                (x + 1..self.len())
                    .map(move |y| (self[x], self[y]))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
