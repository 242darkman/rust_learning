use std::io;

/// enumération représentant les différentes opérations possibles
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Quit
}

/// Affiche les options de choix d'opération à l'utilisateur.
fn print_choosing_operation() {
    println!("Quel type d'opération voulez-vous effectuer ? Entrez un chiffre pour la selection \n");
    println!("- \"1\" pour l'addition \n");
    println!("- \"2\" pour la soustraction \n");
    println!("- \"3\" pour la multiplication \n");
    println!("- \"4\" pour la division \n");
    println!("- \"q\" Pour quitter \n");
}

/// Cette fonction demande à l'utilisateur de choisir une opération parmi l'addition,
/// la soustraction, la multiplication, la division, ou de quitter.
///
/// # Retour
///
/// `Operation` - Une valeur de l'énumération `Operation` représentant l'opération choisie par l'utilisateur.
///
/// # Erreurs
///
/// Cette fonction peut échouer si la lecture de l'entrée standard échoue.
///
/// # Exemples
///
/// ```rust
/// let operation = get_operation_choice();
/// match operation {
///     Operation::Add => println!("Addition sélectionnée"),
///     Operation::Subtract => println!("Soustraction sélectionnée"),
///     Operation::Multiply => println!("Multiplication sélectionnée"),
///     Operation::Divide => println!("Division sélectionnée"),
///     Operation::Quit => println!("Quitter sélectionné"),
/// }
/// ```
fn get_operation_choice() -> Operation {
    let stdin = io::stdin();
    let mut operation_choice = String::new();

    loop {
        operation_choice.clear();
        print_choosing_operation();
        stdin.read_line(&mut operation_choice).expect("Erreur lors de la lecture de la ligne");

        match operation_choice.trim() {
            "1" => {
                println!("Vous avez choisi l'addition + .");
                return Operation::Add;
            },
            "2" => {
                println!("Vous avez choisi la soustraction - .");
                return Operation::Subtract;
            },
            "3" => {
                println!("Vous avez choisi la multiplication * .");
                return Operation::Multiply;
            },
            "4" => {
                println!("Vous avez choisi la division / .");
                return Operation::Divide;
            },
            "q" => {
                println!("Vous avez choisi de quitter ⛔.");
                return Operation::Quit;
            },
            _ => println!("Le choix \" {} \" est invalide. Faites un choix valide.", operation_choice.trim()),
        }
    }
}


/// Cette fonction demande à l'utilisateur combien de nombres il souhaite entrer,
/// puis lit et renvoie ces nombres dans un vecteur.
///
/// # Retour
///
/// `Vec<f64>` - Un vecteur contenant les nombres entrés par l'utilisateur.
///
/// # Erreurs
///
/// Cette fonction peut échouer si la lecture de l'entrée standard échoue ou si la
/// conversion des entrées en `u8` ou `f64` échoue.
///
/// # Exemples
///
/// ```rust
/// let numbers = get_numbers_for_operation();
/// println!("{:?}", numbers);
/// ```
fn get_numbers_for_operation() -> Vec<f64> {
    let stdin = io::stdin();
    let count_number: u8;

    // on boucle jusqu'à ce que l'utilisateur saisisse un nombre valide (au moins 2)
    loop {
        println!("Combien de nombres voulez-vous saisir pour votre calcul ? (Pour la division, veuillez entrer deux nombres)");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
        match input.trim().parse::<u8>() {
            Ok(num) if num >= 2 => {
                count_number = num;
                break; // on sort de la boucle si le nombre n'est pas valide et >= 2
            },
            _ => println!("⚠️ Veuillez saisir au moins deux nombres pour effectuer une opération \n"),
        }
    }

    let mut numbers = Vec::new();

    // on boucle pour lire chaque nombre saisi par l'utilisateur
    for _ in 1..=count_number {
        println!("Entrer un nombre:");
        loop {
            let mut number = String::new();
            stdin.read_line(&mut number).expect("Erreur lors de la lecture de la ligne");
            if number.trim() != "" {
                match number.trim().parse::<f64>() {
                    Ok(num) => {
                        numbers.push(num);
                        break; // on sort de la boucle interne si le nombre est valide
                    },
                    Err(_) => println!("Veuillez saisir un nombre valide:"),
                }
            } else {
                println!("Veuillez saisir un nombre valide:");
            }
        }
    }

    numbers

}


/// Effectue l'opération choisie sur les nombres fournis.
///
/// # Paramètres
///
/// * `numbers`: Vec<f64> - Un vecteur contenant les nombres sur lesquels effectuer l'opération.
/// * `operation`: Operation - L'opération à effectuer.
///
/// # Retour
///
/// `f64` - Le résultat de l'opération.
fn perform_operation(numbers: Vec<f64>, operation: Operation) {
    match operation {
        Operation::Add => {
            let add_result: f64 = numbers.iter().sum();
            println!("Le resultat de l'operation d'addition de {:?} est : {} \n", numbers, add_result);
        },
        Operation::Multiply => {
            let multiply_result: f64 = numbers.iter().product();
            println!("Le resultat de l'operation de multiplication de {:?} est : {} \n", numbers, multiply_result);
        },
        Operation::Subtract => {
            let mut substract_result = 0.0;
            for  num in numbers.iter() {
                substract_result = substract_result - *num;
            }
            println!("Le resultat de l'operation de soustraction de {:?} est : {} \n", &numbers, substract_result);
        },
        Operation::Divide => {
            if numbers.len() > 2 {
                println!("Le calcul de la division ne peut pas contenir plus de deux nombres. \n");
            }
            let max_value = numbers[0];
            let min_value = numbers[1];
            let divide_result = max_value / min_value;

            println!("Le resultat de l'operation de division de max_value {} et min_value {} est : {} \n", max_value, min_value, divide_result);
        },
        Operation::Quit => {
            println!("Vous avez choisi de quitter le programme. \n");
            return;
        },
    }
}



fn main() {
    let numbers = get_numbers_for_operation();
    println!("Vous avez saisi les nombres suivants : {:?} \n", numbers);

    let operation_choice = get_operation_choice();

    perform_operation(numbers, operation_choice);
}
