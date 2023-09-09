use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum AttributeQuality {
    VL, // Very Low
    LO, // Low
    OK, // Ok
    HI, // High
    VH // Very High
}

impl AttributeQuality {
    pub fn as_float(&self) -> f32 {
        match self {
            Self::VH => 2.,
            Self::HI => 1.,
            Self::OK => 0.,
            Self::LO => -1.,
            Self::VL => -2.
        }
    }

    pub fn from(f: f32) -> Self {
        if f > 1.5 {
            Self::VH
        } else if f > 0.5 {
            Self::HI
        } else if f < -0.5 {
            Self::LO
        } else if f < -1.5 {
            Self::VL
        } else {
            Self::OK
        }
    }
}