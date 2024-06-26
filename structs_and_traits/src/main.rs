mod models;

use models::person::Person;
use models::student::Student;
use uuid::Uuid;

fn last_test() {
    // Création d'un étudiant
    let student_id = Uuid::new_v4();
    let student = Student::new(
        student_id,
        "John Landis".to_string(),
        "johnlandis@betterave.com".to_string(),
        "0123456789".to_string(),
        "20 rue Simone Veil, 69200 Venissieux".to_string(),
        "2023-04-01T00:00:00Z".to_string(),
        "2023-04-01T00:00:00Z".to_string(),
    );
    println!("L'étudiant crée est : {:?} \n", student);
}

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
    last_test();

    println!("**----------------------------Test for Person Struct-------------------------------**");
    println!("**---------------------------------------------------------------------------------**");
    println!("**---------------------------------------------------------------------------------**");

    test_person();
}
