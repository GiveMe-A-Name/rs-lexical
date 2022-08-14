use lexical::ISR;

mod lexical;

fn main() {
    let mut isr = ISR::default();
    let tokens = isr.scanner("const a = 10; let b = 20;".into());
    println!("{:#?}", tokens);
}


