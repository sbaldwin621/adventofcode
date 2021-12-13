#[derive(PartialEq, Eq, Debug)]
pub enum ParserResult {
    Legal,
    Incomplete,
    IllegalChar(char)
}

impl ParserResult {
    pub fn score(&self) -> u64 {
        match self {
            ParserResult::Legal => 0,
            ParserResult::Incomplete => 0,
            ParserResult::IllegalChar(char) => match char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ParserState {
    ExpectingAnything,
    ExpectingChar(char)
}

struct ParserStateStack {
    current_state: ParserState,
    state_stack: Vec<ParserState>
}

impl ParserStateStack {
    pub fn new() -> ParserStateStack {
        let current_state = ParserState::ExpectingAnything;
        let state_stack = vec![];

        ParserStateStack { current_state, state_stack }
    }

    pub fn push(&mut self, state: ParserState) {
        self.state_stack.push(self.current_state);
        self.current_state = state;
    }

    pub fn pop(&mut self) {
        self.current_state = self.state_stack.pop().unwrap();
    }
}

pub fn parse(line: String) -> ParserResult {
    let mut state = ParserStateStack::new();
    
    for char in line.chars() {
        match char {
            '(' => {
                state.push(ParserState::ExpectingChar(')'));
            }
            '[' => {
                state.push(ParserState::ExpectingChar(']'));
            }
            '{' => {
                state.push(ParserState::ExpectingChar('}'));
            }
            '<' => {
                state.push(ParserState::ExpectingChar('>'));
            }
            other => {
                if let ParserState::ExpectingChar(expected_char) = state.current_state {
                    if expected_char == other {
                        state.pop();
                    } else {
                        return ParserResult::IllegalChar(other);
                    }
                } else {
                    return ParserResult::IllegalChar(other);
                }
            }
        }
    }

    if state.state_stack.len() > 0 {
        ParserResult::Incomplete
    } else {
        ParserResult::Legal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal() {
        assert_eq!(ParserResult::Legal, parse("()".to_string()));
        assert_eq!(ParserResult::Legal, parse("[]".to_string()));
        assert_eq!(ParserResult::Legal, parse("{()()()}".to_string()));
        assert_eq!(ParserResult::Legal, parse("<([{}])>".to_string()));
        assert_eq!(ParserResult::Legal, parse("[<>({}){}[([])<>]]".to_string()));
        assert_eq!(ParserResult::Legal, parse("(((((((((())))))))))".to_string()));
    }

    #[test]
    fn incomplete() {
        assert_eq!(ParserResult::Incomplete, parse("(".to_string()));
        assert_eq!(ParserResult::Incomplete, parse("{()()()".to_string()));
    }

    #[test]
    fn illegal() {
        assert_eq!(ParserResult::IllegalChar(']'), parse("(]".to_string()));
        assert_eq!(ParserResult::IllegalChar('>'), parse("{()()()>".to_string()));
        assert_eq!(ParserResult::IllegalChar('}'), parse("(((()))}".to_string()));
        assert_eq!(ParserResult::IllegalChar(')'), parse("<([]){()}[{}])".to_string()));
    }
}