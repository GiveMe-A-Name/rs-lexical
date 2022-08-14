use crate::lexical::dfs_state::DFAState;

pub fn judge_token_type(state: DFAState) -> String {
    use DFAState::*;
    match state {
        Identifier => "Identifier",
        Whitespace => "Whitespace",
        Number => "Number",
        Symbol => "Symbol",
        _ => "Unknown", // DFAStateConst::SReset => todo!(),
    }
    .to_owned()
}
