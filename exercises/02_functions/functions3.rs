// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(89898); // Michaël : la fonction attend à ce qu'un argument soit donné. Un nombre entier non-signé est donné et qui rentre sur 32 bits.
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
