mod models;
mod enums;
mod services;

use std::io::{stdin, stdout, Write};

use services::employee_management::EmployeeManagement;

fn main() {
    let mut employee_management = EmployeeManagement::new();

    loop {
        println!("Menu:");
        println!("1. Ajouter un employé");
        println!("2. Afficher les employés");
        println!("3. Mettre à jour un employé");
        println!("4. Licencier un employé");
        println!("q. Quitter");
        print!("Choisissez une option: ");
        stdout().flush().unwrap();

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => employee_management.add_employee(),
            "2" => employee_management.show_employees(),
            "3" => employee_management.update_employee(),
            "4" => employee_management.fire_employee(),
            "q" => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}
