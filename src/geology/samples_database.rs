use super::{rock_sample::RockSample, physical_properties::PhysicalProperty};

pub struct SamplesDatabase {
    samples: Vec<RockSample>,
}

impl SamplesDatabase {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn add(&mut self, value: RockSample) {
        self.samples.push(value);
    }

    fn avg_porosity(&self) -> Option<PhysicalProperty> {
        match self.samples.first() {
            Some(s) => {
                let value = self
                    .samples
                    .iter()
                    .fold(0.0, |acc, curr| acc + curr.porosity.value)
                    / self.samples.len() as f64;
                let unit = s.porosity.clone().unit;

                Some(PhysicalProperty { value, unit })
            }
            None => None,
        }
    }

    fn avg_permeability(&self) -> Option<PhysicalProperty> {
        match self.samples.first() {
            Some(s) => {
                let value = self
                    .samples
                    .iter()
                    .fold(0.0, |acc, curr| acc + curr.permeability.value)
                    / self.samples.len() as f64;
                let unit = s.permeability.clone().unit;

                Some(PhysicalProperty { value, unit })
            }
            None => None,
        }
    }
}

impl ToString for SamplesDatabase {
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str("Geology Report\n\n");

        if let Some(avg_porosity) = self.avg_porosity() {
            output.push_str(
                format!(
                    "Average Porosity: {} {}\n",
                    avg_porosity.value,
                    avg_porosity.unit.unwrap_or("".to_string())
                )
                .as_str(),
            );
        }

        if let Some(avg_permeability) = self.avg_permeability() {
            output.push_str(
                format!(
                    "Average Permeability: {} {}\n",
                    avg_permeability.value,
                    avg_permeability.unit.unwrap_or("".to_string())
                )
                .as_str(),
            );
        }

        self.samples.iter().enumerate().for_each(|(key, value)| {
            output.push_str(format!("\n{}\n{}\n", key, value.to_string()).as_str())
        });

        output
    }
}
