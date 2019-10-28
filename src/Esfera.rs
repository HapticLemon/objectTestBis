use crate::Objeto::Objeto;
use crate::Definiciones::Point3;
use crate::Vectores::*;

pub struct Esfera{
    pub id : u8
}

impl Objeto for Esfera {
    fn getId(&self) -> u8 {
        return self.id
    }

    fn distancia(&self,punto:Point3) -> f32 {
        (Length(punto) - 5.0)
    }
}