#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum DFAStateConst {
    SReset,
    SIdentifier,
    SWhitespace,
    SNumber,
    SSymbol,
}

impl Default for DFAStateConst {
    fn default() -> Self {
        DFAStateConst::SReset
    }
}
