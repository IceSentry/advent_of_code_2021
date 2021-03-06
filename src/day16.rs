use bitvec::{order::Msb0, prelude::BitVec};

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
            _ => unreachable!("invalid operator: {}", val),
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
}

struct BitReader {
    data: BitVec<Msb0, u8>,
    current: usize,
}

impl BitReader {
    fn from_hex_string(str: &str) -> Self {
        let bits = BitVec::from_iter((0..str.len()).step_by(2).map(|i| {
            u8::from_str_radix(&str[i..=i + 1], 16)
                .unwrap_or_else(|_| panic!("trying to parse {}", &str[i..i + 2]))
        }));
        Self {
            data: bits,
            current: 0,
        }
    }

    fn advance(&mut self, size: usize) -> usize {
        let mut acc = 0;
        let target = self.current + size;
        while self.current < target {
            acc = acc << 1 | self.next() as usize;
        }
        acc
    }

    fn next(&mut self) -> bool {
        self.current += 1;
        self.data[self.current - 1]
    }
}

fn parse_packet(reader: &mut BitReader) -> Packet {
    let version = reader.advance(3);
    let packet_type = match reader.advance(3) {
        4 => {
            let mut acc = 0;
            loop {
                let is_last_packet = reader.next();
                let val = reader.advance(4);
                acc = acc << 4 | val;
                if !is_last_packet {
                    break;
                }
            }
            PacketType::Literal(acc)
        }
        operator => {
            let length_type_id = reader.next();
            let mut packets = vec![];
            match length_type_id {
                false => {
                    let length = reader.advance(15);
                    let target = reader.current + length;
                    while reader.current < target {
                        packets.push(parse_packet(reader));
                    }
                }
                true => {
                    let length = reader.advance(11);
                    while packets.len() < length {
                        packets.push(parse_packet(reader));
                    }
                }
            };
            PacketType::Operator(Operator::from(operator), packets)
        }
    };
    Packet::new(version, packet_type)
}

pub fn parse(input: &str) -> Packet {
    let input = input.trim_end();
    parse_packet(&mut BitReader::from_hex_string(input))
}

pub fn part_1(packet: &Packet) -> usize {
    match &packet.packet_type {
        PacketType::Literal(_) => packet.version,
        PacketType::Operator(_op, packets) => {
            packet.version + packets.iter().map(part_1).sum::<usize>()
        }
    }
}

pub fn part_2(packet: &Packet) -> usize {
    match &packet.packet_type {
        PacketType::Literal(x) => *x,
        PacketType::Operator(operator, packets) => {
            let mut packets = packets.iter().map(part_2);
            match operator {
                Operator::Sum => packets.sum(),
                Operator::Product => packets.product(),
                Operator::Min => packets.min().unwrap(),
                Operator::Max => packets.max().unwrap(),
                Operator::GreaterThan => (packets.next() > packets.next()) as usize,
                Operator::LessThan => (packets.next() < packets.next()) as usize,
                Operator::EqualtTo => (packets.next() == packets.next()) as usize,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn parse() {
        use super::{parse, Operator, Packet, PacketType};

        assert_eq!(parse("D2FE28"), Packet::new(6, PacketType::Literal(2021)));

        assert_eq!(
            parse("38006F45291200"),
            Packet::new(
                1,
                PacketType::Operator(
                    Operator::LessThan,
                    vec![
                        Packet::new(6, PacketType::Literal(10)),
                        Packet::new(2, PacketType::Literal(20)),
                    ],
                ),
            )
        );

        assert_eq!(
            parse("EE00D40C823060"),
            Packet::new(
                7,
                PacketType::Operator(
                    Operator::Max,
                    vec![
                        Packet::new(2, PacketType::Literal(1)),
                        Packet::new(4, PacketType::Literal(2)),
                        Packet::new(1, PacketType::Literal(3)),
                    ],
                ),
            )
        );

        assert_eq!(
            parse("8A004A801A8002F478"),
            Packet::new(
                4,
                PacketType::Operator(
                    Operator::Min,
                    vec![Packet::new(
                        1,
                        PacketType::Operator(
                            Operator::Min,
                            vec![Packet::new(
                                5,
                                PacketType::Operator(
                                    Operator::Min,
                                    vec![Packet::new(6, PacketType::Literal(15))],
                                ),
                            )],
                        ),
                    )],
                ),
            )
        );

        assert_eq!(
            parse("620080001611562C8802118E34"),
            Packet::new(
                3,
                PacketType::Operator(
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
            )
        );
    }

    #[test]
    pub fn part_1() {
        use super::{parse, part_1};

        assert_eq!(part_1(&parse("8A004A801A8002F478")), 16);
        assert_eq!(part_1(&parse("620080001611562C8802118E34")), 12);
        assert_eq!(part_1(&parse("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part_1(&parse("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    pub fn part_2() {
        use super::{parse, part_2};

        assert_eq!(part_2(&parse("C200B40A82")), 3);
        assert_eq!(part_2(&parse("04005AC33890")), 54);
        assert_eq!(part_2(&parse("880086C3E88112")), 7);
        assert_eq!(part_2(&parse("CE00C43D881120")), 9);
        assert_eq!(part_2(&parse("D8005AC2A8F0")), 1);
        assert_eq!(part_2(&parse("F600BC2D8F")), 0);
        assert_eq!(part_2(&parse("9C005AC2F8F0")), 0);
        assert_eq!(part_2(&parse("9C0141080250320F1802104A08")), 1);
    }
}
