use std::collections::HashMap;

pub fn a(input: &str) -> u64 {
    let devices: HashMap<String, Vec<&str>> = input
        .lines()
        .map(|line| (line[0..3].to_owned(), line[5..].split(' ').collect()))
        .collect();

    fn routes_to_out(devices: &HashMap<String, Vec<&str>>, from: &str) -> u64 {
        devices[from]
            .iter()
            .map(|output| {
                if *output == "out" {
                    1
                } else {
                    routes_to_out(devices, output)
                }
            })
            .sum()
    }

    routes_to_out(&devices, "you")
}

pub fn b(input: &str) -> u64 {
    let mut devices_hashmap: HashMap<String, usize> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (line[0..3].to_owned(), i))
        .collect();
    devices_hashmap.insert("out".to_owned(), devices_hashmap.len());

    let mut devices: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line[5..]
                .split(' ')
                .map(|edge| devices_hashmap[edge])
                .collect()
        })
        .collect();
    devices.push(vec![]);

    let svr_id = devices_hashmap["svr"];
    let dac_id = devices_hashmap["dac"];
    let fft_id = devices_hashmap["fft"];
    let out_id = devices_hashmap["out"];

    #[derive(Debug, Default, Clone, Copy)]
    struct DeviceRoutes {
        none: u64,
        dac: u64,
        fft: u64,
        both: u64,
    }

    let mut routes = vec![DeviceRoutes::default(); devices.len()];
    routes[svr_id].none = 1;

    loop {
        let mut new_routes = vec![DeviceRoutes::default(); devices.len()];
        new_routes[out_id] = routes[out_id];
        let mut updates = 0;
        for (device, route) in routes.iter().enumerate() {
            if device != out_id {
                updates += route.none + route.dac + route.fft + route.both;
            }
            for i in &devices[device] {
                let i = *i;
                if device == dac_id {
                    new_routes[i].dac += route.dac + route.none;
                    new_routes[i].both += route.both + route.fft;
                } else if device == fft_id {
                    new_routes[i].fft += route.fft + route.none;
                    new_routes[i].both += route.both + route.dac;
                } else {
                    new_routes[i].none += route.none;
                    new_routes[i].dac += route.dac;
                    new_routes[i].fft += route.fft;
                    new_routes[i].both += route.both;
                }
            }
        }

        if updates == 0 {
            break;
        }

        routes = new_routes;
    }

    routes[out_id].both
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "11_example_a.txt" => 5);
    }

    #[test]
    fn b_example() {
        test_solution!(b, "11_example_b.txt" => 2);
    }
}
