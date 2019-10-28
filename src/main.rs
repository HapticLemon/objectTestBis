mod Esfera;
mod Caja;
mod Objeto;
mod Definiciones;
mod Vectores;

/*fn calculaDistancia(punto : Definiciones::Point3, item: Box<Objeto::Objeto>) ->f32{
    return item.distancia(punto);
}*/

/*unsafe fn mapTheWorld(punto : Definiciones::Point3, Escena : Vec<Box<Objeto::Objeto>>) -> f32{
    let mut distancia : f32 = 1000.0;
    let mut distanciaObjeto: f32 = 0.0;

    for item in Escena.iter() {
        distanciaObjeto = calculaDistancia(punto,item);
        if distanciaObjeto < distancia {
            distancia = distanciaObjeto
        }
    }
    return distancia;
}*/


// Puedo hacerlo de Esfera porque es struct pero no de objeto porque es trait.
fn main () {
    let mut distancia : f32 = 1000.0;
    let mut distanciaObjeto: f32 = 0.0;

    let esfera: Esfera::Esfera = Esfera::Esfera{ id: 0 };
    let caja: Caja::Caja = Caja::Caja{ id: 1 };

    let mut v: Vec<Box<Objeto::Objeto>> = Vec::new();

    //let mut v: Vec<&Objeto::Objeto> = Vec::new();
    v.push(Box::new(esfera));
    v.push(Box::new(caja));

    let punto = Definiciones::Point3{x:1.0, y:2.0, z:3.0};

    for item in v.iter() {
        distanciaObjeto = item.distancia(punto);
        if distanciaObjeto < distancia {
            distancia = distanciaObjeto
        }
    }
    println!("Distancia {}", distancia);
}