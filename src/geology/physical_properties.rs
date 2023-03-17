#[derive(Debug, Clone, Default)]
pub struct PhysicalProperty {
    pub value: f64,
    pub unit: Option<String>,
}

impl ToString for PhysicalProperty {
    fn to_string(&self) -> String {
        match &self.unit {
            Some(unit) => format!("{} {}", self.value, unit),
            None => format!("{}", self.value),
        }
    }
}
