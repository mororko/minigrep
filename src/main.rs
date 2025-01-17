use std::env;
use std::process;
use minigrep::Argumentos;

fn main() {
    let argumentos: Vec<String> = env::args().collect();

    let argumentos = Argumentos::new(&argumentos).unwrap_or_else(|err|
    {
        eprint!("Ha ocurrido un error: {}", err);
        process::exit(1);
    });

    eprintln!("Buscando {}", argumentos.busqueda);
    eprintln!("En el archivo {}", argumentos.nombre_archivo);

    //buscar(argumentos);

    if let Err(err) = minigrep::buscar(argumentos){
        println!("Ha ocurrido un error: {}", err);
        process::exit(1);
    }

}