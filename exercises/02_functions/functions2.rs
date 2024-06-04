// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(3);
}

fn call_me(num:i32) { // Michaël : spécifier le type accepté par la fonction pour permettre à cette fonction de s'executer convenablement
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
