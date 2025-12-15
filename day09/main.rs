use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .map(|x| {
            x.split_once(',')
                .map(|(c, r)| (c.parse().unwrap(), r.parse().unwrap()))
                .unwrap()
        })
        .collect();
    println!("Part 1: {}", max_area(tiles.pairs()));
    // 4781546175

    // Find red tile with no other red tile above or to the left.
    // Tiles directly above and to left of this red tile must be other color, outside of loop.
    let min_column = tiles.iter().map(|x| x.0).min().unwrap();
    let min_row = tiles
        .iter()
        .filter(|x| x.0 == min_column)
        .map(|x| x.1)
        .min()
        .unwrap();
    let upper_left = (min_column, min_row);
    let upper_left_idx = tiles
        .iter()
        .enumerate()
        .find(|coord| *coord.1 == upper_left)
        .unwrap()
        .0;

    // Determine if tile list is clockwise or counterclockwise.
    let next_tile = tiles.get(upper_left_idx + 1).or(tiles.first()).unwrap();
    let rotation = match (
        upper_left.0.cmp(&next_tile.0),
        upper_left.1.cmp(&next_tile.1),
    ) {
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Rotation::Counterclockwise,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Rotation::Clockwise,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Rotation::Clockwise,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Rotation::Counterclockwise,
        _ => unreachable!(),
    };

    let mut outside: HashSet<(i64, i64)> = HashSet::new();
    let mut border: Vec<(i64, i64)> = Vec::new();
    let mut red: HashMap<(i64, i64), Red> = HashMap::new();
    let mut prior_tile = upper_left;

    for tile in tiles
        .iter()
        .cycle()
        .skip_while(|coord| *coord != next_tile)
        .take(tiles.len())
    {
        let direction = relative_direction(prior_tile, *tile);

        red.entry(prior_tile)
            .and_modify(|t| t.direction_out = Some(direction))
            .or_insert(Red {
                direction_in: None,
                direction_out: Some(direction),
            });
        red.entry(*tile)
            .and_modify(|t| t.direction_in = Some(direction.opposite()))
            .or_insert(Red {
                direction_in: Some(direction.opposite()),
                direction_out: None,
            });

        border.push(*tile);

        match direction {
            Direction::N => {
                assert_eq!(prior_tile.0, tile.0);
                let c = prior_tile.0;
                for r in tile.1 + 1..prior_tile.1 {
                    border.push((c, r));
                    outside.insert(match rotation {
                        Rotation::Clockwise => (c - 1, r),
                        Rotation::Counterclockwise => (c + 1, r),
                    });
                }
            }
            Direction::S => {
                assert_eq!(prior_tile.0, tile.0);
                let c = prior_tile.0;
                for r in prior_tile.1 + 1..tile.1 {
                    border.push((c, r));
                    outside.insert(match rotation {
                        Rotation::Clockwise => (c + 1, r),
                        Rotation::Counterclockwise => (c - 1, r),
                    });
                }
            }
            Direction::E => {
                assert_eq!(prior_tile.1, tile.1);
                let r = prior_tile.1;
                for c in prior_tile.0 + 1..tile.0 {
                    border.push((c, r));
                    outside.insert(match rotation {
                        Rotation::Clockwise => (c, r - 1),
                        Rotation::Counterclockwise => (c, r + 1),
                    });
                }
            }
            Direction::W => {
                assert_eq!(prior_tile.1, tile.1);
                let r = prior_tile.1;
                for c in tile.0 + 1..prior_tile.0 {
                    border.push((c, r));
                    outside.insert(match rotation {
                        Rotation::Clockwise => (c, r + 1),
                        Rotation::Counterclockwise => (c, r - 1),
                    });
                }
            }
        }

        prior_tile = *tile;
    }

    for (tile, directions) in red {
        [
            directions.direction_in.unwrap().opposite(),
            directions.direction_out.unwrap(),
        ]
        .iter()
        .map(|d| match d {
            Direction::N => match rotation {
                Rotation::Clockwise => Direction::W,
                Rotation::Counterclockwise => Direction::E,
            },
            Direction::S => match rotation {
                Rotation::Clockwise => Direction::E,
                Rotation::Counterclockwise => Direction::W,
            },
            Direction::E => match rotation {
                Rotation::Clockwise => Direction::N,
                Rotation::Counterclockwise => Direction::S,
            },
            Direction::W => match rotation {
                Rotation::Clockwise => Direction::S,
                Rotation::Counterclockwise => Direction::N,
            },
        })
        .filter(|d| {
            *d != directions.direction_in.unwrap() && *d != directions.direction_out.unwrap()
        })
        .map(|d| match d {
            Direction::N => (tile.0, tile.1 - 1),
            Direction::S => (tile.0, tile.1 + 1),
            Direction::E => (tile.0 + 1, tile.1),
            Direction::W => (tile.0 - 1, tile.1),
        })
        .for_each(|o| {
            outside.insert(o);
        });
    }

    for b in &border {
        outside.remove(b);
    }

    let mut pairs = HashSet::new();
    for p in tiles.pairs() {
        pairs.insert(p);
    }

    for o in &outside {
        let mut invalid = Vec::new();
        for p in &pairs {
            let (a, b) = p;
            if o.0 <= a.0.max(b.0)
                && o.0 >= a.0.min(b.0)
                && o.1 <= a.1.max(b.1)
                && o.1 >= a.1.min(b.1)
            {
                invalid.push(*p);
            }
        }
        for o in invalid {
            pairs.remove(&o);
        }
    }

    println!("Part 2: {}", max_area(pairs));
    // WRONG 1565694816
    // RIGHT 1573359081
}

fn max_area<'a>(pairs: impl IntoIterator<Item = (&'a (i64, i64), &'a (i64, i64))>) -> i64 {
    pairs
        .into_iter()
        .map(|(a, b)| ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Red {
    direction_in: Option<Direction>,
    direction_out: Option<Direction>,
}

fn relative_direction(from: (i64, i64), to: (i64, i64)) -> Direction {
    match (from.0.cmp(&to.0), from.1.cmp(&to.1)) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Direction::E,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Direction::S,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Direction::N,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Direction::W,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

trait Pairs<T> {
    fn pairs(&self) -> Vec<(&T, &T)>;
}

impl<T> Pairs<T> for Vec<T> {
    fn pairs(&self) -> Vec<(&T, &T)> {
        (0..self.len() - 1)
            .flat_map(|x| {
                (x + 1..self.len())
                    .map(|y| (&self[x], &self[y]))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
