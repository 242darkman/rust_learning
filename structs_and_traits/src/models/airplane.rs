use crate::traits::vehicule::Vehicle;

/// Représente un avion avec des attributs pour l'altitude, l'état de mouvement, et la vitesse actuelle
pub struct Airplane {
  /// Altitude de l'avion en pieds
  pub altitude: f64,
  /// Indique si l'avion est en mouvement
  pub is_on_motion: bool,
  /// Vitesse actuelle de l'avion (en km/h)
  pub current_speed: u32,
}

/// Implémentation du trait `Vehicle` pour `Airplane`
impl Vehicle for Airplane {
  /// Démarre l'avion, changeant son état à en mouvement et définissant sa vitesse initiale à 50 km/h.
  ///
  /// Affiche un message indiquant que l'avion est en mouvement avec sa vitesse actuelle et son altitude
  fn start(&mut self) {
      self.is_on_motion = true;
      self.current_speed = 50;
      println!("The airplane is moving at {} km/h with an altitude of {} feet", self.current_speed, self.altitude);
  }

  /// Arrête l'avion, changeant son état à non en mouvement et réinitialisant sa vitesse à 0.
  ///
  /// Affiche un message indiquant que l'avion est arrêté
  fn stop(&mut self) {
      self.is_on_motion = false;
      self.current_speed = 0;
      println!("The airplane stopped");
  }
}