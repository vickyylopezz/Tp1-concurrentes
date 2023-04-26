use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

use tp1::constantes::{MAX_CAFE_POR_PEDIDO, MIN_CANTIDAD_POR_PEDIDO};

use crate::pedido::Pedido;
mod pedido;
use crate::cafetera::Cafetera;
mod cafetera;

/// Abre el archivo del path recibido por parametro y si falla vuelve a pedirlo.
/// Ejemplo:
/// ```rust
/// abrir_archivo(nombreArchivo);
/// ```
fn abrir_archivo(path: String) -> File {
    match File::open(path) {
        Ok(it) => it,
        Err(_) => {
            println!("Nombre de archivo incorrecto. Por favor vuelva a ingresarlo:");
            abrir_archivo(leer_por_pantalla())
        }
    }
}

/// Lee las lineas del archivo recibido por parametro, las guarda en un vector y lo devuelve.
/// Ejemplo:
/// ```rust
/// read_file_lines(nombreArchivo);
/// ```
fn read_file_lines(path: String) -> Result<Vec<Vec<i32>>, Error> {
    let mut vector = Vec::new();
    let reader = BufReader::new(abrir_archivo(path));
    for line in reader.lines() {
        let pedido: Vec<i32> = line?
            .split(',')
            .map(|x| x.parse().expect("Failed to read file"))
            .collect();
        vector.push(pedido);
    }

    Ok(vector)
}

/// Lee por pantalla, guarda lo leido en una variable y la devuelve.
/// Ejemplo:
/// ```rust
/// leer_por_pantall();
/// ```
fn leer_por_pantalla() -> String {
    let mut archivo_ingresado = String::new();

    io::stdin()
        .read_line(&mut archivo_ingresado)
        .expect("Failed to read line");

    archivo_ingresado.trim().to_string()
}

fn pedidos(pedidos_archivo: Vec<Vec<i32>>) -> Vec<Pedido> {
    let mut pedidos = Vec::<Pedido>::new();
    for (i, pedido) in pedidos_archivo.into_iter().enumerate() {
        if pedido[0] > MAX_CAFE_POR_PEDIDO || pedido[0] < MIN_CANTIDAD_POR_PEDIDO {
            if pedido[0] > MAX_CAFE_POR_PEDIDO {
                println!(
                    "La cantidad maxima de cafe por pedido es {}, pedido {} descartado",
                    MAX_CAFE_POR_PEDIDO, i
                );
            } else {
                println!(
                    "La cantidad minima de cafe por pedido es {}, pedido {} descartado",
                    MIN_CANTIDAD_POR_PEDIDO, i
                );
            }

            continue;
        }
        pedidos.push(Pedido {
            cafe_molido: pedido[0],
            agua_caliente: pedido[1],
            cacao: pedido[2],
            espuma: pedido[3],
        })
    }
    pedidos
}

fn main() {
    println!("Bienvenido!");
    println!("Ingrese el archivo con el pedido");
    let pedidos_archivo = read_file_lines(leer_por_pantalla()).expect("Failed to read file");
    let pedidos = pedidos(pedidos_archivo);
    Cafetera::new().preparar_pedidos(pedidos);
}
