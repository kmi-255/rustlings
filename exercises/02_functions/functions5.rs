// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



/// ------------------------------[COMMENTAIRES]------------------------------
/// Dans cet exercice, l'ajout de "{:?}" est utilisé dans la macro println!().
/// Ce qui permet de déclencher l'utilisation de std::fmt::Debug, et ainsi
/// forcer la sortie (output). Le cas échéant, la directive {...} ne permet pas
/// toujours d'afficher la sortie (output).
/// 
/// Dans la fonction fn square(), l'argument passé est déjà un nombre entier.
/// Il n'est donc pas utile d'explicitement retourner un nombre entier.
/// ---------------------------------------------------------------------------
fn main() {
    let answer = square(3);
    println!("The square of 3 is {:?}", answer);
}

fn square(num: i32) { 
    num * num;
}
