// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.


/// ------------------------------[COMMENTAIRES]------------------------------
/// En Rust, les constantes (mot-clé : CONST) doivent avoir un type
/// explicitement spécifié. Le cas échéant, le compilateur retournera une
/// erreur car il ne pourra pas déterminer le type.
/// Dans cet exercice, le type à spécifier est un nombre entier "i8" qui permet
/// de stocker la valeur 3.
/// --------------------------------------------------------------------------
const NUMBER :i8 = 3; 
fn main() {
    println!("Number {}", NUMBER);
}
