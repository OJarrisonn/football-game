use std::ops::Not;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AttributeQuality {
    VL, // Very Low
    LO, // Low
    OK, // Ok
    HI, // High
    VH // Very High
}

impl AttributeQuality {
    pub fn as_f32(self) -> f32 {
        match self {
            Self::VH => 2.,
            Self::HI => 1.,
            Self::OK => 0.,
            Self::LO => -1.,
            Self::VL => -2.,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            AttributeQuality::VL => AttributeQuality::VH,
            AttributeQuality::LO => AttributeQuality::HI,
            AttributeQuality::OK => AttributeQuality::OK,
            AttributeQuality::HI => AttributeQuality::LO,
            AttributeQuality::VH => AttributeQuality::VL,
        }
    }
}

impl From<f32> for AttributeQuality {
    fn from(f: f32) -> Self {
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

impl Into<f32> for AttributeQuality {
    fn into(self) -> f32 {
        self.as_f32()
    }
}

impl Not for AttributeQuality {
    type Output = AttributeQuality;

    fn not(self) -> Self::Output {
        self.opposite()
    }
}

#[derive(Clone, Copy)]
pub struct HomeFactor(AttributeQuality);

impl HomeFactor {
    pub fn new(attribute_quality: AttributeQuality) -> Self {
        Self(attribute_quality)
    }

    pub fn as_f32(self) -> f32 {
        match self.0 {
            AttributeQuality::VL => 1. / 1.04,
            AttributeQuality::LO => 1. / 1.02,
            AttributeQuality::OK => 1.,
            AttributeQuality::HI => 1.02,
            AttributeQuality::VH => 1.04,
        }
    }

    pub fn opposite(&self) -> Self {
        Self(!self.0)
    }
}

impl Into<f32> for HomeFactor {
    fn into(self) -> f32 {
        self.as_f32()
    }
}

impl Not for HomeFactor {
    type Output = HomeFactor;

    fn not(self) -> Self::Output {
        self.opposite()
    }
}