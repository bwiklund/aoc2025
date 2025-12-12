use std::collections::HashMap;

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl Device {
    pub fn parse(s: &str) -> Result<Device, &str> {
        let (name, outputs) = s.split_once(": ").unwrap();
        Ok(Device {
            name: name.to_string(),
            outputs: outputs.split_ascii_whitespace().map(Into::into).collect(),
        })
    }
}

pub fn solve(part: u32) -> u64 {
    let devices: HashMap<String, Device> = std::fs::read_to_string("./src/day11_input.txt")
        .unwrap()
        .lines()
        .filter_map(|s| Device::parse(s).ok())
        .map(|d| (d.name.clone(), d))
        .collect();

    match part {
        0 => {
            // let seen = HashSet::<String>::new();
            fn search(devices: &HashMap<String, Device>, id: &str) -> u64 {
                match id {
                    "out" => 1,
                    _ => devices[id]
                        .outputs
                        .iter()
                        .map(|oid| search(devices, oid))
                        .sum::<u64>(),
                }
            }
            search(&devices, "you")
        }

        1 => {
            // different enough that i don't wanna share the traversal code.
            // the addition of dac/fft is a wrinkle but the main problem here is that srv->out goes through way more nodes than you->out.
            // i think this is a dp problem, where we do math on the number of ways we've reached a node as we get there. so we need a bfs that has a list of (node, tally) we're cascading out to (possibly multiple copies of each node, merged), and keeps a running tally of how many paths got us there
            // ok nevermind this is how the fft/dac requirement fucks us lol
            // 
            // fn search(
            //     devices: &HashMap<String, Device>,
            //     id: &str,
            //     seen_dac: bool, // no need to get too fancy
            //     seen_fft: bool, // no need to get too fancy
            // ) -> u64 {
            //     match id {
            //         "out" => (seen_dac && seen_fft) as u64,
            //         _ => devices[id]
            //             .outputs
            //             .iter()
            //             .map(|oid| {
            //                 search(
            //                     devices,
            //                     oid,
            //                     seen_dac || id == "dac",
            //                     seen_fft || id == "fft",
            //                 )
            //             })
            //             .sum::<u64>(),
            //     }
            // }
            // search(&devices, "svr", false, false)

            0
        }

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11() {
        assert_eq!(solve(0), 523);
        assert_eq!(solve(1), 0);
    }
}
