use bitreader::BitReader;
use hex::FromHex;
use std::fs::File;
use std::io::{self, BufRead};

fn load_array() -> Vec<u8> {
    let file = File::open("16.txt").unwrap();
    let mut lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let hexstring = lines.next().unwrap().unwrap();
    return Vec::from_hex(hexstring).unwrap();
}
#[derive(Debug)]
struct Packet {
    version: u8,
    kind: u8,
    payload: Payload,
}
#[derive(Debug)]
enum OperatorLength {
    Bits(u64),
    Packets(u64),
}
#[derive(Debug)]
enum Payload {
    Literal {
        value: u64,
    },
    Operator {
        length: OperatorLength,
        children: Vec<Packet>,
    },
}

impl OperatorLength {
    fn parse(reader: &mut BitReader) -> OperatorLength {
        let length_type_id = reader.read_bool().unwrap();
        if length_type_id {
            OperatorLength::Packets(reader.read_u64(11).unwrap())
        } else {
            OperatorLength::Bits(reader.read_u64(15).unwrap())
        }
    }
}

impl Packet {
    fn parse(reader: &mut BitReader) -> Packet {
        let version = reader.read_u8(3).unwrap();
        let kind = reader.read_u8(3).unwrap();
        if kind == 4 {
            let mut literal_value: u64 = 0;
            while {
                let more_packets = reader.read_bool().unwrap();
                let bytes = reader.read_u64(4).unwrap();
                literal_value <<= 4;
                literal_value |= bytes;
                more_packets
            } {}
            Packet {
                version,
                kind,
                payload: Payload::Literal {
                    value: literal_value,
                },
            }
        } else {
            let op_length = OperatorLength::parse(reader);
            let start_position = reader.position();
            let mut children = Vec::new();
            while match op_length {
                OperatorLength::Bits(b) => reader.position() - start_position < b as u64,
                OperatorLength::Packets(p) => children.len() < p as usize,
            } {
                children.push(Packet::parse(reader));
            }
            Packet {
                version,
                kind,
                payload: Payload::Operator {
                    length: op_length,
                    children,
                },
            }
        }
    }

    fn eval(&self) -> u64 {
        match &self.payload {
            Payload::Literal { value } => *value,
            Payload::Operator { children, .. } => match self.kind {
                0 => children.iter().map(Packet::eval).sum(),
                1 => children.iter().map(Packet::eval).product(),
                2 => children.iter().map(Packet::eval).min().unwrap(),
                3 => children.iter().map(Packet::eval).max().unwrap(),
                5 => if children[0].eval() > children[1].eval() {1} else {0},
                6 => if children[0].eval() < children[1].eval() {1} else {0},
                7 => if children[0].eval() == children[1].eval() {1} else {0},
                _ => panic!("Unrecognized operator!"),
            },
        }
    }
}

fn version_sum(packet: &Packet) -> u64 {
    match &packet.payload {
        Payload::Literal { .. } => packet.version.into(),
        Payload::Operator { children, .. } => {
            packet.version as u64 + children.iter().map(version_sum).sum::<u64>()
        }
    }
}

#[test]
fn part1() {
    let array = load_array();
    let mut reader = BitReader::new(array.as_slice());
    let packet = Packet::parse(&mut reader);
    println!("packet {:#?}", packet);
    println!("version sum {}", version_sum(&packet));
}

#[test]
fn part2() {
    let array = load_array();
    let mut reader = BitReader::new(array.as_slice());
    let packet = Packet::parse(&mut reader);
    println!("packet {:#?}", packet);
    println!("value {}", packet.eval());
}
