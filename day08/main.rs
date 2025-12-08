use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
    circuit: Option<usize>,
}
type JunctionBoxRef = Rc<RefCell<JunctionBox>>;

fn main() {
    //let (connections, input) = (10, include_str!("test.txt"));
    let (connections, input) = (1000, include_str!("input.txt"));

    let boxes: Vec<JunctionBoxRef> = input
        .lines()
        .map(|x| {
            let mut coords = x.split(',');
            Rc::new(RefCell::new(JunctionBox {
                x: coords.next().unwrap().parse().unwrap(),
                y: coords.next().unwrap().parse().unwrap(),
                z: coords.next().unwrap().parse().unwrap(),
                circuit: None,
            }))
        })
        .collect();

    // Calculate distances between all junction box pairs and sort by distance.
    let mut box_pairs: Vec<(JunctionBoxRef, JunctionBoxRef, f32)> = (0..boxes.len() - 1)
        .flat_map(|i| {
            (i + 1..boxes.len())
                .map(|j| {
                    let a = boxes[i].borrow();
                    let b = boxes[j].borrow();
                    let distance = ((a.x as f32 - b.x as f32).powi(2)
                        + (a.y as f32 - b.y as f32).powi(2)
                        + (a.z as f32 - b.z as f32).powi(2))
                    .powf(0.5);
                    (boxes[i].clone(), boxes[j].clone(), distance)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    box_pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Create circuits out of pairs with shortest distance.
    for (circuit_id, pair) in box_pairs.iter().take(connections).enumerate() {
        let (mut box_a, mut box_b) = (pair.0.borrow_mut(), pair.1.borrow_mut());
        if let Some(circuit_a) = box_a.circuit
            && let Some(circuit_b) = box_b.circuit
        {
            // Merge two circuits, without mangling references when scanning original list.
            drop(box_a);
            drop(box_b);
            for junction_box in boxes.iter() {
                let circuit = junction_box.borrow().circuit;
                if let Some(circuit) = circuit
                    && circuit == circuit_a
                {
                    junction_box.borrow_mut().circuit = Some(circuit_b);
                }
            }
        } else if let Some(existing_circuit) = box_a.circuit {
            // Join circuit.
            box_b.circuit = Some(existing_circuit);
        } else if let Some(existing_circuit) = box_b.circuit {
            // Join circuit.
            box_a.circuit = Some(existing_circuit);
        } else {
            // New circuit.
            box_a.circuit = Some(circuit_id);
            box_b.circuit = Some(circuit_id);
        }
    }

    // Calculate size of each circuit and find largest.
    let mut count = HashMap::<usize, usize>::new();
    for b in boxes.iter() {
        if let Some(c) = b.borrow().circuit {
            count.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    let mut part1: Vec<usize> = count.values().copied().collect();
    part1.sort_by(|a, b| b.cmp(a));
    let part1: usize = part1.iter().take(3).product();

    println!("Part 1: {part1}");
}
