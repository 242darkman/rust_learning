use uuid::Uuid;

/// Représente un animal de compagnie avec des informations de base
#[derive(Debug)]
pub struct Pet {
  id: Uuid,
  name: String,
  age: i32,
  is_vaccinated: bool
}

impl Pet {
  /// Crée une nouvelle instance de `Pet` avec les informations spécifiées
  ///
  /// # Arguments
  ///
  /// * `id` - Un `Uuid` représentant l'identifiant unique de l'animal.
  /// * `name` - Une `String` représentant le nom de l'animal.
  /// * `age` - Un `i32` représentant l'âge de l'animal.
  /// * `is_vaccinated` - Un `bool` indiquant si l'animal est vacciné ou non.
  ///
  /// # Retourne
  ///
  /// Une nouvelle instance de `Pet`
  pub fn new(id: Uuid, name: String, age: i32, is_vaccinated: bool) -> Self {
    Self {id, name, age, is_vaccinated}
  }

  /// Obtient le nom de l'animal
  ///
  /// # Retourne
  ///
  /// Une référence à une `String` représentant le nom de l'animal
  pub fn get_name(&self) -> &String {
      &self.name
  }

  /// Définit le nom de l'animal
  ///
  /// # Arguments
  ///
  /// * `name` - Une `String` représentant le nouveau nom de l'animal
  pub fn set_name(&mut self, name: String) {
      self.name = name;
  }

  /// Définit l'âge de l'animal.
  ///
  /// # Arguments
  ///
  /// * `age` - Un `i32` représentant le nouvel âge de l'animal
  pub fn set_age(&mut self, age: i32) {
      self.age = age;
  }

  /// Définit le statut de vaccination de l'animal
  ///
  /// # Arguments
  ///
  /// * `is_vaccinated` - Un `bool` indiquant si l'animal est vacciné ou non
  pub fn set_vaccinated(&mut self, is_vaccinated: bool) {
      self.is_vaccinated = is_vaccinated;
  }

  /// Détermine si l'animal peut être autorisé dans un lieu en se basant sur son statut de vaccination.
  ///
  /// # Retourne
  ///
  /// Un `bool` indiquant si l'animal est autorisé ou non
  pub fn allow_pet(&mut self) -> bool {
    self.is_vaccinated
  }
}