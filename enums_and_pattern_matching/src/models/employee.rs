use uuid::Uuid;


/// Représente un employé
pub struct Employee {
  /// Identifiant unique de l'employé
  id: Uuid,
  /// Nom complet de l'employé
  name: String,
  /// Statut de l'employé (actif, en congé, licencié, etc.)
  status: String,
}

impl Employee {
  /// Crée une nouvelle instance de `Employee` (constructeur)
    ///
    /// # Arguments
    ///
    /// * `id` - Un `Uuid` représentant l'identifiant unique de l'employé.
    /// * `name` - Une `String` représentant le nom de l'employé.
    /// * `status` - Une `String` représentant le statut de l'employé.
    ///
    /// # Retourne
    ///
    /// Une nouvelle instance de `Employee`
  pub fn new(name: String, status: String) -> Self {
      Self {
          id: Uuid::new_v4(),
          name,
          status,
      }
  }

  /// Renvoie l'identifiant de l'employé
  pub fn get_id(&self) -> Uuid {
    self.id
  }

  /// Renvoie le nom de l'employé
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /// Renvoie le statut de l'employé
  pub fn get_status(&self) -> String {
    self.status.clone()
  }

  /// Modifie le nom de l'employé
  pub fn set_name(&mut self, new_name: String) {
    self.name = new_name;
  }

  pub fn set_status(&mut self, new_status: String) {
    self.status = new_status;
  }
}