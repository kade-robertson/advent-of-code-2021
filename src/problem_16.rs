use crate::problem::Problem;

#[derive(Debug)]
pub struct Packet {
    version: u8,
    type_id: u8,
    value: u64,
    subpackets: Vec<Packet>,
}

impl Packet {
    pub fn sum_versions(&self) -> i64 {
        self.version as i64
            + self
                .subpackets
                .iter()
                .fold(0, |acc, p| acc + p.sum_versions())
    }
}

fn parse_packet(packet: &Vec<u8>, packet_limit: usize, bit_limit: usize) -> (Vec<Packet>, usize) {
    let mut index = 0;
    let mut packet_stack = Vec::new();
    while index < packet.len() - 6 {
        if bit_limit > 0 && index >= bit_limit {
            break;
        }
        if packet_limit > 0 && packet_stack.len() == packet_limit {
            break;
        }

        let version = (packet[index] << 2) + (packet[index + 1] << 1) + packet[index + 2];
        let type_id = (packet[index + 3] << 2) + (packet[index + 4] << 1) + packet[index + 5];

        if type_id == 0x4 {
            index += 6;
            let mut value = 0;
            loop {
                let first_bit = packet[index];
                value = (value << 4)
                    + ((packet[index + 1] << 3) as u64
                        + (packet[index + 2] << 2) as u64
                        + (packet[index + 3] << 1) as u64
                        + packet[index + 4] as u64);
                index += 5;
                if first_bit == 0 {
                    break;
                }
            }
            packet_stack.push(Packet {
                version,
                type_id,
                value,
                subpackets: Vec::with_capacity(0),
            });
        } else {
            let length_type = packet[index + 6];
            let offset = match length_type {
                0x0 => 15,
                _ => 11,
            };
            index += 7;
            if index >= packet.len() - (offset + 1) {
                continue;
            }
            let length = packet[index..index + offset]
                .iter()
                .fold(0usize, |acc, v| (acc << 1) + (*v as usize & 0x1));

            let subpackets = parse_packet(
                &packet[index + offset..].to_vec(),
                match length_type == 0x1 {
                    true => length,
                    _ => 0,
                },
                match length_type == 0x0 {
                    true => length,
                    _ => 0,
                },
            );

            packet_stack.push(Packet {
                version,
                type_id,
                value: 0,
                subpackets: subpackets.0,
            });
            index += offset + subpackets.1;
        }
    }
    (packet_stack, index)
}

pub struct Problem16 {}

impl Problem16 {
    pub fn new() -> Problem16 {
        Problem16 {}
    }

    fn parse(&self, input: String) -> Vec<u8> {
        input
            .chars()
            .map(|c| {
                let parsed = c.to_digit(16).unwrap() as u8;
                vec![
                    (parsed & 0x8) >> 3,
                    (parsed & 0x4) >> 2,
                    (parsed & 0x2) >> 1,
                    parsed & 0x1,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>()
    }

    fn solve_actual(&self, packet: &Vec<u8>) -> i64 {
        let parsed_packet = &parse_packet(packet, 0, 0).0[0];
        parsed_packet.sum_versions()
    }
}

impl Problem for Problem16 {
    fn name(&self) -> &str {
        "Day 16: Packet Decoder"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_16.txt");
        let packet = self.parse(input);
        self.solve_actual(&packet)
    }

    fn solve_part2(&self) -> (i64, Option<String>) {
        (0, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example_01() {
        let problem = Problem16::new();
        let input = get_input!("./inputs/problem_16_example_01.txt");
        let packet = problem.parse(input);
        assert_eq!(problem.solve_actual(&packet), 6);
    }

    #[test]
    fn test_solve_actual_from_example_02() {
        let problem = Problem16::new();
        let input = get_input!("./inputs/problem_16_example_02.txt");
        let packet = problem.parse(input);
        assert_eq!(problem.solve_actual(&packet), 9);
    }

    #[test]
    fn test_solve_actual_from_example_03() {
        let problem = Problem16::new();
        let input = get_input!("./inputs/problem_16_example_03.txt");
        let packet = problem.parse(input);
        assert_eq!(problem.solve_actual(&packet), 14);
    }

    #[test]
    fn test_solve_actual_from_example_04() {
        let problem = Problem16::new();
        let input = get_input!("./inputs/problem_16_example_04.txt");
        let packet = problem.parse(input);
        assert_eq!(problem.solve_actual(&packet), 16);
    }

    #[test]
    fn test_solve_actual_from_example_05() {
        let problem = Problem16::new();
        let input = get_input!("./inputs/problem_16_example_05.txt");
        let packet = problem.parse(input);
        assert_eq!(problem.solve_actual(&packet), 12);
    }

    #[test]
    fn test_solve_actual_from_example_06() {
        let problem = Problem16::new();
        let input = get_input!("./inputs/problem_16_example_06.txt");
        let packet = problem.parse(input);
        assert_eq!(problem.solve_actual(&packet), 23);
    }

    #[test]
    fn test_solve_actual_from_example_07() {
        let problem = Problem16::new();
        let input = get_input!("./inputs/problem_16_example_07.txt");
        let packet = problem.parse(input);
        assert_eq!(problem.solve_actual(&packet), 31);
    }
}
