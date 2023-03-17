use std::f64::consts::PI;

use super::physical_properties::PhysicalProperty;

#[derive(Debug, Clone, Default)]
pub struct SampleGeometry {
    pub r: Box<PhysicalProperty>,
    pub h: Box<PhysicalProperty>,
}

impl SampleGeometry {
    pub fn new(r: PhysicalProperty, h: PhysicalProperty) -> Self {
        Self {
            r: Box::new(r),
            h: Box::new(h),
        }
    }

    pub fn volume(&self) -> PhysicalProperty {
        let unit = match &self.r.unit {
            Some(u) => Some(format!("{}³", u)),
            None => None,
        };

        PhysicalProperty {
            value: PI * self.r.value.powi(2) * self.h.value,
            unit,
        }
    }

    pub fn diameter(&self) -> PhysicalProperty {
        let unit = match &self.r.unit {
            Some(u) => Some(u),
            None => None,
        }
        .cloned();

        PhysicalProperty {
            value: self.r.value * 2.0,
            unit,
        }
    }
}
