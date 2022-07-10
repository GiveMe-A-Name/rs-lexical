use crate::const_variable::DFAStateConst;

pub fn judge_token_type(state: DFAStateConst) -> String {
    match state {
        DFAStateConst::SIdentifier => "Identifier",
        DFAStateConst::SWhitespace => "Whitespace",
        DFAStateConst::SNumber => "Number",
        DFAStateConst::SSymbol => "Symbol",
        _ => "Unknown", // DFAStateConst::SReset => todo!(),
    }
    .to_owned()
}
