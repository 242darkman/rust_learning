fn main() {
    // Création d'une chaîne mutable (modifiable) avec une capacité initiale de 10 caractères.
    let mut first_name = String::with_capacity(10);
    // Ajout de la chaîne "John" à la variable first_name.
    first_name.push_str("John");

    // Création d'une chaîne littérale et conversion en String pour mutabilité.
    let last_name = "LANDIS";
    let last_name = last_name.to_string();

    // Concaténation des chaînes first_name et last_name en utilisant la macro format!.
    let full_name = format!("{} {}", first_name, last_name);

    // Une autre façon de concaténer les chaînes en utilisant clone et l'opérateur +.
    let full_name = first_name.clone() + " " + &last_name;

    // Ajout de la chaîne last_name à la fin de first_name.
    first_name.push_str(&last_name);

    // Nouvelle concaténation de first_name (après modification) et de last_name.
    let full_name = first_name.clone() + &last_name;

    // Déclaration et initialisation d'un caractère mutable.
    let mut char = 'a';
    // Modification du caractère.
    char = 'b';

    // Tentative d'ajouter un caractère à la chaîne full_name (ce qui ne fonctionne pas, car full_name est une String).
    // full_name.push_str(char); // Cette ligne provoquera une erreur de compilation car push_str attend une &str.

    // Affichage du nom complet.
    println!("full_name: {}", full_name);
}
