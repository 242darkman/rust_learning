use crate::traits::vehicule::Vehicle;

/// Représente un bateau avec des attributs pour le nom, l'état de mouvement, et la vitesse actuelle
pub struct Boat {
  /// Nom du bateau
  pub name: String,
  /// Indique si le bateau est en mouvement
  pub is_on_motion: bool,
  /// Vitesse actuelle du bateau (en km/h)
  pub current_speed: u32,
}

/// Implémentation du trait `Vehicle` pour `Boat`
impl Vehicle for Boat {
  // Démarre le bateau, changeant son état à en mouvement et définissant sa vitesse initiale à 5 km/h.
  ///
  /// Affiche un message indiquant que le bateau est en mouvement avec sa vitesse actuelle
  fn start(&mut self) {
      self.is_on_motion = true;
      self.current_speed = 5;
      println!("The {} boat is moving at {} km/h", self.name, self.current_speed);
  }

  /// Arrête le bateau, changeant son état à non en mouvement et réinitialisant sa vitesse à 0.
  ///
  /// Affiche un message indiquant que le bateau est arrêté
  fn stop(&mut self) {
      self.is_on_motion = false;
      self.current_speed = 0;
      println!("The boat stopped");
  }
}