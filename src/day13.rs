use serde::Deserialize;
use serde_json;
use std::str::FromStr;

use crate::util::{convert_str_to_sections, parse_lines_into, read_input_to_str, AOCSolution};

solution!(Day 13 => PacketReceiver);

#[derive(Debug)]
pub struct PacketReceiver {
    packet_pairs: Vec<PacketPair>,
}

impl AOCSolution for PacketReceiver {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        let good_packets = self.get_good_packet_indices();
        format!("{}", good_packets.iter().sum::<usize>())
    }

    fn part_2(&mut self) -> String {
        format!("{}", self.get_decoder_key())
    }
}

impl PacketReceiver {
    pub fn new_from_file(input_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_path, true)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pair_blocks = convert_str_to_sections(input_str, true)?;
        let mut packet_pairs = Vec::new();
        for block in pair_blocks.iter() {
            let packet_pair = block.parse::<PacketPair>()?;
            packet_pairs.push(packet_pair)
        }
        Ok(Self { packet_pairs })
    }

    pub fn get_good_packet_indices(&self) -> Vec<usize> {
        let mut good = Vec::new();
        for (idx, pair) in self.packet_pairs.iter().enumerate() {
            if pair.left < pair.right {
                good.push(idx + 1); // Answer is 1-indexed
            }
        }
        good
    }

    pub fn get_decoder_key(&self) -> usize {
        let ordered_packets = self.construct_ordered_message();
        let mut idxprod: usize = 1;
        for (idx, packet) in ordered_packets.iter().enumerate() {
            if *packet == self.divider_packet(2) || *packet == self.divider_packet(6) {
                idxprod *= idx + 1; // 1-indexed
            }
        }
        idxprod
    }

    fn construct_ordered_message(&self) -> Vec<PacketValue> {
        let mut all_packets = self.get_all_packets();

        all_packets.push(self.divider_packet(2));
        all_packets.push(self.divider_packet(6));
        all_packets.sort_unstable();
        all_packets
    }

    fn get_all_packets(&self) -> Vec<PacketValue> {
        self.packet_pairs
            .iter()
            .map(|p| vec![p.left.clone(), p.right.clone()])
            .flatten()
            .collect::<Vec<PacketValue>>()
    }

    fn divider_packet(&self, value: usize) -> PacketValue {
        PacketValue::List(vec![Box::new(PacketValue::List(vec![Box::new(
            PacketValue::Int(value),
        )]))])
    }
}

#[derive(Debug)]
struct PacketPair {
    left: PacketValue,
    right: PacketValue,
}

impl FromStr for PacketPair {
    type Err = PacketPairParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let packets = parse_lines_into::<PacketValue>(s)
            .map_err(|_| PacketPairParseError("Invalid packet"))?;
        if packets.len() != 2 {
            return Err(PacketPairParseError("Invalid number of packets for pair"));
        }
        Ok(Self {
            left: packets[0].clone(),
            right: packets[1].clone(),
        })
    }
}

custom_error!(PacketPairParseError);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum PacketValue {
    Int(usize),
    List(PacketList),
}

type PacketList = Vec<Box<PacketValue>>;

impl FromStr for PacketValue {
    type Err = PacketValueParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| PacketValueParseError("Unable to parse packet"))
    }
}

custom_error!(PacketValueParseError);

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketValue::Int(i), PacketValue::Int(j)) => i.partial_cmp(j),
            (PacketValue::Int(_), PacketValue::List(_)) => {
                PacketValue::List(vec![Box::new(self.clone())]).partial_cmp(other)
            }
            (PacketValue::List(_), PacketValue::Int(_)) => {
                self.partial_cmp(&PacketValue::List(vec![Box::new(other.clone())]))
            }
            (PacketValue::List(l1), PacketValue::List(l2)) => {
                let pairs = l1.iter().zip(l2.iter());
                for pair in pairs {
                    if let Some(ord) = pair.0.partial_cmp(&pair.1) {
                        if ord == std::cmp::Ordering::Equal {
                            continue;
                        }
                        return Some(ord);
                    }
                }
                l1.len().partial_cmp(&l2.len())
            }
        }
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given() {
        let input_str = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]        
        ";

        let mut recv = PacketReceiver::new_from_str(input_str).expect("Failed parse");
        assert_eq!(recv.part_1(), "13");
        assert_eq!(recv.part_2(), "140");
    }
}
