use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
pub struct BusInfo {
    interval: u64,
    offset: u64,
}

impl BusInfo {
    #[inline]
    pub fn valid_part_one(&self, time: u64) -> bool {
        time % self.interval == 0
    }

    #[inline]
    pub fn valid_part_two(&self, time: u64) -> bool {
        ((time + self.offset) % self.interval) == 0
    }

    pub fn get_new_step(&self, start_time: u64, existing_step: u64) -> (u64, u64) {
        let mut times: Vec<u64> = vec![];
        // Iterate through times, starting at start_time and stepping by existing_step,
        // until you find the next two instances where the bus is valid.  Return the
        // first instance as the new start value and the difference as the new interval.
        let mut current_time = start_time;
        loop {
            if self.valid_part_two(current_time) {
                times.push(current_time);
                if times.len() == 2 {
                    let new_start_time = times[0];
                    let new_step_value = times[1] - times[0];
                    return (new_start_time, new_step_value);
                }
            }
            current_time += existing_step;
        }
    }
}

type InputData = (u64, Vec<BusInfo>);

pub fn generator(input: &str) -> InputData {
    let part_one_start_time = input
        .lines()
        .next()
        .unwrap()
        .parse::<u64>()
        .expect("Couldn't parse start time");
    let bus_list = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, busname)| match busname {
            "x" => None,
            _ => Some(BusInfo {
                offset: i as u64,
                interval: busname.parse::<u64>().expect("Couldn't parse busname"),
            }),
        })
        .sorted_unstable_by(|x, y| x.interval.cmp(&y.interval))
        .rev()
        .collect();
    (part_one_start_time, bus_list)
}

pub fn part_one(data: &InputData) -> u64 {
    let (part_one_start_time, bus_list) = data.clone();
    let mut current_time = part_one_start_time;
    let arrived_bus = loop {
        let arrived_buses: Vec<BusInfo> = bus_list
            .iter()
            .filter(|bus| bus.valid_part_one(current_time))
            .copied()
            .collect();
        if arrived_buses.len() > 0 {
            break arrived_buses[0];
        }
        current_time += 1
    };
    arrived_bus.interval * (current_time - part_one_start_time)
}

pub fn part_two(data: &InputData) -> u64 {
    let (_, bus_list) = data.clone();
    let mut current_step: u64 = 1;
    let mut start_time: u64 = 0;

    for bus in bus_list {
        // I wanted to do a destructuring assignment (i.e. (x, y) = func_that_returns_tuple),
        // but apparently Rust doesn't support that yet
        let new_values = bus.get_new_step(start_time, current_step);
        start_time = new_values.0;
        current_step = new_values.1;
    }
    start_time
}
