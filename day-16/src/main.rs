use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-16.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let bits = hex_to_bin(input)?;

    let packet = parse_packet(&bits)?;

    Ok(packet.version_sum())
}

fn part_2(input: &str) -> Result<u64> {
    let bits = hex_to_bin(input)?;

    let packet = parse_packet(&bits)?;

    Ok(packet.value)
}

#[derive(Default)]
struct Packet {
    version: u8,
    type_id: u8,
    size: usize,
    value: u64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version as u32
            + self
                .sub_packets
                .iter()
                .map(|sub_packet| sub_packet.version_sum())
                .sum::<u32>()
    }
}

/// Converts the input hex string into bit vector.
fn hex_to_bin(hex: &str) -> Result<Vec<u8>> {
    let mut bin = vec![0; hex.len() * 4];

    for index in 0..hex.len() {
        let bits = match hex.as_bytes()[index] {
            b'0' => [0, 0, 0, 0],
            b'1' => [0, 0, 0, 1],
            b'2' => [0, 0, 1, 0],
            b'3' => [0, 0, 1, 1],
            b'4' => [0, 1, 0, 0],
            b'5' => [0, 1, 0, 1],
            b'6' => [0, 1, 1, 0],
            b'7' => [0, 1, 1, 1],
            b'8' => [1, 0, 0, 0],
            b'9' => [1, 0, 0, 1],
            b'a' | b'A' => [1, 0, 1, 0],
            b'b' | b'B' => [1, 0, 1, 1],
            b'c' | b'C' => [1, 1, 0, 0],
            b'd' | b'D' => [1, 1, 0, 1],
            b'e' | b'E' => [1, 1, 1, 0],
            b'f' | b'F' => [1, 1, 1, 1],
            x => return Err(anyhow!("Invalid hex value: {}", x)),
        };

        bin[index * 4] = bits[0];
        bin[index * 4 + 1] = bits[1];
        bin[index * 4 + 2] = bits[2];
        bin[index * 4 + 3] = bits[3];
    }

    Ok(bin)
}

/// Converts the input bits into a Packet.
fn parse_packet(bits: &[u8]) -> Result<Packet> {
    let mut packet = Packet {
        version: bin_to_u8(&bits[0..3])?,
        type_id: bin_to_u8(&bits[3..6])?,
        size: bits.len(),
        ..Default::default()
    };

    if packet.type_id == 4 {
        // Literal -- do early return.

        let (literal, literal_size) = parse_literal(&bits[6..])?;

        packet.value = literal;
        packet.size = 6 + literal_size;

        return Ok(packet);
    }

    if bits[6] == 0 {
        // Operator with total sub-packet length.

        let len = bin_to_u64(&bits[7..22])? as usize;

        packet.size = 22 + len;

        packet.sub_packets = parse_sub_packets_by_len(&bits[22..packet.size])?;
    } else {
        // Operator with sub-packet count.

        let count = bin_to_u64(&bits[7..18])? as usize;

        packet.sub_packets = parse_sub_packets_by_count(&bits[18..], count)?;

        packet.size = 18
            + packet
                .sub_packets
                .iter()
                .map(|sub_packet| sub_packet.size)
                .sum::<usize>();
    };

    // Update value for operator.
    match packet.type_id {
        0 => {
            packet.value = packet
                .sub_packets
                .iter()
                .map(|sub_packet| sub_packet.value)
                .sum()
        }
        1 => {
            packet.value = packet
                .sub_packets
                .iter()
                .map(|sub_packet| sub_packet.value)
                .product()
        }
        2 => {
            packet.value = packet
                .sub_packets
                .iter()
                .map(|sub_packet| sub_packet.value)
                .min()
                .ok_or(anyhow!("Cannot find min from sub-packets"))?
        }
        3 => {
            packet.value = packet
                .sub_packets
                .iter()
                .map(|sub_packet| sub_packet.value)
                .max()
                .ok_or(anyhow!("Cannot find max from sub-packets"))?
        }
        5 => {
            if packet.sub_packets.len() < 2 {
                return Err(anyhow!("Insufficient sub-packets for greater than op"));
            }

            packet.value = (packet.sub_packets[0].value > packet.sub_packets[1].value) as u64;
        }
        6 => {
            if packet.sub_packets.len() < 2 {
                return Err(anyhow!("Insufficient sub-packets for less than op"));
            }

            packet.value = (packet.sub_packets[0].value < packet.sub_packets[1].value) as u64;
        }
        7 => {
            if packet.sub_packets.len() < 2 {
                return Err(anyhow!("Insufficient sub-packets for equal to op"));
            }

            packet.value = (packet.sub_packets[0].value == packet.sub_packets[1].value) as u64;
        }
        _ => return Err(anyhow!("Invalid type ID: {}", packet.type_id)),
    }

    Ok(packet)
}

