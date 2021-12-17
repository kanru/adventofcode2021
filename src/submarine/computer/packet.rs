use nom::{
    bits::complete::{tag, take},
    branch::alt,
    combinator::{map, verify},
    multi::many_till,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: usize,
    type_id: TypeId,
    value: Option<usize>,
    sub_packets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub enum TypeId {
    LiteralValue,
    SumOp,
    ProductOp,
    MinOp,
    MaxOp,
    GreaterOp,
    LessOp,
    EqualOp,
    Other(u8),
}

#[derive(Debug, PartialEq)]
pub enum LengthType {
    TotalBits,
    Number,
}

impl Packet {
    pub fn from_hex(input: &str) -> Packet {
        let input = hex_to_bytes(input);
        let (_, packet) = alt((value_packet, operator_packet))((&input, 0)).unwrap();
        packet
    }

    pub fn version_sum(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(|p| p.version_sum())
                .sum::<usize>()
    }

    pub fn eval(&self) -> usize {
        match self.type_id {
            TypeId::LiteralValue => self.value.unwrap(),
            TypeId::SumOp => self.sub_packets.iter().map(|p| p.eval()).sum(),
            TypeId::ProductOp => self.sub_packets.iter().map(|p| p.eval()).product(),
            TypeId::MinOp => self.sub_packets.iter().map(|p| p.eval()).min().unwrap(),
            TypeId::MaxOp => self.sub_packets.iter().map(|p| p.eval()).max().unwrap(),
            TypeId::GreaterOp | TypeId::LessOp | TypeId::EqualOp => {
                let sub1 = &self.sub_packets[0];
                let sub2 = &self.sub_packets[1];
                match (&self.type_id, sub1.eval().cmp(&sub2.eval())) {
                    (TypeId::LessOp, std::cmp::Ordering::Less) => 1,
                    (TypeId::EqualOp, std::cmp::Ordering::Equal) => 1,
                    (TypeId::GreaterOp, std::cmp::Ordering::Greater) => 1,
                    _ => 0,
                }
            }
            TypeId::Other(_) => todo!(),
        }
    }

    pub fn dot(&self) {
        println!(
            "\"{:p}\" [label=\"{{ver|{}}}|{{op|{:?}}}|{{val|{:?}}}\"]",
            self, self.version, self.type_id, self.value
        );
        match self.type_id {
            TypeId::LiteralValue => (),
            _ => {
                for p in &self.sub_packets {
                    println!("\"{:p}\" -> {}", self, p.pptr());
                }
                for p in &self.sub_packets {
                    p.dot();
                }
            }
        }
    }

    pub fn pptr(&self) -> String {
        format!("\"{:p}\"", self)
    }
}

fn hex_to_bytes(input: &str) -> Vec<u8> {
    let bytes: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect();
    bytes
        .chunks_exact(2)
        .map(|it| (it[0] << 4) + it[1])
        .collect()
}

fn value_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let value_type_id = verify(type_id, |type_id| type_id == &TypeId::LiteralValue);
    map(
        tuple((packet_version, value_type_id, literal_value)),
        |(version, type_id, value)| Packet {
            version,
            type_id,
            value: value.into(),
            sub_packets: vec![],
        },
    )(input)
}

fn bits_len(input: (&[u8], usize)) -> usize {
    8 * input.0.len() - input.1
}

fn operator_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let mut other_type_id = verify(type_id, |type_id| type_id != &TypeId::LiteralValue);
    let (input, version) = packet_version(input)?;
    let (input, type_id) = other_type_id(input)?;
    let (input, length_type) = length_type(input)?;
    let (input, sub_packets) = match length_type {
        LengthType::TotalBits => {
            let (input, length) = take(15u8)(input)?;
            let bits: usize = length;
            let mut rest = input;
            let mut sub_packets = vec![];
            loop {
                let (r, packet) = alt((value_packet, operator_packet))(rest)?;
                rest = r;
                sub_packets.push(packet);
                if bits_len(input) - bits_len(rest) >= bits {
                    break;
                }
            }
            (rest, sub_packets)
        }
        LengthType::Number => {
            let (input, length) = take(11u8)(input)?;
            let num_packets: usize = length;
            let mut rest = input;
            let mut sub_packets = vec![];
            for _ in 0..num_packets {
                let (r, packet) = alt((value_packet, operator_packet))(rest)?;
                rest = r;
                sub_packets.push(packet);
            }
            (rest, sub_packets)
        }
    };
    Ok((
        input,
        Packet {
            version,
            type_id,
            value: None,
            sub_packets,
        },
    ))
}

fn packet_version(input: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    take(3u8)(input)
}

fn type_id(input: (&[u8], usize)) -> IResult<(&[u8], usize), TypeId> {
    map(take(3u8), |type_id| match type_id {
        0 => TypeId::SumOp,
        1 => TypeId::ProductOp,
        2 => TypeId::MinOp,
        3 => TypeId::MaxOp,
        4 => TypeId::LiteralValue,
        5 => TypeId::GreaterOp,
        6 => TypeId::LessOp,
        7 => TypeId::EqualOp,
        _ => TypeId::Other(type_id),
    })(input)
}

