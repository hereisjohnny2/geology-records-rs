use super::{physical_properties::PhysicalProperty, sample_geometry::SampleGeometry};

#[derive(Debug, Clone)]
pub struct RockSample {
    pub name: String,
    pub description: String,
    pub mass: Box<PhysicalProperty>,
    pub porosity: Box<PhysicalProperty>,
    pub permeability: Box<PhysicalProperty>,
    pub geometry: Box<SampleGeometry>,
}

impl RockSample {
    pub fn new(
        name: String,
        description: String,
        mass: PhysicalProperty,
        porosity: PhysicalProperty,
        permeability: PhysicalProperty,
        geometry: SampleGeometry,
    ) -> Self {
        Self {
            name,
            description,
            mass: Box::new(mass),
            porosity: Box::new(porosity),
            permeability: Box::new(permeability),
            geometry: Box::new(geometry),
        }
    }

    pub fn density(&self) -> PhysicalProperty {
        let volume = self.geometry.clone().volume();

        let unit = match (&self.mass.unit, volume.unit) {
            (Some(mass_unit), Some(volume_unit)) => Some(format!("{}/{}", mass_unit, volume_unit)),
            (_, _) => None,
        };

        PhysicalProperty {
            value: self.mass.value / self.geometry.clone().volume().value,
            unit,
        }
    }
}

impl<'a> ToString for RockSample {
    fn to_string(&self) -> String {
        format!(
            "{}\n{}\n#mass: {}\n#porosity: {}\n#permeability: {}\n#height: {}\n#diameter: {}\n#volume: {}\n#density: {}\n",
            self.name,
            self.description,
            self.mass.to_string(),
            self.porosity.to_string(),
            self.permeability.to_string(),
            self.geometry.h.to_string(),
            self.geometry.diameter().to_string(),
            self.geometry.clone().volume().to_string(),
            self.density().to_string()
        )
        .to_string()
    }
}
