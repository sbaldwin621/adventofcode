#[derive(Debug)]
pub struct Packet {
    version: u8,
    contents: PacketContents    
}

impl Packet {
    pub fn new(version: u8, contents: PacketContents) -> Packet {
        Packet { version, contents }
    }
}

#[derive(Debug)]
pub enum PacketContents {
    Literal(u32)
}