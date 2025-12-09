use std::{collections::HashMap, mem::take};

pub fn a<const N: usize>(input: &str) -> u64 {
    let coords = input
        .lines()
        .map(|line| {
            let mut values = line.split(',').map(|v| v.parse::<i64>().unwrap());
            (
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let len = coords.len();

    let mut pairs: Vec<Pair> = Vec::with_capacity(len.pow(2) / 2);
    for a in 0..(len - 1) {
        for b in (a + 1)..len {
            pairs.push(Pair {
                dist_sq: (coords[a].0 - coords[b].0).pow(2)
                    + (coords[a].1 - coords[b].1).pow(2)
                    + (coords[a].2 - coords[b].2).pow(2),
                a: a as u16,
                b: b as u16,
            });
        }
    }
    pairs.select_nth_unstable_by_key(N, |p| p.dist_sq);
    let shortest_pairs = &pairs[..N];

    let mut circuits: Vec<Vec<u16>> = vec![];
    let mut node_to_circuit_idx: HashMap<u16, u16> = HashMap::new();
    for pair in shortest_pairs {
        let a_circuit_idx = node_to_circuit_idx.get(&pair.a);
        let b_circuit_idx = node_to_circuit_idx.get(&pair.b);
        if let Some(aci) = a_circuit_idx
            && let Some(bci) = b_circuit_idx
        {
            let aci = *aci;
            let bc = take(&mut circuits[*bci as usize]);
            circuits[aci as usize].extend_from_slice(&bc);
            for node in bc {
                *node_to_circuit_idx.get_mut(&node).unwrap() = aci;
            }
        } else if let Some(i) = a_circuit_idx {
            circuits[*i as usize].push(pair.b);
            node_to_circuit_idx.insert(pair.b, *i);
        } else if let Some(i) = b_circuit_idx {
            circuits[*i as usize].push(pair.a);
            node_to_circuit_idx.insert(pair.a, *i);
        } else {
            let i = circuits.len();
            circuits.push(vec![pair.a, pair.b]);
            node_to_circuit_idx.insert(pair.a, i as u16);
            node_to_circuit_idx.insert(pair.b, i as u16);
        }
    }
    let clen = circuits.len();
    circuits.select_nth_unstable_by_key(clen - 3, |c| c.len());

    circuits[(clen - 3)..]
        .iter()
        .map(|c| c.len() as u64)
        .product()
}

pub fn b(input: &str) -> u64 {
    let coords = input
        .lines()
        .map(|line| {
            let mut values = line.split(',').map(|v| v.parse::<i64>().unwrap());
            (
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let len = coords.len();

    let mut pairs: Vec<Pair> = Vec::with_capacity(len.pow(2) / 2);
    for a in 0..(len - 1) {
        for b in (a + 1)..len {
            pairs.push(Pair {
                dist_sq: (coords[a].0 - coords[b].0).pow(2)
                    + (coords[a].1 - coords[b].1).pow(2)
                    + (coords[a].2 - coords[b].2).pow(2),
                a: a as u16,
                b: b as u16,
            });
        }
    }

    pairs.sort_unstable_by_key(|p| p.dist_sq);

    let mut circuits: Vec<Vec<u16>> = vec![];
    let mut node_to_circuit_idx: HashMap<u16, u16> = HashMap::new();
    for pair in pairs {
        let a_circuit_idx = node_to_circuit_idx.get(&pair.a);
        let b_circuit_idx = node_to_circuit_idx.get(&pair.b);
        if let Some(aci) = a_circuit_idx
            && let Some(bci) = b_circuit_idx
        {
            let aci = *aci;
            let bc = take(&mut circuits[*bci as usize]);
            circuits[aci as usize].extend_from_slice(&bc);
            for node in bc {
                *node_to_circuit_idx.get_mut(&node).unwrap() = aci;
            }
        } else if let Some(i) = a_circuit_idx {
            circuits[*i as usize].push(pair.b);
            node_to_circuit_idx.insert(pair.b, *i);
        } else if let Some(i) = b_circuit_idx {
            circuits[*i as usize].push(pair.a);
            node_to_circuit_idx.insert(pair.a, *i);
        } else {
            let i = circuits.len();
            circuits.push(vec![pair.a, pair.b]);
            node_to_circuit_idx.insert(pair.a, i as u16);
            node_to_circuit_idx.insert(pair.b, i as u16);
        }

        if circuits.iter().any(|c| c.len() == coords.len()) {
            return (coords[pair.a as usize].0 * coords[pair.b as usize].0) as u64;
        }
    }

    todo!()
}

#[derive(Debug, Clone)]
struct Pair {
    dist_sq: i64,
    a: u16,
    b: u16,
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a::<10>, "8_example.txt" => 40);
    }

    #[test]
    fn b_example() {
        test_solution!(b, "8_example.txt" => 25272);
    }
}
