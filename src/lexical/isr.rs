use crate::const_variable::DFAStateConst;

use super::dfa::{Path, Token, DFA};

#[derive(Default)]
pub struct ISR {
    stream: String,
    length: usize,
    cursor: usize,
    dfa: DFA,
}

// implement tool methods
impl ISR {
    fn next_char(&self) -> Option<char> {
        self.stream.chars().nth(self.cursor)
    }
    fn is_last_char(&self) -> bool {
        self.cursor == self.length - 1
    }
    fn move_cursor(&mut self) {
        self.cursor += 1
    }
}

impl ISR {
    fn initial(&mut self, stream: String) {
        self.stream = stream.trim().into();
        self.length = self.stream.len();
        self.cursor = 0;
        self.dfa.reset();
    }

    pub fn scanner(&mut self, stream: String) -> &Vec<Token> {
        self.initial(stream);
        self.read();
        &self.dfa.tokens
    }

    fn read(&mut self) {
        while let Some(ch) = self.next_char() {
            let mut end = false;
            let state = self.dfa.state;
            let next_state = self.dfa.next_state(ch, state);
            let is_match = if next_state != DFAStateConst::SReset {
                if self.is_last_char() {
                    end = true;
                }
                true
            } else {
                false
            };
            let path = Path {
                state,
                ch,
                next_state,
                is_match,
                end,
            };
            println!("{:?}", path);
            self.dfa.path_grow(path);
            if is_match {
                self.move_cursor();
                self.dfa.flow_to_next(ch, next_state);
                if end {
                    self.dfa.product_token();
                    self.dfa.clear();
                }
            } else {
                self.dfa.product_token();
                self.dfa.clear();
            }
        }
    }
}
