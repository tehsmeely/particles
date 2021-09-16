use crate::grid::{Direction, GridPosition};
use array2d::Array2D;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy)]
pub struct Pixel(u64);
/*
A pixel is a u64 packed with various info:


Currently lazily using aligned u8s for all small fields until more info needs to be packed in


|--|--|--|--|--|--|--|--|
| 7| 6| 5| 4| 3| 2| 1| 0|

byte 0: material_type
byte 1: fill_level
byte 2: unused
byte 3: unused
byte 4: unused
byte 5: unused
byte 6: unused
byte 7: unused

 */

const MATERIAL_TYPE_MASK: u64 = 0x00000000000000FF;
const FILL_LEVEL_MASK: u64 = 0x000000000000FF00;

#[repr(u8)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum MaterialType {
    Sand = 1,
    Water = 2,
    Concrete = 3,
    Nothing = 0,
}

#[derive(Debug)]
pub struct UnpackedPixel {
    pub material_type: MaterialType,
    pub fill_level: u8,
}

impl Pixel {
    pub fn material_type_raw(&self) -> u8 {
        (self.0 & MATERIAL_TYPE_MASK) as u8
    }
    pub fn fill_level_raw(&self) -> u8 {
        ((self.0 & FILL_LEVEL_MASK) >> 8) as u8
    }

    pub fn unpack(&self) -> UnpackedPixel {
        UnpackedPixel {
            material_type: MaterialType::from_u8(self.material_type_raw()),
            fill_level: self.fill_level_raw(),
        }
    }

    pub fn pack(unpacked: UnpackedPixel) -> Self {
        let mut t = 0u64;
        t = t | (unpacked.material_type as u64);
        let fill_level_raw = (unpacked.fill_level as u64) << 8;
        t = t | fill_level_raw;
        Self(t)
    }

    pub fn blank() -> Self {
        Self(0u64)
    }

    pub fn update(&self, current_pos: &GridPosition, grid: &Array2D<Self>) -> GridPosition {
        current_pos.new_in_direction(&Direction::Down)
    }
}

impl MaterialType {
    pub fn as_colour(&self) -> ggez::graphics::Color {
        match self {
            MaterialType::Sand => [1.0, 0.741, 0.291, 1.0].into(),
            MaterialType::Water => [0.0, 0.0, 1.0, 1.0].into(),
            MaterialType::Concrete => [0.5, 0.5, 0.5, 1.0].into(),
            MaterialType::Nothing => [1.0, 1.0, 1.0, 1.0].into(),
        }
    }

    pub fn from_u8(u: u8) -> Self {
        match u {
            0 => Self::Nothing,
            1 => Self::Sand,
            2 => Self::Water,
            3 => Self::Concrete,
            _ => panic!("nope"),
        }
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let unpacked = self.unpack();
        write!(f, "Pixel<{},{:?}>", self.0, unpacked)
    }
}
