use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    string_expressions: Vec<StringExpression>
}

impl PuzzleInput {
    pub fn difference(&self) -> usize {
        self.string_expressions.iter().map(|s| s.difference()).sum()
    }

    pub fn reencoded_difference(&self) -> usize {
        self.string_expressions.iter().map(|s| s.reencoded_difference()).sum()
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string_expressions = vec![];
        for line in s.lines() {
            let expression = line.parse::<StringExpression>()?;
            string_expressions.push(expression);
        }

        Ok(PuzzleInput { string_expressions })
    }
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("couldn't parse string expression: {0}")]
    ParseStringExpressionError(#[from] ParseStringExpressionError)
}

#[derive(Debug)]
pub struct StringExpression {
    text: String,
    actual: String
}

impl StringExpression {
    pub fn difference(&self) -> usize {
        self.text.len() - self.actual.len()
    }

    pub fn reencode(&self) -> String {
        let mut reencoded = String::new();

        reencoded.push('"');

        for c in self.text.chars() {
            match c {
                '"' => {
                    reencoded.push_str("\\\"");
                },
                '\\' => {
                    reencoded.push_str("\\\\")
                },
                _ => {
                    reencoded.push(c);
                }
            }
        }
        
        reencoded.push('"');

        reencoded
    }

    pub fn reencoded_difference(&self) -> usize {
        self.reencode().len() - self.text.len()
    }
}

impl FromStr for StringExpression {
    type Err = ParseStringExpressionError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.to_owned();
        let mut actual = String::new();
        
        let mut iter = s.chars();
    
        let mut parser_state = StringExpressionParserState::Start;
        loop {
            parser_state = match parser_state {
                StringExpressionParserState::Start => {
                    match iter.next() {
                        Some('"') => StringExpressionParserState::Contents,
                        Some(_) => { return Err(ParseStringExpressionError::UnexpectedCharacter); },
                        None => { return Err(ParseStringExpressionError::UnexpectedEndOfLine); }
                    }
                },
                StringExpressionParserState::Contents => {
                    match iter.next() {
                        Some('\\') => StringExpressionParserState::EscapeSequence,
                        Some('"') => StringExpressionParserState::EndOfLine,
                        Some(c) => {
                            actual.push(c);
                            StringExpressionParserState::Contents
                        },
                        None => {
                            return Err(ParseStringExpressionError::UnexpectedEndOfLine);
                        }
                    }
                },
                StringExpressionParserState::EscapeSequence => {
                    match iter.next() {
                        Some(c) if c == '\\' || c == '"' => {
                            actual.push(c);
                            StringExpressionParserState::Contents
                        },
                        Some('x') => {
                            match (iter.next(), iter.next()) {
                                (Some(a), Some(b)) => {
                                    if let Some(byte) = hex_pair_to_byte(a, b) {
                                        actual.push(byte as char);
                                        StringExpressionParserState::Contents
                                    } else {
                                        return Err(ParseStringExpressionError::InvalidHexCode);
                                    }                                    
                                },
                                (_, _) => {
                                    return Err(ParseStringExpressionError::InvalidHexCode);
                                }
                            }
                        },
                        Some(_) => {
                            return Err(ParseStringExpressionError::UnexpectedCharacter);
                        },
                        None => {
                            return Err(ParseStringExpressionError::UnexpectedEndOfLine);
                        }
                    }
                },
                StringExpressionParserState::EndOfLine => {
                    match iter.next() {
                        None => { 
                            return Ok(StringExpression { text, actual })
                        },
                        _ => {
                            return Err(ParseStringExpressionError::UnexpectedCharacter);
                        }
                    }
                }
            }
        }
    }
}

fn hex_to_byte(c: char) -> Option<u8> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' => Some(10),
        'b' => Some(11),
        'c' => Some(12),
        'd' => Some(13),
        'e' => Some(14),
        'f' => Some(15),
        _ => None
    }
}

fn hex_pair_to_byte(a: char, b: char) -> Option<u8> {
    let high = hex_to_byte(a)?;
    let low = hex_to_byte(b)?;

    Some(high << 1 | low)
}

enum StringExpressionParserState {
    Start,
    Contents,
    EscapeSequence,
    EndOfLine
}

#[derive(Error, Debug)]
pub enum ParseStringExpressionError {
    #[error("invalid hex code")]
    InvalidHexCode,
    #[error("unexpected character")]
    UnexpectedCharacter,
    #[error("unexpected end of line")]
    UnexpectedEndOfLine
}


#[cfg(test)]
mod tests {
    use super::*;

    fn case(s: &str) -> String {
        s.parse::<StringExpression>().unwrap().reencode()
    }

    #[test]
    pub fn reencode() {
        assert_eq!(case(r#""""#), r#""\"\"""#);
        assert_eq!(case(r#""abc""#), r#""\"abc\"""#);
        assert_eq!(case(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(case(r#""\x27""#), r#""\"\\x27\"""#);
    }
}
