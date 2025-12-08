use std::collections::{HashSet, VecDeque};

struct Node {
    x: f64,
    y: f64,
    z: f64,
    circuit: u32,
    links: Vec<usize>,
}

struct Wire {
    len: f64,
    a_idx: usize,
    b_idx: usize,
}

fn bucket_fill(nodes: &mut Vec<Node>, idx: usize, circuit_id: u32) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back(idx);
    let mut circuit_size = 0;
    while let Some(next) = queue.pop_front() {
        let node = &mut nodes[next];
        if !node.circuit == circuit_id {
            continue;
        }
        node.circuit = circuit_id;
        circuit_size += 1;

        let links = node.links.clone(); // bad! how do i made rust not fight me here.
        for link in &links {
            let link_node = &nodes[*link];
            if link_node.circuit != circuit_id {
                // TODO target color...?
                queue.push_back(*link);
            }
        }
    }
    circuit_size
}

pub fn solve(part: u32) -> u64 {
    let mut nodes: Vec<_> = std::fs::read_to_string("./src/day8_input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            let nums: Vec<_> = l.split(',').map(|s| s.parse::<f64>().unwrap()).collect();
            Node {
                x: nums[0],
                y: nums[1],
                z: nums[2],
                circuit: idx as u32,
                links: vec![],
            }
        })
        .collect();

    let mut connections = vec![];
    for a_idx in 0..nodes.len() {
        for b_idx in 0..a_idx {
            let v1 = &nodes[a_idx];
            let v2 = &nodes[b_idx];
            let len = ((v2.x - v1.x).powf(2.0) + (v2.y - v1.y).powf(2.0) + (v2.z - v1.z).powf(2.0))
                .sqrt();
            connections.push(Wire { len, a_idx, b_idx });
        }
    }
    connections.sort_by(|a, b| a.len.partial_cmp(&b.len).unwrap());

    match part {
        0 => {
            // brute force, it's only 1000 elements.
            // make a list of all connections with a struct including their distance from each other (use lower idx as first key to make stable)
            // sort it and take the first 1000

            // iterate through the nodes. when you find a node with links, bucket fill all those links, saving them to a working hashset, and tally up the product in an accumulator for the puzzle answer

            // wire things up both ways for bucket fill pass
            connections.iter().take(1000).for_each(|c| {
                nodes[c.a_idx].links.push(c.b_idx);
                nodes[c.b_idx].links.push(c.a_idx);
            });

            let mut circuit_sizes = vec![];
            let mut circuit_id = 0;
            for idx in 0..nodes.len() {
                let n = &nodes[idx];
                if n.links.is_empty() {
                    continue;
                }

                let circuit_size = bucket_fill(&mut nodes, idx, circuit_id);
                circuit_id += 1;

                circuit_sizes.push(circuit_size);
            }

            circuit_sizes.sort();
            circuit_sizes.reverse();

            circuit_sizes.iter().take(3).product::<u64>()
        }

        1 => {
            // different approach. each step, establish one connection. if that connection touches nothing, give the two of them a new cirtcuit id. if one or both ends have links, pick one and bucket fill the other side

            let mut lit = 0;
            for c in connections {
                nodes[c.a_idx].links.push(c.b_idx);
                nodes[c.b_idx].links.push(c.a_idx);
            }

            0
        }

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve(0), 140008);
        // assert_eq!(solve(1), 0);
    }
}
