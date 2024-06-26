use uuid::Uuid;
use std::io;

use crate::{
  enums::employee_status::EmployeeStatus,
  models::employee::Employee
};

pub struct EmployeeManagement {
  employees: Vec<Employee>,
}

/// Gère les opérations sur les employés d'une entreprise
impl EmployeeManagement {
  /// Crée une nouvelle instance de `EmployeeManagement` (constructeur)
  ///
  /// # Retourne
  ///
  /// Une nouvelle instance de `EmployeeManagement`
  pub fn new() -> Self {
    Self {
        employees: Vec::new(),
    }
  }

  /// Ajoute un nouvel employé à la liste.
  ///
  /// Demande le nom de l'employé à l'utilisateur avant de l'ajouter
  pub fn add_employee(&mut self) {
    let stdin = io::stdin();
    let mut name = String::new();
    println!("Entrez le nom de l'employé: ");
    stdin.read_line(&mut name).unwrap();

    let employee = Employee::new(name.trim().to_string(), EmployeeStatus::Active.to_string());
    self.employees.push(employee);
  }

  /// Affiche tous les employés existants
  pub fn show_employees(&self) {
    for employee in &self.employees {
      println!("ID: {}, Nom: {}, Statut: {}", employee.get_id(), employee.get_name(), employee.get_status());
    }
  }

  /// Met à jour les informations d'un employé
  pub fn update_employee(&mut self) {
    let id = EmployeeManagement::prompt_for_employee_id();
    let convert_id_to_uuid = Uuid::parse_str(&id).unwrap();

    if let Some(employee) = self.find_employee_by_id(convert_id_to_uuid) {
      EmployeeManagement::update_employee_name(employee);
      EmployeeManagement::update_employee_status(employee);
    }
  }

  /// Méthode permettant de licencié un employé
  pub fn fire_employee(&mut self) {
    println!("Entrez le nom de l'employé à licencier: ");
    let stdin = io::stdin();
    let mut name = String::new();
    stdin.read_line(&mut name).unwrap();
    let name = name.trim();

    if let Some(index) = self.employees.iter().position(|employee| employee.get_name() == name) {
        let mut fired_employee: &mut Employee = &mut self.employees[index];
        fired_employee.set_status(EmployeeStatus::Fired.to_string());
        self.employees.remove(index);
        println!("Employé licencié.");
    } else {
        println!("Aucun employé trouvé avec ce nom.");
    }
  }

  /// Prompt de demande de l'identifiant d'un employé à mettre à jour
  fn prompt_for_employee_id() -> String {
    println!("Entrez l'ID de l'employé à mettre à jour: ");
    let stdin = io::stdin();
    let mut id = String::new();
    stdin.read_line(&mut id).unwrap();
    id.trim().to_string()
  }

  /// Récupère l'employé dont correspondat à l'identifiant `id`
  fn find_employee_by_id(&mut self, id: Uuid) -> Option<&mut Employee> {
    self.employees.iter_mut().find(|employee| employee.get_id() == id)
  }

  /// Met à jour le nom de l'employé
  fn update_employee_name(employee: &mut Employee) {
    let stdin = io::stdin();
    let mut name = String::new();
    println!("Entrez le nouveau nom de l'employé (laissez vide pour ne pas changer): ");
    stdin.read_line(&mut name).unwrap();

    if !name.trim().is_empty() {
        employee.set_name(name.trim().to_string());
    }
  }

  /// Met à jour le statut de l'employé
  fn update_employee_status(employee: &mut Employee) {
    let stdin = io::stdin();
    let mut status = String::new();
    println!("Entrez le nouveau statut de l'employé (laissez vide pour ne pas changer): ");
    stdin.read_line(&mut status).unwrap();

    let new_status = match status.trim() {
        "1" => EmployeeStatus::Active.to_string(),
        "2" => EmployeeStatus::OnLeave.to_string(),
        "3" => EmployeeStatus::Fired.to_string(),
        _ => return,
    };

    employee.set_status(new_status);
  }
}