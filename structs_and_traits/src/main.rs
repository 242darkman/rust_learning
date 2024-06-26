mod models;
mod traits;

use models::boat::Boat;
use models::person::Person;
use models::pet::Pet;
use models::student::Student;
use models::car::Car;
use traits::vehicule::Vehicle as _;
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

    // Création d'un animal de compagnie
    let pet_id = Uuid::new_v4();
    let mut pet = Pet::new(
        pet_id,
        "Milou".to_string(),
        5,
        true,
    );
    println!("L'animal crée est: {:?} \n", pet);

    // Modification de l'animal de compagnie
    pet.set_name("Max".to_string());
    pet.set_age(6);
    pet.set_vaccinated(false);

    println!("L'animal modifié est: {:?} \n", pet);

    // Vérification si l'animal est autorisé
    if pet.allow_pet() {
        println!("{} est autorisé", pet.get_name());
    } else {
        println!("{} n'est pas autorisé\n", pet.get_name());
    }


    println!("**-----------------------------------------------------------**");
    let mut car = Car {
        model: "BMW".to_string(),
        color: "blue".to_string(),
        is_on_motion: false,
        current_speed: 0,
    };

    let mut my_boat = Boat {
        name: "Trucker".to_string(),
        is_on_motion: false,
        current_speed: 0,
    };

    my_boat.start();
    car.start();
    my_boat.stop();
    car.stop();
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
