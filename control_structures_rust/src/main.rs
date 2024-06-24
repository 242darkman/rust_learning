// Fonction pour imprimer les nombres de 1 à 100 sauf les multiples de 5
fn imprimer_1_a_100_sauf_multiples_de_5() {
    let mut i = 1; // Initialisation du compteur à 1

    loop {
        // Sortir de la boucle si le compteur dépasse 100
        if i > 100 {
            break;
        }

        // Ignorer les multiples de 5
        if i % 5 != 0 {
            // Imprimer le nombre
            println!("{}", i);
        }

        // Incrémenter le compteur
        i += 1;
    }
}

// Fonction pour calculer la somme des nombres de 1 à 100
fn somme_de_1_a_100() -> i32 {
    let mut somme = 0; // Initialisation de la somme à 0
    let mut i = 1;     // Initialisation du compteur à 1

    // Boucle while pour parcourir les nombres de 1 à 100
    while i <= 100 {
        somme += i; // Ajouter le nombre courant à la somme
        i += 1;     // Incrémenter le compteur
    }

    somme // Retourner la somme calculée
}

// Fonction pour imprimer les nombres pairs de 1 à 20
fn imprimer_nombres_pairs_de_1_a_20() {
    // Boucle for avec un range pour parcourir les nombres de 1 à 20
    for i in 1..=20 {
        // Vérifier si le nombre est pair
        if i % 2 == 0 {
            // Imprimer le nombre pair
            println!("{}", i);
        }
    }
}

fn main() {
    imprimer_1_a_100_sauf_multiples_de_5();
    let somme = somme_de_1_a_100();
    println!("La somme des nombres de 1 à 100 est : {}", somme);
    imprimer_nombres_pairs_de_1_a_20();
}
