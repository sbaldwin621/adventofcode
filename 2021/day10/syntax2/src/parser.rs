use std::net::Incoming;

#[derive(PartialEq, Eq, Debug)]
pub enum ParserResult {
    Legal,
    Incomplete(ParserStateStack),
    IllegalChar(char)
}

impl ParserResult {
    pub fn score(&self) -> u64 {
        match self {
            ParserResult::Incomplete(stack) => {
                let mut score = 0;
                let mut stack = stack.clone();
                loop {
                    if let ParserState::ExpectingChar(expected) = stack.current_state {
                        let character_score = match expected {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0
                        };

                        if character_score > 0 {
                            score = score * 5 + character_score;
                        }
                    }
                    
                    if stack.pop().is_none() {
                        break;
                    }
                }

                score
            },
            _ => 0
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParserState {
    ExpectingAnything,
    ExpectingChar(char)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParserStateStack {
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

    pub fn pop(&mut self) -> Option<ParserState> {
        let popped_state = self.state_stack.pop();
        if let Some(popped_state) = popped_state {
            self.current_state = popped_state;
        } else {
            self.current_state = ParserState::ExpectingAnything;
        }
        
        popped_state
    }
}

impl FromIterator<ParserState> for ParserStateStack {
    fn from_iter<T: IntoIterator<Item = ParserState>>(iter: T) -> Self {
        let mut result = ParserStateStack::new();

        for state in iter {
            result.push(state);
        }

        result
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
        ParserResult::Incomplete(state)
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
        assert_eq!(ParserResult::Incomplete(ParserStateStack::from_iter(vec![ParserState::ExpectingChar(')')].into_iter())), parse("(".to_string()));
        assert_eq!(ParserResult::Incomplete(ParserStateStack::from_iter(vec![
            ParserState::ExpectingChar('}'),
            ParserState::ExpectingChar(']')
        ].into_iter())), parse("{()[()()".to_string()));
    }

    #[test]
    fn illegal() {
        assert_eq!(ParserResult::IllegalChar(']'), parse("(]".to_string()));
        assert_eq!(ParserResult::IllegalChar('>'), parse("{()()()>".to_string()));
        assert_eq!(ParserResult::IllegalChar('}'), parse("(((()))}".to_string()));
        assert_eq!(ParserResult::IllegalChar(')'), parse("<([]){()}[{}])".to_string()));
    }
}