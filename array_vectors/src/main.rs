use std::collections::HashMap;
use std::io;

enum CommandOperation {
    AddItem,
    PrintItems,
    PrintFiveFirstItems,
    ChangeItemStatus,
    DeleteItem,
    Quit,
}

/// Cette méthode permet d'ajouter un article dans le panier
fn add_item_to_shopping_cart(shopping_cart: &mut HashMap<String, bool>, name: &str){
    shopping_cart.insert(name.to_string(), false);
    println!("✅ L'article {} a bien été ajouté à votre liste", name);
}

fn print_shopping_cart_status(item: &str, is_purchased: bool) {
    let status = if is_purchased { "Acheté" } else { "Pas encore acheté" };
    println!("{:<30} | {:<30}", item, status);
}

/// cette méthode affiche les articles dans le panier
fn print_items(shopping_cart: &HashMap<String, bool>, show_only_five_first_items: bool){
    let count_item = 5;

    if shopping_cart.len() == 0 {
        println!("Il n'y a aucun article dans votre liste");
        return;
    }

    println!("✨ Voici la liste de vos articles : \n");
    if show_only_five_first_items {
        println!("{:<30} | {:<30}", "Nom de l'article", "Statut de l'achat");
        println!("{:-<61}", ""); // Affiche une ligne de séparation

        for (key, value) in shopping_cart.iter().take(count_item) {
            print_shopping_cart_status(&key, *value);
            //println!("{} -> {}", key, value);
        }
    } else {
        println!("{:<30} | {:<30}", "Nom de l'article", "Statut de l'achat");
        println!("{:-<61}", ""); // Affiche une ligne de séparation

        for (key, value) in shopping_cart.iter() {
            print_shopping_cart_status(&key, *value);
            //println!("{} -> {}", key, value);
        }
    }
}


/// cette méthode permet de changer le statut d'un article
fn change_item_status(shopping_cart: &mut HashMap<String, bool>, name: &str){
    if let Some(status) = shopping_cart.get_mut(name) {
        *status = !*status;
    }
    println!("✅ L'article {} a bien été modifié", name);
}

/// cette méthode permet de supprimer un article dans notre liste
fn delete_item_into_shopping_cart(shopping_cart: &mut HashMap<String, bool>, name: &str){
    shopping_cart.remove(name);
    println!("❌ L'article {} a bien été supprimé de votre liste", name);
}

/// cette méthode permet d'afficher les options du magasin que l'utilisateur peut choisir
fn print_choosing_action() {
    println!("\n Que voulez-vous effectuer ? Entrez un chiffre pour la selection \n");
    println!("- \"1\" pour ajouter un article \n");
    println!("- \"2\" afficher tous les articles \n");
    println!("- \"3\" pour afficher les 5 premiers articles \n");
    println!("- \"4\" pour changer le statut d'un article \n");
    println!("- \"5\" pour supprimer un article \n");
    println!("- \"q\" Pour quitter \n");
}

/// cette méthode permet de récupérer le choix de l'utilisateur
fn get_buyer_choice() -> CommandOperation {
    let stdin = io::stdin();
    print_choosing_action();
    let mut operation_choice = String::new();

    loop {
        operation_choice.clear();
        stdin.read_line(&mut operation_choice).expect("Erreur lors de la lecture de la ligne");

        match operation_choice.trim() {
            "1" => {
                println!("Super ! Vous voulez ajouter un article");
                return CommandOperation::AddItem;
            },
            "2" => {
                println!("Nous allons vous montrer tous vos articles");
                return CommandOperation::PrintItems;
            },
            "3" => {
                println!("Nous allons vous montrer vos 5 premiers articles");
                return CommandOperation::PrintFiveFirstItems;
            },
            "4" => {
                println!("Super ! Vous allez pouvoir changer le statut de l'article de votre choix");
                return CommandOperation::ChangeItemStatus;
            },
            "5" => {
                println!("Nous sommes triste de avoir que vous voulez supprimer un article");
                return CommandOperation::DeleteItem;
            },
            "q" => {
                println!("Vous avez choisi de quitter le magasin ⛔.");
                return CommandOperation::Quit;
            },
            _ => println!("Le choix \" {} \" est invalide. Faites un choix valide.", operation_choice.trim()),
        }
    }
}

/// cette méthode réalise les actions de l'utilisateur selon de son choix d'opération
fn perform_operation(shopping_cart: &mut HashMap<String, bool>, operation: CommandOperation) {
    match operation {
        CommandOperation::AddItem => {
            println!("Entrez le nom de l'article à ajouter :");
            let name = read_user_input();
            add_item_to_shopping_cart(shopping_cart, &name);
        },
        CommandOperation::PrintItems => print_items(shopping_cart, false),
        CommandOperation::PrintFiveFirstItems => print_items(shopping_cart, true),
        CommandOperation::ChangeItemStatus => {
            println!("Entrez le nom de l'article dont vous voulez changer le statut :");
            let name = read_user_input();
            change_item_status(shopping_cart, &name);
        },
        CommandOperation::DeleteItem => {
            println!("Entrez le nom de l'article à supprimer :");
            let name = read_user_input();
            delete_item_into_shopping_cart(shopping_cart, &name);
        },
        CommandOperation::Quit => println!("Vous avez choisi de quitter le programme. \n"),
    }
}

/// cette méthode permet de lire une entrée de l'utilisateur sur terminal
fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");
    input.trim().to_string()
}

fn main() {
    // Création du panier
    let mut shopping_cart: HashMap<String, bool> = HashMap::new();

    loop {
        let operation = get_buyer_choice();
        if let CommandOperation::Quit = operation {
            break;
        }
        perform_operation(&mut shopping_cart, operation);
    }
}


