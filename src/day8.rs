use std::collections::VecDeque;

struct Node {
    x: i64,
    y: i64,
    z: i64,
    color: u32,
    links: Vec<usize>,
}

struct Wire {
    len: f64,
    a_idx: usize,
    b_idx: usize,
}

fn bucket_fill(nodes: &mut Vec<Node>, idx: usize) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back(idx);
    let mut circuit_size = 0;
    let color = nodes[idx].color;
    // note that this bucket fill has a special hardcoded case for the idx we started on, to allow us to flow out from it but not return, even if it matches the circuit id at (in fact it always will)
    while let Some(next) = queue.pop_front() {
        if nodes[next].color == color && next != idx {
            continue;
        }
        nodes[next].color = color;
        circuit_size += 1;

        for link in &nodes[next].links {
            let link_node = &nodes[*link];
            if link_node.color != color && *link != idx {
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
            let nums: Vec<_> = l.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            Node {
                x: nums[0],
                y: nums[1],
                z: nums[2],
                color: idx as u32,
                links: vec![],
            }
        })
        .collect();

    let mut connections = vec![];
    for a_idx in 0..nodes.len() {
        for b_idx in 0..a_idx {
            let v1 = &nodes[a_idx];
            let v2 = &nodes[b_idx];
            let len = (((v2.x - v1.x) as f64).powf(2.0)
                + ((v2.y - v1.y) as f64).powf(2.0)
                + ((v2.z - v1.z) as f64).powf(2.0))
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
            for idx in 0..nodes.len() {
                let n = &nodes[idx];
                if n.links.is_empty() {
                    continue;
                }

                let circuit_size = bucket_fill(&mut nodes, idx);

                circuit_sizes.push(circuit_size);
            }

            circuit_sizes.sort();
            circuit_sizes.reverse();

            circuit_sizes.iter().take(3).product::<u64>()
        }

        1 => {
            // different approach. each step, establish one connection. if that connection touches nothing, give the two of them a new cirtcuit id. if one or both ends have links, pick one and bucket fill the other side

            for (_conn_idx, c) in connections.iter().enumerate() {
                nodes[c.a_idx].links.push(c.b_idx);
                nodes[c.b_idx].links.push(c.a_idx);

                bucket_fill(&mut nodes, c.a_idx);
                if nodes.iter().all(|n| n.color == nodes[c.a_idx].color) {
                    // this check can be faster and smarter! but it think this can work to get the answer...?
                    // idea: always pick lower circuit id to bucket out, and keep track of just that id (0). but we passed and this is already quite fast so i'm moving on.
                    // return conn_idx as u64;
                    return (nodes[c.a_idx].x * nodes[c.b_idx].x) as u64; // this is the answer format
                }
            }

            panic!("didn't connect all the circuits!");
        }

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8() {
        assert_eq!(solve(0), 140008);
        assert_eq!(solve(1), 9253260633);
    }
}
