use anyhow::{bail, Context, Result};

type Data = Vec<Packet>;

// TODO, this could probably be refactored to be one big enum with Literal just being an operator

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualtTo,
}

impl Operator {
    fn from(val: usize) -> Self {
        match val {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Min,
            3 => Operator::Max,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::EqualtTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(usize),
    Operator(Operator, Vec<Packet>),
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: usize,
    packet_type: PacketType,
}

impl Packet {
    fn new(version: usize, packet_type: PacketType) -> Self {
        Self {
            version,
            packet_type,
        }
    }

    fn version_sum(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(_) => self.version,
            PacketType::Operator(_op, packets) => {
                self.version + packets.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        }
    }

    fn evaluate(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(x) => *x,
            PacketType::Operator(operator, packets) => {
                let packets = packets.iter().map(|p| p.evaluate());
                match operator {
                    Operator::Sum => packets.sum(),
                    Operator::Product => packets.product(),
                    Operator::Min => packets.min().expect("min not found"),
                    Operator::Max => packets.max().expect("max not found"),
                    Operator::GreaterThan => {
                        let packets: Vec<usize> = packets.collect();
                        assert!(packets.len() == 2);
                        (packets[0] > packets[1]) as usize
                    }
                    Operator::LessThan => {
                        let packets: Vec<usize> = packets.collect();
                        assert!(packets.len() == 2);
                        (packets[0] < packets[1]) as usize
                    }
                    Operator::EqualtTo => {
                        let packets: Vec<usize> = packets.collect();
                        assert!(packets.len() == 2);
                        (packets[0] == packets[1]) as usize
                    }
                }
            }
        }
    }
}

struct PacketParser {
    data: Vec<char>,
    current: usize,
}

impl PacketParser {
    fn new(data: Vec<char>) -> Self {
        Self { data, current: 0 }
    }

    fn parse(
        &mut self,
        scope_length: Option<usize>,
        scope_packets_limit: Option<usize>,
    ) -> Result<Vec<Packet>> {
        // println!("parse: {:?}", self.data[self.current..].iter());
        let mut packets = vec![];
        while !self.is_last_zeroes() {
            if let Some(scope_length) = scope_length {
                if self.current >= scope_length {
                    break;
                }
            }
            if let Some(scope_packets_limit) = scope_packets_limit {
                if packets.len() == scope_packets_limit {
                    break;
                }
            }

            let version = from_binary(&self.advance(3)?);
            let packet_type = match from_binary(&self.advance(3)?) {
                // Literal
                4 => {
                    let mut packets: Vec<char> = vec![];
                    loop {
                        let first = *self.next()?;
                        packets.extend(self.advance(4)?.iter());
                        if first == '0' {
                            break;
                        }
                    }
                    PacketType::Literal(from_binary(&packets))
                }
                //Operator
                op_type => {
                    let length_type_id = self.next()?;
                    // println!("length_type_id: {}", length_type_id);

                    // pass these constrain to parse() ???
                    PacketType::Operator(
                        Operator::from(op_type),
                        match length_type_id {
                            '0' => {
                                let length = from_binary(&self.advance(15)?);
                                // scope_end = current + length
                                self.parse(Some(self.current + length), None)?
                            }
                            '1' => {
                                let length = from_binary(&self.advance(11)?);
                                // packets.len() == scope_packets_length
                                self.parse(None, Some(length))?
                            }
                            _ => unreachable!(),
                        },
                    )
                }
            };
            let packet = Packet::new(version, packet_type);
            // println!("{:?}", packet);
            packets.push(packet);
        }

        Ok(packets)
    }

    fn advance(&mut self, size: usize) -> Result<Vec<char>> {
        let mut data = vec![];
        for _ in 0..size {
            data.push(*self.next().context(format!(
                "size {} is longer than data {}",
                size,
                data.len()
            ))?);
        }
        Ok(data)
    }

    fn next(&mut self) -> Result<&char> {
        if !self.is_at_end() {
            self.current += 1;
        } else {
            bail!("Reached the end unexpectedly")
        }
        Ok(self.previous())
    }

