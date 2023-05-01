use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::error::FileError;

/// Abre el archivo del path recibido por parametro y si falla vuelve a pedirlo.
pub fn abrir_archivo(path: &str) -> Result<File, FileError> {
    match File::open(path) {
        Ok(it) => Ok(it),
        Err(_) => {
            println!("Nombre de archivo incorrecto");
            Err(FileError::ArchivoInexistente)
        }
    }
}

/// Lee las lineas del archivo recibido por parametro, las guarda en un vector y lo devuelve.
pub fn read_file_lines(path: &str) -> Result<Vec<Vec<i32>>, FileError> {
    let mut vector = Vec::new();
    if let Ok(file) = abrir_archivo(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let pedido: Vec<i32> = line
                .unwrap()
                .split(',')
                .map(|x| x.parse().expect("Failed to read file"))
                .collect();
            vector.push(pedido);
        }

        Ok(vector)
    } else {
        Err(FileError::ArchivoInexistente)
    }
}

/// Lee por pantalla, guarda lo leido en una variable y la devuelve.
pub fn leer_por_pantalla() -> String {
    let mut archivo_ingresado = String::new();

    io::stdin()
        .read_line(&mut archivo_ingresado)
        .expect("Failed to read line");

    archivo_ingresado.trim().to_string()
}
