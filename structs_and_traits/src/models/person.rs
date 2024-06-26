// Représente une personne avec ses informations
pub struct Person {
  name: String,
  age: u8,
  gender: char,
  address: String,
  city: String,
}

impl Person {
  /// Crée une nouvelle instance de `Person` (constructeur)
  ///
  /// # Arguments
  ///
  /// * `name` - Une `String` représentant le nom de la personne.
  /// * `age` - Un `u8` représentant l'âge de la personne.
  /// * `gender` - Un `char` représentant le genre de la personne ('M' pour masculin, 'F' pour féminin).
  /// * `address` - Une `String` représentant l'adresse de la personne.
  /// * `city` - Une `String` représentant la ville où vit la personne.
  ///
  /// # Retourne
  ///
  /// Une nouvelle instance de `Person`.
  pub fn new(name: String, age: u8, gender: char, address: String, city: String) -> Self {
      Self { name, age, gender, address, city }
  }

  /// Obtient le nom de la personne
  ///
  /// # Retourne
  ///
  /// Une `String` représentant le nom de la personne
  pub fn get_name(&self) -> String {
      self.name.clone()
  }

  /// Obtient l'âge de la personne
  ///
  /// # Retourne
  ///
  /// Un `u8` représentant l'âge de la personne
  pub fn get_age(&self) -> u8 {
      self.age
  }

  /// Obtient le genre de la personne
  ///
  /// # Retourne
  ///
  /// Un `char` représentant le genre de la personne
  pub fn get_gender(&self) -> char {
      self.gender
  }

  /// Obtient l'adresse de la personne
  ///
  /// # Retourne
  ///
  /// Une `String` représentant l'adresse de la personne
  pub fn get_address(&self) -> String {
      self.address.clone()
  }

  /// Obtient la ville où vit la personne
  ///
  /// # Retourne
  ///
  /// Une `String` représentant la ville où vit la personne
  pub fn get_city(&self) -> String {
      self.city.clone()
  }

  /// Affiche les informations détaillées de la personne
  ///
  /// # Retourne
  ///
  /// Une `String` contenant un message formaté avec les détails de la personne
  pub fn afficher_details(&self) -> String {
    let person_gender = {
      if self.get_gender() == 'M' {
          "un Homme"
      } else {
          "une Femme"
      }
    };

    return format!(
        "Salut, Je m'appelle {}. J'ai {} ans, je suis '{}', J'habite au {}, {}. \n",
        self.get_name(), self.get_age(), person_gender, self.get_address(), self.get_city()
    );
  }

  /// Incrémente l'âge de la personne de 1
  pub fn incremente_age(&mut self) {
      self.age += 1;
      println!("{} a maintenant {} ans", self.get_name(), self.get_age());
  }

  /// Change le nom de la personne
  ///
  /// # Arguments
  ///
  /// * `new_name` - Une `String` représentant le nouveau nom de la personne
  pub fn changer_nom(&mut self, new_name: String) {
    let previous_name = self.get_name();
    self.name = new_name.clone();
    println!("{} s'appelle maintenant {}", previous_name, new_name.to_string());
  }

  /// Compare l'âge de deux instances de `Personne`.
  ///
  /// # Arguments
  ///
  /// * `person` - Une référence à une autre instance de `Personne` pour comparer l'âge
  pub fn is_older_than(&self, person: &Self) {
    if self.get_age() > person.get_age() {
      println!("{} est plus age que {}", self.get_name(), person.get_name());
    } else {
      println!("{} n'est pas plus age que {}", self.get_name(), person.get_name());
    }
  }
}