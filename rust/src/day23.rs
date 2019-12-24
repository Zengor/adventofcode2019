use crate::intcode::{IntcodeInput, IntcodeMachine, RunResult};
use std::{
    collections::VecDeque,
    io::{empty, sink},
};
const NAT_ADDRESS: i64 = 255;
struct Network {
    computers: Vec<IntcodeMachine>,
    nat_packet: Option<Packet>,
}

impl Network {
    fn new(source_code: &str) -> Self {
        let program = IntcodeMachine::from_str(source_code);
        let mut computers = Vec::new();
        for i in 0..50 {
            let mut computer = program.clone();
            computer.run_single_input(&mut Some(i), &mut std::io::sink());
            computers.push(computer);
        }
        Self {
            computers,
            nat_packet: None,
        }
    }

    fn run(&mut self, quick_end: bool) -> Packet {
        let mut packet_queues: Vec<_> = (0..self.computers.len())
            .map(|_| VecDeque::<Packet>::new())
            .collect();
        // auxiliary buffers to hold packet data while it's being built
        let mut out_buffers: Vec<_> = (0..self.computers.len())
            .map(|_| Vec::with_capacity(3))
            .collect();
        let mut since_out = 0;
        let mut last_packet = None;
        loop {
            // used to track idle network
            for (i, computer) in self.computers.iter_mut().enumerate() {
                let packet_queue = &mut packet_queues[i];
                let out_buffer = &mut out_buffers[i];
                match computer.step(&mut empty(), out_buffer) {
                    RunResult::InputRequest => {
                        // rerun step with proper input.
                        // since this is known to be an input instruction,
                        // output is impossible
                        if packet_queue.is_empty() {
                            computer.step(&mut Some(-1), &mut sink());
                        } else {
                            let packet = packet_queue.front_mut().unwrap();
                            computer.step(packet, &mut sink());
                            if packet.reads >= 2 {
                                packet_queue.pop_front();
                            }
                        }
                    }
                    _ => (),
                }
                if out_buffer.len() == 3 {
                    let (dest, x, y) = (out_buffer[0], out_buffer[1], out_buffer[2]);
                    let packet = Packet::new(x, y);
                    if dest == NAT_ADDRESS {
                        if quick_end {
                            return packet;
                        }
                        self.nat_packet = Some(packet);
                    } else {
                        packet_queues[dest as usize].push_back(packet);
                    }
                    out_buffer.clear();
                }
            }
            // this is a bit confusing to take advantage of shortcircuiting
            // if any packet queues are not empty, do not increment
            if !packet_queues.iter().any(|q| q.len() > 0) {
                since_out += 1;
            } else {
                since_out = 0;
            }
            // not sure what the definition of "continuously" is
            // supposed to be exactly, but 10000 times in a row is
            // probably more than enough.
            if since_out > 1000 {
                let packet = self.nat_packet.take().unwrap();
                if last_packet.is_some() && last_packet.unwrap() == packet {
                    return packet;
                }
                packet_queues[0].push_back(packet.clone());
                last_packet = Some(packet);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Packet {
    x: i64,
    y: i64,
    reads: u8,
}

impl Packet {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y, reads: 0 }
    }
}

impl IntcodeInput for Packet {
    fn read(&mut self) -> Option<i64> {
        self.reads += 1;
        match self.reads {
            1 => Some(self.x),
            2 => Some(self.y),
            _ => None,
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut network = Network::new(input);
    network.run(true).y
}

pub fn part2(input: &str) -> i64 {
    let mut network = Network::new(input);
    network.run(false).y
}
