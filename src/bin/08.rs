use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JunctionBox {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut junction_boxes = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let coords: Vec<u64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let junction_box = JunctionBox {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };
        junction_boxes.push(junction_box);
    }

    let pair_count = if junction_boxes.len() < 30 { 10 } else { 1000 };

    let mut min_distance_pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let box_a = &junction_boxes[i];
            let box_b = &junction_boxes[j];

            let dx = box_a.x.abs_diff(box_b.x);
            let dy = box_a.y.abs_diff(box_b.y);
            let dz = box_a.z.abs_diff(box_b.z);

            let sqare_distance = dx * dx + dy * dy + dz * dz;

            min_distance_pairs.push((sqare_distance, (i, j)));
        }
    }
    min_distance_pairs.sort_by_key(|k| k.0);

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut junction_box_circuits_map: HashMap<usize, usize> = HashMap::new();

    let mut connections = 0;
    for (_, (jb_a_index, jb_b_index)) in min_distance_pairs {
        if connections >= pair_count {
            break;
        }

        if junction_box_circuits_map.contains_key(&jb_a_index)
            && junction_box_circuits_map.contains_key(&jb_b_index)
        {
            if junction_box_circuits_map[&jb_a_index] != junction_box_circuits_map[&jb_b_index] {
                let circuit_a_index = junction_box_circuits_map[&jb_a_index];
                let circuit_b_index = junction_box_circuits_map[&jb_b_index];

                let boxes_to_move: Vec<usize> = circuits[circuit_b_index].iter().cloned().collect();
                for jb_index in boxes_to_move {
                    junction_box_circuits_map.insert(jb_index, circuit_a_index);
                    circuits[circuit_a_index].insert(jb_index);
                }
                circuits[circuit_b_index].clear();
                println!(
                    "Merged circuits {} and {}",
                    circuit_a_index, circuit_b_index
                );
            } else {
                println!("Both junction boxes already assigned to circuits, skipping");
                continue;
            }
        } else if junction_box_circuits_map.contains_key(&jb_a_index) {
            let circuit_index = junction_box_circuits_map[&jb_a_index];
            junction_box_circuits_map.insert(jb_b_index, circuit_index);
            circuits[circuit_index].insert(jb_b_index);
            println!(
                "Assigned junction box {:?} to existing circuit {}",
                junction_boxes[jb_b_index], circuit_index
            );
        } else if junction_box_circuits_map.contains_key(&jb_b_index) {
            let circuit_index = junction_box_circuits_map[&jb_b_index];
            junction_box_circuits_map.insert(jb_a_index, circuit_index);
            circuits[circuit_index].insert(jb_a_index);
            println!(
                "Assigned junction box {:?} to existing circuit {}",
                junction_boxes[jb_a_index], circuit_index
            );
        } else {
            let circuit_index = circuits.len();
            junction_box_circuits_map.insert(jb_a_index, circuit_index);
            junction_box_circuits_map.insert(jb_b_index, circuit_index);
            circuits.push(HashSet::new());
            circuits[circuit_index].insert(jb_a_index);
            circuits[circuit_index].insert(jb_b_index);
            println!(
                "Created new circuit {} with junction boxes {:?} and {:?}",
                circuit_index, junction_boxes[jb_a_index], junction_boxes[jb_b_index]
            );
        };

        connections += 1;
        println!("Connection {}, circuits : {:?}", connections, circuits);
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
