use crate::Objeto::Objeto;
use crate::Definiciones::{Point3, ColorRGB};
use crate::Vectores::*;

pub struct Esfera{
    pub id : u8,
    pub radio : u8,
    pub traslacion : Point3,
    pub color : ColorRGB
}

impl Objeto for Esfera {
    fn getId(&self) -> u8 {
        return self.id
    }

    fn distancia(&self,punto:Point3) -> f32 {
        (Length(punto) - 5.0)
    }
}