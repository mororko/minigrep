
use std::{env, fs};
use std::error::Error;


pub fn buscar(argumentos:Argumentos) -> Result<(), Box<dyn Error>>{

    let contenido = fs::read_to_string(argumentos.nombre_archivo)?;

    let resultados = if argumentos.ignorar_case{
        buscar_palabra_insensible_al_case(&argumentos.busqueda, &contenido)
    } else {
        buscar_palabra(&argumentos.busqueda, &contenido)
    };

    for linea in resultados {
        println!("{linea}");
    }

    Ok(())
}

pub struct Argumentos {
    pub busqueda: String,
    pub nombre_archivo: String,
    pub ignorar_case: bool,
}

impl Argumentos {
    pub fn new(argumentos: &[String]) -> Result<Argumentos, &str> {

        if argumentos.len()<3{
            return Err("No hay suficientes argumentos");
        }

        let busqueda = argumentos[1].clone();
        let nombre_archivo = argumentos[2].clone();

        let ignorar_case = env::var("IGNORAR_CASE").is_ok();

        Ok(Argumentos {busqueda, nombre_archivo, ignorar_case})
    }
}

pub fn buscar_palabra<'a>(busqueda: &str, contenido: &'a str) -> Vec<&'a str> {
    let mut resultados = Vec::new();
    for linea in contenido.lines(){
        if linea.contains(busqueda) {
            resultados.push(linea);
        }
    }
    resultados
}

pub fn buscar_palabra_insensible_al_case<'a>(busqueda: &str, contenido: &'a str,) -> Vec<&'a str> {
    let busqueda = busqueda.to_lowercase();
    let mut resultados = Vec::new();
    
    for linea in contenido.lines(){
        if linea.to_lowercase().contains(&busqueda) {
            resultados.push(linea);
        }
    }
    resultados
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn un_resultado(){
        let busqueda = "pro";
        let contenido = "\
Rust:
seguro, rápido, productivo.
Elige tres.";

        assert_eq!(vec!["seguro, rápido, productivo."], buscar_palabra(busqueda, contenido));
    }


    #[test]
    fn insensible_al_case(){
        let busqueda = "gU";
        let contenido = "\
Rust:
seguro, rápido, productivo.
Elige tres.";

        assert_eq!(
            vec!["seguro, rápido, productivo."], 
            buscar_palabra_insensible_al_case(busqueda, contenido)
        );
    }

}