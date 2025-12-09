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
    for (a, b) in pairs(&tiles) {
        let area = ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs();
        max_area = max_area.max(area);
    }
    println!("Part 1: {max_area}");
}

fn pairs<T>(v: &[T]) -> Vec<(T, T)>
where
    T: Clone + Copy,
{
    (0..v.len() - 1)
        .flat_map(|x| {
            (x + 1..v.len())
                .map(move |y| (v[x], v[y]))
                .collect::<Vec<_>>()
        })
        .collect()
}
