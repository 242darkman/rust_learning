/// Trait `Vehicle` définissant les comportements de base d'un véhicule
pub trait Vehicle {
  /// Démarre le véhicule, modifiant son état et potentiellement sa vitesse.
  fn start(&mut self);

  /// Arrête le véhicule, réinitialisant son état et sa vitesse à zéro
  fn stop(&mut self);
}