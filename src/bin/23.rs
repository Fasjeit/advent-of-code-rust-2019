use advent_of_code::intcode::Machine;
use std::collections::VecDeque;

advent_of_code::solution!(23);

const NAT_ADDRESS: usize = 255;
const NAT_INDEX: usize = 50;

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

pub fn solve(input: &str, part_two: bool) -> Option<u64> {
    let firmware: Vec<i64> = input
        .trim_end()
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let ext_memory_size = 4096;
    let mut ext_memory: Vec<i64> = Vec::with_capacity(ext_memory_size);
    for i in 0..ext_memory_size {
        if i < firmware.len() {
            ext_memory.push(firmware[i]);
        } else {
            ext_memory.push(0);
        }
    }

    let mut computers = Vec::<Machine>::new();
    let mut packet_queues = Vec::<VecDeque<Packet>>::new();

    for address in 0..50 {
        let mut machine = Machine::new(ext_memory.clone());
        machine.input.push(address);
        computers.push(machine);
        packet_queues.push(VecDeque::new());
    }

    // 50 address is 255 NAT address
    packet_queues.push(VecDeque::new());

    let mut last_delivered_nat_y = -1;

    loop {
        let mut any_output = false;
        for machine in &mut computers {
            let machine_outputs = run_machine(machine, &mut packet_queues);
            if !part_two && let Some(packet_to_nat) = packet_queues[NAT_INDEX].pop_back() {
                return Some(packet_to_nat.y as u64);
            }
            any_output |= machine_outputs;
        }

        if part_two {
            // No output, queues are empty except for nat
            if !any_output && packet_queues[..NAT_INDEX].iter().all(|c| c.is_empty()) {
                // send last NAT packet to 0
                if let Some(nat_packet) = packet_queues[NAT_INDEX].pop_back() {
                    if last_delivered_nat_y == nat_packet.y {
                        return Some(nat_packet.y as u64);
                    }
                    last_delivered_nat_y = nat_packet.y;
                    packet_queues[0].push_back(nat_packet);
                }
            }
        }
    }
}

struct Packet {
    x: i64,
    y: i64,
}

fn run_machine(machine: &mut Machine, queues: &mut [VecDeque<Packet>]) -> bool {
    let machine_queue = &mut queues[machine.input[0] as usize];
    if let Some(packet) = machine_queue.pop_front() {
        machine.input.push(packet.x);
        machine.input.push(packet.y);
    } else {
        machine.input.push(-1);
    }

    let mut output_pointer = machine.output_pointer;
    machine.execute();

    let output_end = machine.output_pointer;
    let mut any_output = false;

    while output_end > output_pointer {
        let mut packet_to_send = machine.output.iter().skip(output_pointer);
        let mut address = *packet_to_send.next().expect("Expected packet address.") as usize;
        let x = packet_to_send.next().expect("Expected x in packet");
        let y = packet_to_send.next().expect("Expected y in packet");
        output_pointer += 3;

        if address == NAT_ADDRESS {
            address = NAT_INDEX;
        }
        queues[address].push_back(Packet { x: *x, y: *y });
        any_output = true;
    }

    any_output
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }

//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
