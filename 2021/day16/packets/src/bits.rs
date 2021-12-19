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

   pub fn evaluate(&self) -> usize {
       self.contents.evaluate()
   }
}

#[derive(Debug)]
pub enum PacketContents {
    Literal(usize),
    Operator(u8, Vec<Packet>)
}

impl PacketContents {
    pub fn version_sum(&self) -> usize {
        match self {
            PacketContents::Literal(_) => 0,
            PacketContents::Operator(_, packets) => packets.iter().map(|p| p.version_sum()).sum()
        }
    }

    pub fn evaluate(&self) -> usize {
       match self {
            PacketContents::Literal(value) => (*value) as usize,
            PacketContents::Operator(operator_id, packets) => {
                match operator_id {
                    0 => packets.iter().map(|p| p.evaluate()).sum(),
                    1 => packets.iter().map(|p| p.evaluate()).product(),
                    2 => packets.iter().map(|p| p.evaluate()).min().unwrap(),
                    3 => packets.iter().map(|p| p.evaluate()).max().unwrap(),
                    5 => {
                        let a = packets[0].evaluate();
                        let b = packets[1].evaluate();

                        if a > b {
                            1
                        } else {
                            0
                        }
                    },
                    6 => {
                        let a = packets[0].evaluate();
                        let b = packets[1].evaluate();

                        if a < b {
                            1
                        } else {
                            0
                        }
                    },
                    7 => {
                        let a = packets[0].evaluate();
                        let b = packets[1].evaluate();

                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("unknown operator")
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_packet;

    fn evaluate(s: &str) -> usize {
        parse_packet(s).evaluate()
    }

    #[test]
    fn test() {
        assert_eq!(3, evaluate("C200B40A82"));
        assert_eq!(54, evaluate("04005AC33890"));
        assert_eq!(7, evaluate("880086C3E88112"));
        assert_eq!(9, evaluate("CE00C43D881120"));
        assert_eq!(1, evaluate("D8005AC2A8F0"));
        assert_eq!(0, evaluate("F600BC2D8F"));
        assert_eq!(0, evaluate("9C005AC2F8F0"));
        assert_eq!(1, evaluate("9C0141080250320F1802104A08"));
    }
}