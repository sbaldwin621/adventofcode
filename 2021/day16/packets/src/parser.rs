use std::cmp::min;

use nom::{
    IResult,
    bits,
    bits::complete::{tag, take},
    combinator::map,
    multi::{many0, many1, many_m_n},
    sequence::{pair, tuple}, branch::alt
};

use crate::bits::{Packet, PacketContents};

pub fn parse_packet(i: &[u8]) -> IResult<&[u8], Packet> {
    bits(parse_packet_bits)(i)
}

fn parse_packet_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (i, version) = take_3_bits(i)?;
    let (i, contents) = packet_contents(i)?;
    Ok((i, Packet::new(version, contents)))
}

fn packet_contents(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketContents> {
    alt((literal, operator))(i)
}

fn literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketContents> {
    // ID of "4"
    let (i, _) = tag(0b100, 3usize)(i)?;
    let (i, (values, terminating_value)) = pair(many0(literal_group), literal_group_terminator)(i)?;

    let mut result: u32 = 0;
    for value in values {
        result = result << 4;
        result |= value as u32;
    }

    result = result << 4;
    result |= terminating_value as u32;
    
    Ok((i, PacketContents::Literal(result.into())))
}

fn literal_group(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    let (i, _) = tag(0b1, 1usize)(i)?;
    let (i, value) = take(4usize)(i)?;
    
    Ok((i, value))
}

fn literal_group_terminator(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    let (i, _) = tag(0b0, 1usize)(i)?;
    let (i, value) = take(4usize)(i)?;

    Ok((i, value))
}

fn operator(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketContents> {
    let (i, id) = take_3_bits(i)?;
    let (i, subpackets) = alt((
        operator_zero, operator_one
    ))(i)?;


    Ok((i, PacketContents::Operator(id, subpackets)))
}

fn operator_zero(i: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    let (i, _) = tag(0b0, 1usize)(i)?;
    let (i, length) = zero_mode_length(i)?;

    let mut sub_bits = vec![];
    let mut i = i;

    let mut length = length;
    while length > 0 {
        let n = min(length, 8);
        let (i2, byte) = take_n_bits_as_u8(n, i)?;
        i = i2;

        let byte = byte << (8 - n);

        sub_bits.push(byte);

        length = length.saturating_sub(8);
    }

    let slice = &sub_bits[..];
    let (_, packets) = many1(parse_packet_bits)((slice, 0)).unwrap();

    Ok((i, packets))
}

fn zero_mode_length(i: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    take(15usize)(i)
}

fn operator_one(i: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    let (i, _) = tag(0b1, 1usize)(i)?;
    let (i, length) = one_mode_length(i)?;

    println!("length {}", length);

    let (i, packets) = many_m_n(length, length, parse_packet_bits)(i)?;

    Ok((i, packets))
}

fn one_mode_length(i: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    take(11usize)(i)
}

fn take_3_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(i)
}

fn take_n_bits_as_u8(n: usize, i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(n)(i)
}
