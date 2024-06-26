use uuid::Uuid;

/// Représente un étudiant avec des informations personnelles
#[derive(Debug)]
pub struct Student {
  id: Uuid,
  name: String,
  email: String,
  phone: String,
  address: String,
  created_at: String,
  updated_at: String,
}

impl Student {
    /// Crée une nouvelle instance de `Student` (constructeur)
    ///
    /// # Arguments
    ///
    /// * `id` - Un `Uuid` représentant l'identifiant unique de l'étudiant.
    /// * `name` - Une `String` représentant le nom de l'étudiant.
    /// * `email` - Une `String` représentant l'adresse email de l'étudiant.
    /// * `phone` - Une `String` représentant le numéro de téléphone de l'étudiant.
    /// * `address` - Une `String` représentant l'adresse de l'étudiant.
    /// * `created_at` - Une `String` représentant la date de création de l'enregistrement de l'étudiant.
    /// * `updated_at` - Une `String` représentant la date de la dernière mise à jour de l'enregistrement de l'étudiant.
    ///
    /// # Retourne
    ///
    /// Une nouvelle instance de `Student`
    pub fn new(id: Uuid, name: String, email: String, phone: String, address: String, created_at: String, updated_at: String) -> Self {
      Self {id, name, email, phone, address, created_at, updated_at}
    }
}