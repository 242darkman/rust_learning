use crate::traits::vehicule::Vehicle;

/// Représente une voiture avec des attributs pour le modèle, la couleur, l'état de mouvement, et la vitesse actuelle
#[derive(Debug)]
pub struct Car {
  /// Modèle de la voiture
  pub model: String,
  /// Couleur de la voiture
  pub color: String,
  /// Indique si la voiture est en mouvement
  pub is_on_motion: bool,
  /// Vitesse actuelle de la voiture (en km/h)
  pub current_speed: u32,
}

impl Vehicle for Car {
  /// Démarre la voiture, changeant son état à en mouvement et définissant sa vitesse initiale à 10 km/h.
  ///
  /// Affiche un message indiquant que la voiture est en mouvement avec sa vitesse actuelle
  fn start(&mut self) {
    self.is_on_motion = true;
    self.current_speed = 10;
    println!("The {} {} is moving at {} km/h", self.color, self.model, self.current_speed);
  }

  /// Arrête la voiture, changeant son état à non en mouvement et réinitialisant sa vitesse à 0.
  ///
  /// Affiche un message indiquant que la voiture est arrêtée
  fn stop(&mut self) {
    self.is_on_motion = false;
    self.current_speed = 0;
    println!("The car stopped");
  }
}