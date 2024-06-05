// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.



/// ------------------------------[COMMENTAIRES]------------------------------
/// Dans cet exercice, il a fallu construire la fonction fn bigger() à l'aide
/// d'instructions IF.
/// 
/// Ainsi, si a est plus petit que b alors la macro println!() affiche la
/// valeur de b et renvoie b qui est le nombre entier attendu par la fonction.
/// 
/// Si a est plus grand que b, alors la macro println!() affichage la valeur
/// de a et renvoie a qui est le nombre entier attendu par la fonction.
/// 
/// Enfin, si les deux instructions précédentes ne voient pas leur condition
/// remplies, alors on détermine que a est égal à b et qu'il est logique de
/// renvoyer une des deux valeurs car elles sont identiques.
/// ---------------------------------------------------------------------------
pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a < b {
        println!("b was bigger than a {}", b);
        return b;
    }
    if a > b {
        println!("a was bigger than b {}", a);
        return a;
    }
    else {
        println!("a and b are equal {} and {}", a ,b);
        return a;
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
