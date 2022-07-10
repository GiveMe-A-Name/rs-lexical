use crate::{const_variable::DFAStateConst, utils::judge_token_type};

#[derive(Debug)]
pub struct Path {
    pub state: DFAStateConst,
    pub ch: char,
    pub next_state: DFAStateConst,
    pub is_match: bool,
    pub end: bool,
}

#[derive(Debug)]

pub struct Token {
    pub _type: String,
    pub value: String,
}

#[derive(Default)]
pub struct DFA {
    pub state: DFAStateConst,
    paths: Vec<Path>,
    matches: Vec<char>,
    pub tokens: Vec<Token>,
}

impl DFA {
    pub fn reset(&mut self) {
        self.state = DFAStateConst::default();
        self.paths.clear();
        self.matches.clear();
        self.tokens.clear();
    }

    pub fn next_state(&self, ch: char, state: DFAStateConst) -> DFAStateConst {
        match state {
            DFAStateConst::SReset => {
                if ch.is_alphabetic() {
                    return DFAStateConst::SIdentifier;
                }
                if ch.is_ascii_digit() {
                    return DFAStateConst::SNumber;
                }
                if ch.is_whitespace() {
                    return DFAStateConst::SWhitespace;
                }
                if ch == ';' || ch == '=' {
                    return DFAStateConst::SSymbol;
                }
            }
            DFAStateConst::SIdentifier => {
                if ch.is_ascii_digit() || ch.is_alphabetic() {
                    return DFAStateConst::SIdentifier;
                }
            }
            DFAStateConst::SNumber => {
                if ch.is_ascii_digit() {
                    return DFAStateConst::SNumber;
                }
            }
            _ => (),
        }
        DFAStateConst::SReset
    }

    pub fn path_grow(&mut self, path: Path) {
        self.paths.push(path);
    }

    pub fn flow_to_next(&mut self, ch: char, next_state: DFAStateConst) {
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
        self.state = DFAStateConst::default();
    }
}
