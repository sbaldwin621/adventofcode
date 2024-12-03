pub enum SnailfishNumber {
    Constant(usize),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>)
}