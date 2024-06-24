

/// Fonction pour calculer la moyenne de 4 valeurs
fn calculate_average(val1: i32, val2: f64, val3: i32, val4: f64) -> f64 {
    let sum: f64 = val1 as f64 + val2 + val3 as f64 + val4;
    let count = 4.0;

    if count != 0.0 {
        sum / count
    } else {
        panic!("Erreur: Division par zéro");
    }
}

fn main() {
    // Déclaration des variables
    let a: i32 = 10;
    let b: f64 = 20.5;
    let c: i32 = -30;
    let d: f64 = 40.75;

    // Appel de la fonction pour calculer la moyenne
    let average = calculate_average(a, b, c, d);

    // Affichage du résultat
    println!("La moyenne est: {}", average);
}