    fn previous(&self) -> &char {
        &self.data[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.data.len()
    }

    fn is_last_zeroes(&self) -> bool {
        self.data[self.current..].iter().all(|bit| *bit == '0')
    }

    fn _peek(&self) -> &char {
        &self.data[self.current]
    }
}

fn from_binary(chars: &[char]) -> usize {
    usize::from_str_radix(&String::from_iter(chars), 2).expect("Failed to parse binary")
}

pub fn parse(input: &str) -> Data {
    let binary: String = input
        .chars()
        .flat_map(|c| c.to_digit(16))
        .map(|d| format!("{:0>4b}", d))
        .collect();
    let mut parser = PacketParser::new(binary.chars().collect());
    parser.parse(None, None).expect("Failed to parse")
}

pub fn part_1(input: &Data) -> usize {
    assert!(input.len() == 1);
    input[0].version_sum()
}

pub fn part_2(input: &Data) -> usize {
    assert!(input.len() == 1);
    input[0].evaluate()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day16::{Operator, Packet, PacketParser, PacketType};

    #[test]
    pub fn parse() {
        let result = super::parse("D2FE28");
        assert_eq!(result, vec![Packet::new(6, PacketType::Literal(2021))]);

        let mut parser = PacketParser::new("11010001010".chars().collect());
        let result = parser.parse(None, None).expect("Failed to parse");
        assert_eq!(result, vec![Packet::new(6, PacketType::Literal(10))]);

        let mut parser = PacketParser::new("0101001000100100".chars().collect());
        let result = parser.parse(None, None).expect("Failed to parse");
        assert_eq!(result, vec![Packet::new(2, PacketType::Literal(20))]);

        let result = super::parse("38006F45291200");
        assert_eq!(
            result,
            vec![Packet {
                version: 1,
                packet_type: PacketType::Operator(
                    Operator::LessThan,
                    vec![
                        Packet::new(6, PacketType::Literal(10)),
                        Packet::new(2, PacketType::Literal(20))
                    ]
                )
            }]
        );

        let result = super::parse("EE00D40C823060");
        assert_eq!(
            result,
            vec![Packet {
                version: 7,
                packet_type: PacketType::Operator(
                    Operator::Max,
                    vec![
                        Packet::new(2, PacketType::Literal(1)),
                        Packet::new(4, PacketType::Literal(2)),
                        Packet::new(1, PacketType::Literal(3))
                    ]
                )
            }]
        );

        let result = super::parse("8A004A801A8002F478");
        assert_eq!(
            result,
            vec![Packet {
                version: 4,
                packet_type: PacketType::Operator(
                    Operator::Min,
                    vec![Packet::new(
                        1,
                        PacketType::Operator(
                            Operator::Min,
                            vec![Packet::new(
                                5,
                                PacketType::Operator(
                                    Operator::Min,
                                    vec![Packet::new(6, PacketType::Literal(15))]
                                )
                            )]
                        )
                    ),]
                )
            }]
        );

        let result = super::parse("620080001611562C8802118E34");
        let expected = vec![Packet {
            version: 3,
            packet_type: PacketType::Operator(
                Operator::Sum,
                vec![
                    Packet::new(
                        0,
                        PacketType::Operator(
                            Operator::Sum,
                            vec![
                                Packet::new(0, PacketType::Literal(10)),
                                Packet::new(5, PacketType::Literal(11)),
                            ],
                        ),
                    ),
                    Packet::new(
                        1,
                        PacketType::Operator(
                            Operator::Sum,
                            vec![
                                Packet::new(0, PacketType::Literal(12)),
                                Packet::new(3, PacketType::Literal(13)),
                            ],
                        ),
                    ),
                ],
            ),
        }];
        assert_eq!(result, expected);
    }

    #[test]
    pub fn part_1() {
        let input = super::parse("8A004A801A8002F478");
        let result = super::part_1(&input);
        assert_eq!(result, 16);

        let input = super::parse("620080001611562C8802118E34");
        let result = super::part_1(&input);
        assert_eq!(result, 12);

        let input = super::parse("C0015000016115A2E0802F182340");
        let result = super::part_1(&input);
        assert_eq!(result, 23);

        let input = super::parse("A0016C880162017C3686B18A3D4780");
        let result = super::part_1(&input);
        assert_eq!(result, 31);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse("C200B40A82");
        let result = super::part_2(&input);
        assert_eq!(result, 3);

        let input = super::parse("04005AC33890");
        let result = super::part_2(&input);
        assert_eq!(result, 54);

        let input = super::parse("880086C3E88112");
        let result = super::part_2(&input);
        assert_eq!(result, 7);

        let input = super::parse("CE00C43D881120");
        let result = super::part_2(&input);
        assert_eq!(result, 9);

        let input = super::parse("D8005AC2A8F0");
        let result = super::part_2(&input);
        assert_eq!(result, 1);

        let input = super::parse("F600BC2D8F");
        let result = super::part_2(&input);
        assert_eq!(result, 0);

        let input = super::parse("9C005AC2F8F0");
        let result = super::part_2(&input);
        assert_eq!(result, 0);

        let input = super::parse("9C0141080250320F1802104A08");
        let result = super::part_2(&input);
        assert_eq!(result, 1);
    }
}
