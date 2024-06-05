// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.


/// ------------------------------[COMMENTAIRES]--------------------------------
/// Pour cet exercice, la fonction `animal_habitat` a nécéssité des changements:
///     La première instruction `else if` de la variable `identifier` si vraie, 
///     renvoit initialement un float. Alors que `habitat` attend un nombre
///     entier (2). Il a été nécessaire de remplacer le float par un nombre entier.
///     L'instruction `else` de cette même variable, si vraie, renvoit
///     initialement une chaîne de caractère donnant lieu à une erreur de
///     compilation. Il a été nécessaire de mettre un nombre entier (4).
///  
/// ---------------------------------------------------------------------------
pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2 
    } else if animal == "snake" {
        3
    } else {
        4 
        
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
