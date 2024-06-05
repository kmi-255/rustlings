// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



/// ------------------------------[COMMENTAIRES]------------------------------
/// Dans cet exerice, la méthode `iter_mut` permet de rendre le vecteur
/// `mutable`. Il est donc possible d'itérer sur chaque valeur pour la
/// modifier.
/// 
/// L'utilisation de `*element *=2` permet d'utiliser le pointeur lié à chaque
/// `element` du vecteur pour modifier la valeur associée et ainsi
/// la multiplier par 2.
/// 
/// Dans la fonction `vec_map`, et au même titre que la fonction `vec_loop`,
/// le pointeur `*element` est utilisé pour multiplier par 2 la valeur associée
/// du vecteur. Cette modification n'est pas refletée par l'abscence du "="
/// ---------------------------------------------------------------------------

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2; 
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        *element *2 
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
