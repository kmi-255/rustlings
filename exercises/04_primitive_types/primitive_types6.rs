// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

/// ------------------------------[COMMENTAIRES]------------------------------
/// Dans cet exercice, il est demandé d'accéder au deuxième élément figurant
/// dans le tuple `numbers`. Pour ce faire, une variable `second` est déclarée
/// dans laquelle est appelé le tuple `numbers` suivi du nombre `1`
/// L'index débute toujours à 0 donc pour accéder au deuxième élément du tuple,
/// le nombre 1 est utilisé.
/// ---------------------------------------------------------------------------
#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1; // Michaël : pour accéder au deuxième nombre dans le tuple, on assigne la variable "second" avec l'index 1 du tuple "numbers"

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
