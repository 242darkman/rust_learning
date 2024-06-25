fn main() {
    // - Créez un tuple hébergeant 3 i32
    // - Ajoutez une première instruction allant chercher les 2 premiers i32 et les additionnant (en affichant le résultat dans la console
    // - Ecrivez une autre instruction qui va prendre les 2e et 3e i32 et les soustraire
    // - Utilisez soit l’accès direct via l’opérateur . ou bien la déstructuration

    let tuple1 = (0, 0, 0);

    let sum = tuple1.0 + tuple1.1;
    println!("{sum}");

    let (first_number, second_number, third_number) = tuple1;
    let res = second_number - third_number;
    println!("{res}");
}
