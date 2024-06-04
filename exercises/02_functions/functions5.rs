// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {:?}", answer); // Michaël : ajout de "{:?} pour permettre le format-string"
}

fn square(num: i32) { // Michaël : l'argument envoyé est déjà un nombre entier, il n'est donc pas utile d'explicitement retourner un nombre entier. Suppression de "-> i32"
    num * num;
}
