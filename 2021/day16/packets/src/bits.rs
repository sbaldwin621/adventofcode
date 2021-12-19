#[derive(Debug)]
pub struct Packet {
    version: u8,
    contents: PacketContents    
}

impl Packet {
    pub fn new(version: u8, contents: PacketContents) -> Packet {
        Packet { version, contents }
    }

    pub fn version_sum(&self) -> usize {
        let version = self.version as usize;
        version + self.contents.version_sum()
    }
}

#[derive(Debug)]
pub enum PacketContents {
    Literal(u32),
    Operator(u8, Vec<Packet>)
}

impl PacketContents {
    pub fn version_sum(&self) -> usize {
        match self {
            PacketContents::Literal(_) => 0,
            PacketContents::Operator(_, packets) => packets.iter().map(|p| p.version_sum()).sum()
        }
    }
}