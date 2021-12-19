use nom::{
    IResult,
    bits,
    bits::complete::{tag, take},
    combinator::map,
    multi::{many0},
    sequence::{pair, tuple}
};

use crate::bits::{Packet, PacketContents};

pub fn parse_packet(i: &[u8]) -> IResult<&[u8], Packet> {
    bits(parse_packet_bits)(i)
}

fn parse_packet_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (i, version) = take_3_bits(i)?;
    let (i, contents) = literal(i)?;
    Ok((i, Packet::new(version, contents)))
}

fn literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketContents> {
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

fn take_3_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(i)
}

fn check_tag(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    tag(0x01, 1usize)(i)
}
