use lexical::isr::ISR;

mod const_variable;
mod lexical;
mod utils;

fn main() {
    let mut isr = ISR::default();
    let tokens = isr.scanner("const a = 10; let b = 20;".into());
    println!("{:#?}", tokens);
}
