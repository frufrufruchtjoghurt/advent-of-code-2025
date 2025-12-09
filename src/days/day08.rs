use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    error::Error,
    hash::Hash,
};

pub fn solve_part1(input: &str) -> usize {
    solve_part1_with_size(input, 1000)
}

pub fn solve_part2(input: &str) -> usize {
    let mut electrical_system = load_electrical_system(input);

    for i in 0..electrical_system.junction_boxes.len() {
        for j in (i + 1)..electrical_system.junction_boxes.len() {
            let from = JunctionBoxId(i);
            let to = JunctionBoxId(j);
            electrical_system.create_connection(from, to);
        }
    }

    let last_connection = electrical_system
        .process_until_single_circuit()
        .expect("Failed to connect all junction boxes into a single circuit");

    electrical_system.junction_boxes[last_connection.from.0].x as usize
        * electrical_system.junction_boxes[last_connection.to.0].x as usize
}

fn solve_part1_with_size(input: &str, k: usize) -> usize {
    let mut electrical_system = load_electrical_system(input);

    for i in 0..electrical_system.junction_boxes.len() {
        for j in (i + 1)..electrical_system.junction_boxes.len() {
            let from = JunctionBoxId(i);
            let to = JunctionBoxId(j);
            electrical_system.create_connection(from, to);
        }
    }

    electrical_system.process_k_connections(k);

    let mut circuit_sizes: Vec<usize> = electrical_system
        .circuits
        .iter()
        .map(|c| c.junction_boxes.len())
        .collect();

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.iter().take(3).product()
}

fn load_electrical_system(input: &str) -> ElectricalSystem {
    let num_boxes = input.lines().count();
    let mut electrical_system = ElectricalSystem::new(num_boxes);

    input.lines().for_each(|line| {
        let coords: Vec<f64> = line
            .split(',')
            .map(|num| num.trim().parse::<f64>().unwrap())
            .collect();
        electrical_system.add_junction_box((coords[0], coords[1], coords[2]), num_boxes);
    });

    electrical_system
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBoxId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CircuitId(usize);

#[derive(Debug)]
struct JunctionBox {
    x: f64,
    y: f64,
    z: f64,
    circuit: CircuitId,
}

impl JunctionBox {
    fn distance_to_squared(&self, other: &JunctionBox) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }

    fn distance_to(&self, other: &JunctionBox) -> f64 {
        self.distance_to_squared(other).sqrt()
    }
}

#[derive(Debug, Clone)]
struct Connection {
    from: JunctionBoxId,
    to: JunctionBoxId,
    distance: f64,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
            && (self.from == other.from && self.to == other.to
                || self.from == other.to && self.to == other.from)
    }
}

impl Eq for Connection {}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse order for min-heap behavior
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug)]
struct Circuit {
    junction_boxes: HashSet<JunctionBoxId>,
}

#[derive(Debug)]
struct ElectricalSystem {
    junction_boxes: Vec<JunctionBox>,
    connections: BinaryHeap<Connection>,
    circuits: Vec<Circuit>,
}

impl ElectricalSystem {
    fn new(size: usize) -> Self {
        Self {
            junction_boxes: Vec::with_capacity(size),
            connections: BinaryHeap::with_capacity(size * size),
            circuits: Vec::with_capacity(size),
        }
    }

    fn add_junction_box(&mut self, (x, y, z): (f64, f64, f64), circuit_capacity: usize) {
        let circuit_id = CircuitId(self.circuits.len());
        self.junction_boxes.push(JunctionBox {
            x,
            y,
            z,
            circuit: circuit_id,
        });
        self.circuits.push(Circuit {
            junction_boxes: HashSet::with_capacity(circuit_capacity),
        });
        self.circuits[circuit_id.0]
            .junction_boxes
            .insert(JunctionBoxId(self.junction_boxes.len() - 1));
    }

    fn create_connection(&mut self, from: JunctionBoxId, to: JunctionBoxId) {
        let distance = self.junction_boxes[from.0].distance_to(&self.junction_boxes[to.0]);
        let connection = Connection { from, to, distance };
        self.connections.push(connection);
    }

    fn get_circuit_id_for_junction_box(&self, junction_box_id: JunctionBoxId) -> CircuitId {
        self.junction_boxes[junction_box_id.0].circuit
    }

    fn merge_circuits(&mut self, circuit_a: CircuitId, circuit_b: CircuitId) {
        if circuit_a == circuit_b {
            return;
        }

        let (smaller_id, larger_id) = if self.circuits[circuit_a.0].junction_boxes.len()
            < self.circuits[circuit_b.0].junction_boxes.len()
        {
            (circuit_a, circuit_b)
        } else {
            (circuit_b, circuit_a)
        };

        let smaller_junction_boxes = self.circuits[smaller_id.0].junction_boxes.clone();

        let larger_circuit = &mut self.circuits[larger_id.0];
        for &junction_box_id in &smaller_junction_boxes {
            larger_circuit.junction_boxes.insert(junction_box_id);
            self.junction_boxes[junction_box_id.0].circuit = larger_id;
        }

        self.circuits[smaller_id.0].junction_boxes.clear();
    }

    fn add_connection_to_circuit(&mut self, connection: Connection) {
        let from_circuit_id = self.get_circuit_id_for_junction_box(connection.from);
        let to_circuit_id = self.get_circuit_id_for_junction_box(connection.to);

        if from_circuit_id != to_circuit_id {
            self.merge_circuits(from_circuit_id, to_circuit_id);
        }
    }

    fn process_k_connections(&mut self, k: usize) {
        for _ in 0..k {
            if let Some(connection) = self.connections.pop() {
                self.add_connection_to_circuit(connection);
            } else {
                break;
            }
        }
    }

    fn process_until_single_circuit(&mut self) -> Result<Connection, Box<dyn Error>> {
        loop {
            if let Some(connection) = self.connections.pop() {
                self.add_connection_to_circuit(connection.clone());

                let non_empty_circuits: Vec<&Circuit> = self
                    .circuits
                    .iter()
                    .filter(|c| !c.junction_boxes.is_empty())
                    .collect();

                if non_empty_circuits.len() == 1 {
                    return Ok(connection);
                }
            } else {
                return Err("Not enough connections to form a single circuit".into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    #[test]
    fn test_solve_part1() {
        let input = read_to_string("input/day08/example.txt").unwrap();
        let result = solve_part1_with_size(&input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_solve_part2() {
        let input = read_to_string("input/day08/example.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 25272);
    }
}
