mod models;

use models::person::Person;

fn test_person() {
    let mut person = Person::new(
        "John Landis".to_string(),
        20,
        'M',
        "20 rue des champs de patates".to_string(),
        "Paris".to_string(),
    );

    let person2 = Person::new(
        "Julietta Rae".to_string(),
        50,
        'F',
        "20 rue des champs de patates".to_string(),
        "Paris".to_string(),
    );

    // affiche les infos de la personne
    println!("{}", person.afficher_details());

    // incremente l'age de la personne
    person.incremente_age();

    // changer le nom de la personne
    person.changer_nom("Chuck Norris".to_string());

    // vérifie si la personne est plus age que la personne2
    person.is_older_than(&person2);
}


fn main() {

    println!("**----------------------------Test for Person Struct-------------------------------**");
    println!("**---------------------------------------------------------------------------------**");
    println!("**---------------------------------------------------------------------------------**");

    test_person();
}
