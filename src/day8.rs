use std::collections::{HashSet, VecDeque};

struct Node {
    x: f64,
    y: f64,
    z: f64,
    links: Vec<usize>,
}

struct Wire {
    len: f64,
    a_idx: usize,
    b_idx: usize,
}

pub fn solve(part: u32) -> u64 {
    let mut nodes: Vec<_> = std::fs::read_to_string("./src/day8_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let nums: Vec<_> = l.split(',').map(|s| s.parse::<f64>().unwrap()).collect();
            Node {
                x: nums[0],
                y: nums[1],
                z: nums[2],
                links: vec![],
            }
        })
        .collect();

    match part {
        0 => {
            // brute force, it's only 1000 elements.
            // make a list of all connections with a struct including their distance from each other (use lower idx as first key to make stable)
            // sort it and take the first 1000
            {
                let mut connections = vec![];
                for a_idx in 0..nodes.len() {
                    for b_idx in 0..a_idx {
                        let v1 = &nodes[a_idx];
                        let v2 = &nodes[b_idx];
                        let len = ((v2.x - v1.x).powf(2.0)
                            + (v2.y - v1.y).powf(2.0)
                            + (v2.z - v1.z).powf(2.0))
                        .sqrt();
                        connections.push(Wire { len, a_idx, b_idx });
                    }
                }

                connections.sort_by(|a, b| a.len.partial_cmp(&b.len).unwrap());
                // dbg!(&connections.iter().take(10).collect::<Vec<_>>());

                // todo make a graph of the 1000 shortest connections and bucket fill and remove and repeat until you've counted what we need?

                // wire things up both ways for bucket fill pass
                connections.iter().take(1000).for_each(|c| {
                    nodes[c.a_idx].links.push(c.b_idx);
                    nodes[c.b_idx].links.push(c.a_idx);
                });
            }

            // iterate through the nodes. when you find a node with links, bucket fill all those links, saving them to a working hashset, and tally up the product in an accumulator for the puzzle answer

            let mut seen = HashSet::new();
            let mut circuit_sizes = vec![];
            for idx in 0..nodes.len() {
                let n = &nodes[idx];
                if n.links.is_empty() || seen.contains(&idx) {
                    continue;
                }

                let mut queue = VecDeque::new();
                queue.push_back(idx);
                let mut circuit_size = 0;
                while let Some(next) = queue.pop_front() {
                    if seen.contains(&next) {
                        // this is possible!
                        continue;
                    }
                    seen.insert(next);
                    circuit_size += 1;

                    for link in &nodes[next].links {
                        if !seen.contains(link) {
                            queue.push_back(*link);
                        }
                    }
                }
                circuit_sizes.push(circuit_size);
            }

            circuit_sizes.sort();
            circuit_sizes.reverse();

            dbg!(&circuit_sizes);
            dbg!(&circuit_sizes.iter().sum::<u64>());

            circuit_sizes.iter().take(3).product()
        }

        1 => 0,
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
