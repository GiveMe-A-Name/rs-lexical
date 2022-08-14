#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum DFAState {
    Reset,
    Identifier,
    Whitespace,
    Number,
    Symbol,
}

impl Default for DFAState {
    fn default() -> Self {
        DFAState::Reset
    }
}
