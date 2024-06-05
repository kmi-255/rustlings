// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.
// 

/// ------------------------------[COMMENTAIRES]------------------------------
/// Dans cet exercice, il est nécessaire d'importer le module `time`.
/// Puisque seulement les macros `SystemTime` et `UNIX_EPOCH` sont utilisées,
/// uniquement celles-ci sont importées. 
/// Le cas échéant, et si d'autres macros sont importées, un avertissement
/// est levé par le compilateur indiquant que des macros importées ne sont pas
/// utilisées. 
/// La bonne pratique étant d'importer uniquement les modules et les macros
/// qui sont utilisés.
/// ---------------------------------------------------------------------------
// TODO: Complete this use statement
use std::time::{SystemTime,UNIX_EPOCH}; 

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
