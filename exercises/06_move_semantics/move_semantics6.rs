// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



/// ------------------------------[COMMENTAIRES]------------------------------
/// Dans cet exercice, il a été nécessaire d'ajouter une référence partagée
/// au pointeur lié à la valeur que contient `data` pour qu'elle puisse
/// être lue sans en prendre la propriété, mentoinné par l'esperluette `&`.
/// ---------------------------------------------------------------------------

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char { 
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
