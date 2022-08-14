use super::dfa::{Path, Token, DFA};
use super::DFAState;

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

    pub fn scanner(&mut self, stream: String) -> Vec<&Token> {
        self.initial(stream);
        self.read();
        self.dfa.tokens()
    }

    fn read(&mut self) {
        while let Some(ch) = self.next_char() {
            let mut end = false;
            let state = self.dfa.state;
            let next_state = self.dfa.next_state(ch, state);
            let is_match = if next_state != DFAState::Reset {
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

#[cfg(test)]
mod tests {
    use crate::{lexical::dfa::Token, *};

    fn create_isr() -> ISR {
        ISR::default()
    }

    #[test]
    fn test_create_tokens() {
        let code = "const a = 10;";
        let tokens = [
            &Token {
                _type: "Identifier".to_owned(),
                value: "const".to_owned(),
            },
            &Token {
                _type: "Identifier".to_owned(),
                value: "a".to_owned(),
            },
            &Token {
                _type: "Symbol".to_owned(),
                value: "=".to_owned(),
            },
            &Token {
                _type: "Number".to_owned(),
                value: "10".to_owned(),
            },
            &Token {
                _type: "Symbol".to_owned(),
                value: ";".to_owned(),
            },
        ];
        let mut isr = create_isr();
        let result = isr.scanner(code.into());
        let _tokens: [&Token; 5] = result.try_into().unwrap();
        assert_eq!(_tokens, tokens);
    }
}
