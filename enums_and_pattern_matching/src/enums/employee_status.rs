pub enum EmployeeStatus {
  Active,
  OnLeave,
  Fired,
}

impl EmployeeStatus {
  pub fn to_string(&self) -> String {
    match self {
      EmployeeStatus::Active => "Actif".to_string(),
      EmployeeStatus::OnLeave => "En congé".to_string(),
      EmployeeStatus::Fired => "Licencié".to_string(),
    }
  }
}