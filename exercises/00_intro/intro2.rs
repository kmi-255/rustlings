// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.

/// ------------------------------[COMMENTAIRES]------------------------------
/// printline n'est pas une macro valide dans Rust. 
/// Il faut utiliser par example :
/// println!() qui permet d'afficher un élément en sortie 
/// (et un saut de ligne, qui est propre à println!())
/// Dans cet exercice, la macro printline est à remplacer et j'ai opté pour
/// println qui est une macro figurant dans la crate native std 
/// ---------------------------------------------------------------------------
fn main() {
    println!("Hello there!")
}