fn length_type(input: (&[u8], usize)) -> IResult<(&[u8], usize), LengthType> {
    map(take(1u8), |length_type| match length_type {
        0 => LengthType::TotalBits,
        1 => LengthType::Number,
        _ => unreachable!("bit is not 0 or 1"),
    })(input)
}

fn literal_value(input: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    map(
        many_till(
            preceded(tag(1, 1u8), take(4u8)),
            preceded(tag(0, 1u8), take(4u8)),
        ),
        |(body, tail): (Vec<usize>, usize)| {
            let mut value = 0;
            for v in body {
                value += v;
                value <<= 4;
            }
            value += tail;
            value
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::submarine::computer::packet::{
        hex_to_bytes, literal_value, packet_version, type_id, value_packet, Packet, TypeId,
    };

    #[test]
    fn test_parse_packet_version() {
        let input = [0b00000110];
        assert_eq!(packet_version((&input, 5)), Ok((([].as_ref(), 0), 6)));
        let input = [0b00100000];
        assert_eq!(
            packet_version((&input, 0)),
            Ok((([0b00100000].as_ref(), 3), 1))
        );
    }

    #[test]
    fn test_parse_type_id() {
        let input = [0b00000100];
        assert_eq!(
            type_id((&input, 5)),
            Ok((([].as_ref(), 0), TypeId::LiteralValue))
        );
        let input = [0b11000000];
        assert_eq!(
            type_id((&input, 0)),
            Ok((([0b11000000].as_ref(), 3), TypeId::LessOp))
        );
    }

    #[test]
    fn test_parse_literal_value() {
        let input = [0b10111111, 0b10001010];
        assert_eq!(
            literal_value((&input, 0)),
            Ok((([0b10001010].as_ref(), 7), 2021))
        );
        let input = [0b01011111, 0b11000101];
        assert_eq!(literal_value((&input, 1)), Ok((([].as_ref(), 0), 2021)));
    }

    #[test]
    fn test_parse_literal_value_packet() {
        let input = [0b11010010, 0b11111110, 0b00101000];
        let packet = value_packet((&input, 0)).unwrap().1;
        assert_eq!(
            Packet {
                version: 6,
                type_id: TypeId::LiteralValue,
                value: Some(2021),
                sub_packets: vec![]
            },
            packet
        );
    }

    #[test]
    fn test_hex_to_bytes() {
        assert_eq!(
            vec![
                0b00111000, 0b00000000, 0b01101111, 0b01000101, 0b00101001, 0b00010010, 0b00000000
            ],
            hex_to_bytes("38006F45291200")
        )
    }

    #[test]
    fn test_parse_literal_value_packet_hex() {
        let input = "D2FE28";
        let packet = Packet::from_hex(input);
        assert_eq!(
            Packet {
                version: 6,
                type_id: TypeId::LiteralValue,
                value: Some(2021),
                sub_packets: vec![]
            },
            packet
        );
    }

    #[test]
    fn test_parse_sub_packets() {
        let input = "38006F45291200";
        let packet = Packet::from_hex(input);
        assert_eq!(
            Packet {
                version: 1,
                type_id: TypeId::LessOp,
                value: None,
                sub_packets: vec![
                    Packet {
                        version: 6,
                        type_id: TypeId::LiteralValue,
                        value: Some(10),
                        sub_packets: vec![],
                    },
                    Packet {
                        version: 2,
                        type_id: TypeId::LiteralValue,
                        value: Some(20),
                        sub_packets: vec![]
                    }
                ]
            },
            packet
        );

        let input = "EE00D40C823060";
        let packet = Packet::from_hex(input);
        assert_eq!(
            Packet {
                version: 7,
                type_id: TypeId::MaxOp,
                value: None,
                sub_packets: vec![
                    Packet {
                        version: 2,
                        type_id: TypeId::LiteralValue,
                        value: Some(1),
                        sub_packets: vec![],
                    },
                    Packet {
                        version: 4,
                        type_id: TypeId::LiteralValue,
                        value: Some(2),
                        sub_packets: vec![]
                    },
                    Packet {
                        version: 1,
                        type_id: TypeId::LiteralValue,
                        value: Some(3),
                        sub_packets: vec![]
                    }
                ]
            },
            packet
        );
    }

    #[test]
    fn test_day16_part1() {
        assert_eq!(
            [16, 12, 23, 31],
            [
                Packet::from_hex("8A004A801A8002F478").version_sum(),
                Packet::from_hex("620080001611562C8802118E34").version_sum(),
                Packet::from_hex("C0015000016115A2E0802F182340").version_sum(),
                Packet::from_hex("A0016C880162017C3686B18A3D4780").version_sum(),
            ]
        )
    }

    #[test]
    fn test_day16_part2() {
        assert_eq!(
            [3, 54, 7, 9, 1, 0, 0, 1],
            [
                Packet::from_hex("C200B40A82").eval(),
                Packet::from_hex("04005AC33890").eval(),
                Packet::from_hex("880086C3E88112").eval(),
                Packet::from_hex("CE00C43D881120").eval(),
                Packet::from_hex("D8005AC2A8F0").eval(),
                Packet::from_hex("F600BC2D8F").eval(),
                Packet::from_hex("9C005AC2F8F0").eval(),
                Packet::from_hex("9C0141080250320F1802104A08").eval(),
            ]
        )
    }
}
