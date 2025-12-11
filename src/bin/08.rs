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

    let mut circuits = HashMap::new();
    for i in 0..junction_boxes.len() {
        circuits.insert(i, vec![i]);
    }

    let mut connections = 0;
    for (_, (jb_a_index, jb_b_index)) in min_distance_pairs {
        if connections >= pair_count {
            break;
        }

        if circuits[&jb_a_index] != circuits[&jb_b_index] {
            let mut circuit_a = circuits.remove(&jb_a_index).unwrap();
            let mut circuit_b = circuits.remove(&jb_b_index).unwrap();

            circuit_a.append(&mut circuit_b);

            for &jb_index in &circuit_a {
                circuits.insert(jb_index, circuit_a.clone());
            }

            
        }   
        connections += 1;

        let unique_circuits:HashSet<Vec<usize>>  = HashSet::from_iter(circuits.values().cloned());
        //println!("circuits: {:?}", unique_circuits);
    }
    let unique_circuits:HashSet<Vec<usize>>  = HashSet::from_iter(circuits.values().cloned());
    let mut sorted_circuits: Vec<_> = unique_circuits.into_iter().collect();
    sorted_circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let top_three: Vec<_> = sorted_circuits.into_iter().take(3).collect();
    //println!("Top 3 largest circuits: {:?}", top_three);
    let result: u64 = top_three.iter().map(|circuit| circuit.len() as u64).product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
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

    let mut circuits = HashMap::new();
    for i in 0..junction_boxes.len() {
        circuits.insert(i, vec![i]);
    }

    for (_, (jb_a_index, jb_b_index)) in min_distance_pairs {

        if circuits[&jb_a_index] != circuits[&jb_b_index] {
            let mut circuit_a = circuits.remove(&jb_a_index).unwrap();
            let mut circuit_b = circuits.remove(&jb_b_index).unwrap();

            circuit_a.append(&mut circuit_b);

            for &jb_index in &circuit_a {
                circuits.insert(jb_index, circuit_a.clone());
            }

            
        }

        let unique_circuits:HashSet<Vec<usize>>  = HashSet::from_iter(circuits.values().cloned());
        if unique_circuits.len() == 1 {
            return Some(junction_boxes[jb_a_index].x * junction_boxes[jb_b_index].x);
        }
        //println!("circuits: {:?}", unique_circuits);
    }
    let unique_circuits:HashSet<Vec<usize>>  = HashSet::from_iter(circuits.values().cloned());
    let mut sorted_circuits: Vec<_> = unique_circuits.into_iter().collect();
    sorted_circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let top_three: Vec<_> = sorted_circuits.into_iter().take(3).collect();
    //println!("Top 3 largest circuits: {:?}", top_three);
    let result: u64 = top_three.iter().map(|circuit| circuit.len() as u64).product();
    Some(result)
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
        assert_eq!(result, Some(25272));
    }
}
