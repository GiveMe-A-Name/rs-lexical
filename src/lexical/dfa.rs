use super::utils::judge_token_type;
use super::DFAState;

#[derive(Debug)]
pub struct Path {
    pub state: DFAState,
    pub ch: char,
    pub next_state: DFAState,
    pub is_match: bool,
    pub end: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub _type: String,
    pub value: String,
}

#[derive(Default)]
pub struct DFA {
    pub state: DFAState,
    paths: Vec<Path>,
    matches: Vec<char>,
    pub tokens: Vec<Token>,
}

impl DFA {
    pub fn reset(&mut self) {
        self.state = DFAState::default();
        self.paths.clear();
        self.matches.clear();
        self.tokens.clear();
    }

    pub fn next_state(&self, ch: char, state: DFAState) -> DFAState {
        use DFAState::*;
        match state {
            Reset => {
                if ch.is_alphabetic() {
                    return Identifier;
                }
                if ch.is_ascii_digit() {
                    return Number;
                }
                if ch.is_whitespace() {
                    return Whitespace;
                }
                if ch == ';' || ch == '=' {
                    return Symbol;
                }
            }
            Identifier => {
                if ch.is_ascii_digit() || ch.is_alphabetic() {
                    return Identifier;
                }
            }
            Number => {
                if ch.is_ascii_digit() {
                    return Number;
                }
            }
            _ => (),
        }
        Reset
    }

    pub fn path_grow(&mut self, path: Path) {
        self.paths.push(path);
    }

    pub fn flow_to_next(&mut self, ch: char, next_state: DFAState) {
        self.matches.push(ch);
        self.state = next_state;
    }

    pub fn product_token(&mut self) {
        if !self.matches.is_empty() {
            // let token: String = self.matches.join("");
            // ?? why Vec<char> can't use join || concat
            let value: String = self.matches.iter().collect();
            let _type = judge_token_type(self.state);
            self.tokens.push(Token { _type, value })
        }
    }

    pub fn clear(&mut self) {
        self.matches.clear();
        self.state = DFAState::default();
    }

    pub fn tokens(&self) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|token| token._type != "Whitespace".to_string())
            .collect::<Vec<_>>()
    }
}
