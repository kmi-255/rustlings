// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)



// Put your function here!
fn calculate_price_of_apples(amount: i32) -> i32 { // La fonction attendue est celle de calculer le nombre de pommes
    // La fonction attend un nombre entier faisant référence au nombre de pomme en entrée,
    // Et retourne un nombre entier en sortie faisant référence au prix.
    if (amount > 40) { // Si le nombre de pommes est supérieur à 40, alors
        amount } // le prix de la pomme = 1 
    else { // Sinon
        (amount * 2) // Il suffit de multiplier par 2 le nombre de pommes pour obtenir le prix initial
    }

}




// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
