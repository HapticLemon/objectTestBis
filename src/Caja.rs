use crate::Objeto::Objeto;
use crate::Definiciones::Point3;

pub struct Caja {
    pub id : u8
}

impl Objeto for Caja {
    fn getId(&self) -> u8 {
        return self.id
    }
    fn distancia(&self, punto:Point3) -> f32 {
        return 2.0
    }
}