/// Converts the input bits into a literal value and the number of bits that are used to encode the
/// value.
fn parse_literal(bits: &[u8]) -> Result<(u64, usize)> {
    let mut data_bits: Vec<u8> = Vec::new();

    let mut index = 0usize;
    loop {
        if bits.len() < (index + 1) * 5 {
            return Err(anyhow!("Unexpected end of bits"));
        }

        data_bits.extend_from_slice(&bits[(index * 5 + 1)..(index * 5 + 5)]);

        if bits[index * 5] == 0 {
            break;
        }

        index += 1;
    }

    Ok((bin_to_u64(&data_bits)?, (index + 1) * 5))
}

/// Converts the input bits into a list of sub-packets.
fn parse_sub_packets_by_len(mut bits: &[u8]) -> Result<Vec<Packet>> {
    let mut sub_packets: Vec<Packet> = Vec::new();

    loop {
        let sub_packet = parse_packet(bits)?;
        bits = &bits[sub_packet.size..]; // shorten the bit slice by previous sub-packet size

        sub_packets.push(sub_packet);

        if bits.iter().all(|bit| *bit == 0) {
            // empty return true
            break;
        }
    }

    Ok(sub_packets)
}

/// Converts the input bits into a list of sub-packets.
fn parse_sub_packets_by_count(mut bits: &[u8], count: usize) -> Result<Vec<Packet>> {
    let mut sub_packets: Vec<Packet> = Vec::new();

    for _ in 0..count {
        let sub_packet = parse_packet(bits)?;
        bits = &bits[sub_packet.size..]; // shorten the bit slice by previous sub-packet size

        sub_packets.push(sub_packet);
    }

    Ok(sub_packets)
}

/// Collapses the input bits into a u8 value.
fn bin_to_u8(bits: &[u8]) -> Result<u8> {
    if bits.len() > 8 {
        return Err(anyhow!("Unexpected bits len: {}", bits.len()));
    }

    bits.iter()
        .copied()
        .reduce(|acc, bit| (acc << 1) + bit)
        .ok_or(anyhow!("Cannot collapse bits to u8"))
}

/// Collapses the input bits into a u64 value.
fn bin_to_u64(bits: &[u8]) -> Result<u64> {
    if bits.len() > 64 {
        return Err(anyhow!("Unexpected bits len: {}", bits.len()));
    }

    bits.iter()
        .map(|bit| *bit as u64)
        .reduce(|acc, bit| (acc << 1) + bit)
        .ok_or(anyhow!("Cannot collapse bits to u64"))
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    #[test]
    fn example_1a() -> Result<()> {
        let input = "8A004A801A8002F478";

        assert_eq!(part_1(trim_input(input))?, 16);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        let input = "620080001611562C8802118E34";

        assert_eq!(part_1(trim_input(input))?, 12);

        Ok(())
    }

    #[test]
    fn example_1c() -> Result<()> {
        let input = "C0015000016115A2E0802F182340";

        assert_eq!(part_1(trim_input(input))?, 23);

        Ok(())
    }

    #[test]
    fn example_1d() -> Result<()> {
        let input = "A0016C880162017C3686B18A3D4780";

        assert_eq!(part_1(trim_input(input))?, 31);

        Ok(())
    }

    #[test]
    fn example_2a() -> Result<()> {
        let input = "C200B40A82";

        assert_eq!(part_2(trim_input(input))?, 3);

        Ok(())
    }

    #[test]
    fn example_2b() -> Result<()> {
        let input = "04005AC33890";

        assert_eq!(part_2(trim_input(input))?, 54);

        Ok(())
    }

    #[test]
    fn example_2c() -> Result<()> {
        let input = "880086C3E88112";

        assert_eq!(part_2(trim_input(input))?, 7);

        Ok(())
    }

    #[test]
    fn example_2d() -> Result<()> {
        let input = "CE00C43D881120";

        assert_eq!(part_2(trim_input(input))?, 9);

        Ok(())
    }

    #[test]
    fn example_2e() -> Result<()> {
        let input = "D8005AC2A8F0";

        assert_eq!(part_2(trim_input(input))?, 1);

        Ok(())
    }

    #[test]
    fn example_2f() -> Result<()> {
        let input = "F600BC2D8F";

        assert_eq!(part_2(trim_input(input))?, 0);

        Ok(())
    }

    #[test]
    fn example_2g() -> Result<()> {
        let input = "9C005AC2F8F0";

        assert_eq!(part_2(trim_input(input))?, 0);

        Ok(())
    }

    #[test]
    fn example_2h() -> Result<()> {
        let input = "9C0141080250320F1802104A08";

        assert_eq!(part_2(trim_input(input))?, 1);

        Ok(())
    }
}
