#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::io::BufReader;
use std::fs::File;
use crate::Definiciones::{Point3, ColorRGB};
//use crate::Esfera::Esfera;

mod Esfera;
mod Caja;
mod Objeto;
mod Definiciones;
mod Vectores;

#[derive(Deserialize)]
struct Landmark {
    id: u8,
    radio: u8,
    traslacion : Point3,
    color : ColorRGB
}

fn calculaDistancia(punto : Definiciones::Point3, item: &Box<dyn Objeto::Objeto>) ->f32{
    return item.distancia(punto);
}

fn mapTheWorld(punto : Definiciones::Point3, Escena : &Vec<Box<dyn Objeto::Objeto>>) -> (f32, u8){
    let mut distancia : f32 = 1000.0;
    let mut distanciaObjeto: f32 = 0.0;
    let mut idObjeto : u8 = 0;
    let mut contador : u8 = 0;


    for item in Escena.iter() {
        //distanciaObjeto = calculaDistancia(punto,item);
        distanciaObjeto = item.distancia(punto);
        if distanciaObjeto < distancia {
            distancia = distanciaObjeto;
            idObjeto = contador;
        }
        contador +=1 ;
    }
    return (distancia, idObjeto);
}

fn calculateNormal(punto : Definiciones::Point3, Escena : &Vec<Box<dyn Objeto::Objeto>>, idObjeto :usize) -> Definiciones::Point3{
    let mut gradiente: Definiciones::Point3 = Definiciones::Point3 { x: 1.0, y: 0.0, z: 0.0 };
    let EPSILON = 0.01;

    //gradiente.x = calculaDistancia(Definiciones::Point3 { x: punto.x + EPSILON, y: punto.y, z: punto.z }, &Escena[idObjeto]) - calculaDistancia(Definiciones::Point3 { x: punto.x - EPSILON, y: punto.y, z: punto.z }, &Escena[idObjeto]);
    //gradiente.y = calculaDistancia(Definiciones::Point3 { x: punto.x , y: punto.y + EPSILON, z: punto.z }, &Escena[idObjeto]) - calculaDistancia(Definiciones::Point3 { x: punto.x, y: punto.y - EPSILON, z: punto.z }, &Escena[idObjeto]);
    //gradiente.z = calculaDistancia(Definiciones::Point3 { x: punto.x , y: punto.y, z: punto.z + EPSILON}, &Escena[idObjeto]) - calculaDistancia(Definiciones::Point3 { x: punto.x , y: punto.y, z: punto.z - EPSILON}, &Escena[idObjeto]);

    gradiente.x = &Escena[idObjeto].distancia(Definiciones::Point3 { x: punto.x + EPSILON, y: punto.y, z: punto.z }) - &Escena[idObjeto].distancia(Definiciones::Point3 { x: punto.x - EPSILON, y: punto.y, z: punto.z });
    gradiente.y = &Escena[idObjeto].distancia(Definiciones::Point3 { x: punto.x , y: punto.y + EPSILON, z: punto.z }) - &Escena[idObjeto].distancia(Definiciones::Point3 { x: punto.x , y: punto.y - EPSILON, z: punto.z });
    gradiente.z = &Escena[idObjeto].distancia(Definiciones::Point3 { x: punto.x , y: punto.y , z: punto.z + EPSILON}) - &Escena[idObjeto].distancia(Definiciones::Point3 { x: punto.x , y: punto.y, z: punto.z - EPSILON});

    Vectores::MultiplyByScalar(gradiente,-1.0);

    return (Vectores::Normalize(gradiente))
}

// Puedo hacerlo de Esfera porque es struct pero no de objeto porque es trait.
fn main () {
    let mut distancia : f32 = 1000.0;
    let mut distanciaObjeto: f32 = 0.0;


    let file = File::open("Escena.json").unwrap();
    let reader = BufReader::new(file);
    let landmarks: Vec<Landmark> = serde_json::from_reader(reader).unwrap();

    for landmark in landmarks{
        println!("Landmark id: {}\tRadio: {}",landmark.id,landmark.radio);
    }

    let esfera: Esfera = Esfera{ id: 0 , radio : 5, traslacion : Point3 { x: 6.0, y: 0.0, z: 0.0 },color :ColorRGB { R: 200, G: 0, B: 0 }};
    let caja: Caja::Caja = Caja::Caja{ id: 1 };

    let mut idObjeto : u8 = 0;

    let mut v: Vec<Box<Objeto::Objeto>> = Vec::new();
    let mut normal: Definiciones::Point3 = Definiciones::Point3 { x: 0.0, y: 0.0, z: 0.0 };

    //let mut v: Vec<&Objeto::Objeto> = Vec::new();
    v.push(Box::new(esfera));
    v.push(Box::new(caja));

    let punto = Definiciones::Point3{x:1.0, y:2.0, z:3.0};

    let (distancia, idObjeto) = mapTheWorld(punto,&v);
    normal = calculateNormal(punto, &v, idObjeto as usize);
    println!("Distancia {}", distancia);
}