use std::{io, str::FromStr};

use crate::geology::{
    physical_properties::PhysicalProperty, rock_sample::RockSample, sample_geometry::SampleGeometry,
};

pub fn safe_read(description: &str) -> String {
    println!("{description}:");
    let stdin = io::stdin();
    let mut buf = String::new();

    match stdin.read_line(&mut buf) {
        Ok(_) => buf.trim().to_string(),
        Err(e) => {
            println!("Try Again: {}", e);
            safe_read_and_parse(description)
        }
    }
}

pub fn safe_read_and_parse<T: FromStr>(description: &str) -> T {
    println!("{description}:");
    let stdin = io::stdin();
    let mut buf = String::new();

    match stdin.read_line(&mut buf) {
        Ok(_) => match buf.trim().parse::<T>() {
            Ok(n) => n,
            Err(_) => {
                println!("Try Again: error when trying to parse input");
                safe_read_and_parse(description)
            }
        },
        Err(e) => {
            println!("Try Again: {}", e);
            safe_read_and_parse(description)
        }
    }
}

fn read_physical_property<'a>(description: &'a str, unit: &'a str) -> PhysicalProperty {
    let value = safe_read_and_parse(format!("{} ({})", description, unit).as_str());

    let unit = if unit.is_empty() {
        None
    } else {
        Some(unit.to_string())
    };

    PhysicalProperty { value, unit }
}

pub fn create_rock_sample_entry(id: usize) -> RockSample {
    let name = safe_read("Conducted by");
    let description = safe_read("Sample Description");
    let mass = read_physical_property("Mass", "kg");
    let porosity = read_physical_property("Porosity", "%");
    let permeability = read_physical_property("Permeability", "md");
    let height = read_physical_property("Height", "mm");
    let radius = read_physical_property("Radius", "mm");

    let geometry = SampleGeometry::new(radius, height);
    RockSample::new(id, name, description, mass, porosity, permeability, geometry)
}
