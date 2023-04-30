use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

use tp1::constantes::{
    MAX_AGUA_POR_PEDIDO, MAX_CACAO_POR_PEDIDO, MAX_CAFE_POR_PEDIDO, MAX_ESPUMA_POR_PEDIDO,
    MIN_CANTIDAD_POR_PEDIDO,
};

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

/// Chequea que la cantidad de cafe del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
/// Ejemplo:
/// ```rust
/// cafe_invalido(cant_cafe_del_pedido,id_del_pedido);
/// ```
fn cafe_invalido(cantidad_cafe: i32, i: usize) -> bool {
    if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_CAFE_POR_PEDIDO).contains(&cantidad_cafe) {
        if cantidad_cafe > MAX_CAFE_POR_PEDIDO {
            println!(
                "La cantidad maxima de cafe por pedido es {}, pedido {} descartado",
                MAX_CAFE_POR_PEDIDO, i
            );
            return true;
        } else {
            println!(
                "La cantidad minima de cafe por pedido es {}, pedido {} descartado",
                MIN_CANTIDAD_POR_PEDIDO, i
            );
            return true;
        }
    }
    false
}

/// Chequea que la cantidad de agua del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
/// Ejemplo:
/// ```rust
/// agua_invalida(cant_agua_del_pedido,id_del_pedido);
/// ```
fn agua_invalida(cantidad_agua: i32, i: usize) -> bool {
    if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_AGUA_POR_PEDIDO).contains(&cantidad_agua) {
        if cantidad_agua > MAX_AGUA_POR_PEDIDO {
            println!(
                "La cantidad maxima de agua por pedido es {}, pedido {} descartado",
                MAX_AGUA_POR_PEDIDO, i
            );
            return true;
        } else {
            println!(
                "La cantidad minima de agua por pedido es {}, pedido {} descartado",
                MIN_CANTIDAD_POR_PEDIDO, i
            );
            return true;
        }
    }
    false
}

/// Chequea que la cantidad de cacao del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
/// Ejemplo:
/// ```rust
/// cacao_invalido(cant_cacao_del_pedido,id_del_pedido);
/// ```
fn cacao_invalido(cantidad_cacao: i32, i: usize) -> bool {
    if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_CACAO_POR_PEDIDO).contains(&cantidad_cacao) {
        if cantidad_cacao > MAX_CACAO_POR_PEDIDO {
            println!(
                "La cantidad maxima de cacao por pedido es {}, pedido {} descartado",
                MAX_CACAO_POR_PEDIDO, i
            );
            return true;
        } else {
            println!(
                "La cantidad minima de cacao por pedido es {}, pedido {} descartado",
                MIN_CANTIDAD_POR_PEDIDO, i
            );
            return true;
        }
    }
    false
}

/// Chequea que la cantidad de espuma del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
/// Ejemplo:
/// ```rust
/// espuma_invalido(cant_espuma_del_pedido,id_del_pedido);
/// ```
fn espuma_invalida(cantidad_espuma: i32, i: usize) -> bool {
    if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_ESPUMA_POR_PEDIDO).contains(&cantidad_espuma) {
        if cantidad_espuma > MAX_ESPUMA_POR_PEDIDO {
            println!(
                "La cantidad maxima de espuma por pedido es {}, pedido {} descartado",
                MAX_ESPUMA_POR_PEDIDO, i
            );
            return true;
        } else {
            println!(
                "La cantidad minima de espuma por pedido es {}, pedido {} descartado",
                MIN_CANTIDAD_POR_PEDIDO, i
            );
            return true;
        }
    }
    false
}

/// Transforma cada pedido ingresado a un objeto del tipo Pedido y descarta los pedidos invalidos
fn pedidos(pedidos_archivo: Vec<Vec<i32>>) -> Vec<Pedido> {
    let mut pedidos = Vec::<Pedido>::new();
    for (i, pedido) in pedidos_archivo.into_iter().enumerate() {
        if cafe_invalido(pedido[0], i)
            || agua_invalida(pedido[1], i)
            || cacao_invalido(pedido[2], i)
            || espuma_invalida(pedido[3], i)
        {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pedidos_ok_test() {
        let pedidos_archivo = vec![vec![2, 3, 4, 5], vec![5, 3, 2, 4], vec![4, 5, 2, 3]];
        let pedidos = pedidos(pedidos_archivo);

        let pedidos_assert = vec![
            Pedido {
                cafe_molido: 2,
                agua_caliente: 3,
                cacao: 4,
                espuma: 5,
            },
            Pedido {
                cafe_molido: 5,
                agua_caliente: 3,
                cacao: 2,
                espuma: 4,
            },
            Pedido {
                cafe_molido: 4,
                agua_caliente: 5,
                cacao: 2,
                espuma: 3,
            },
        ];
        for (i, pedido) in pedidos.into_iter().enumerate() {
            assert_eq!(pedido, pedidos_assert[i]);
        }
    }

    #[test]
    fn pedidos_mal_descartados_test() {
        let pedidos_archivo = vec![vec![-4, 3, 4, 5], vec![5, -5, 2, 4], vec![4, 5, 2, 3]];
        let pedidos = pedidos(pedidos_archivo);

        let pedidos_assert = vec![Pedido {
            cafe_molido: 4,
            agua_caliente: 5,
            cacao: 2,
            espuma: 3,
        }];
        for (i, pedido) in pedidos.into_iter().enumerate() {
            assert_eq!(pedido, pedidos_assert[i]);
        }
    }

    #[test]
    fn espuma_invalida_negativa_test() {
        let espuma_invalida = espuma_invalida(-3, 0);
        assert_eq!(espuma_invalida, true);
    }

    #[test]
    fn espuma_invalida_mas_maximo_test() {
        let espuma_invalida = espuma_invalida(MAX_ESPUMA_POR_PEDIDO + 1, 0);
        assert_eq!(espuma_invalida, true);
    }

    #[test]
    fn espuma_valida_test() {
        let espuma_valida = espuma_invalida(MAX_ESPUMA_POR_PEDIDO - 1, 0);
        assert_eq!(espuma_valida, false);
    }

    #[test]
    fn cacao_invalido_negativo_test() {
        let cacao_invalido = cacao_invalido(-1, 0);
        assert_eq!(cacao_invalido, true);
    }

    #[test]
    fn cacao_invalido_mas_maximo_test() {
        let cacao_invalido = cacao_invalido(MAX_CACAO_POR_PEDIDO + 1, 0);
        assert_eq!(cacao_invalido, true);
    }

    #[test]
    fn cacao_valido_test() {
        let cacao_valido = cacao_invalido(MAX_CACAO_POR_PEDIDO - 1, 0);
        assert_eq!(cacao_valido, false);
    }

    #[test]
    fn agua_invalida_negativa_test() {
        let agua_invalida = agua_invalida(-7, 0);
        assert_eq!(agua_invalida, true);
    }

    #[test]
    fn agua_invalida_mas_maximo_test() {
        let agua_invalida = agua_invalida(MAX_AGUA_POR_PEDIDO + 1, 0);
        assert_eq!(agua_invalida, true);
    }

    #[test]
    fn agua_valida_test() {
        let agua_valida = agua_invalida(MAX_AGUA_POR_PEDIDO - 1, 0);
        assert_eq!(agua_valida, false);
    }

    #[test]
    fn cafe_invalido_negativo_test() {
        let cafe_invalido = cafe_invalido(-2, 0);
        assert_eq!(cafe_invalido, true);
    }

    #[test]
    fn cafe_invalido_mas_maximo_test() {
        let cafe_invalido = cafe_invalido(MAX_CAFE_POR_PEDIDO + 1, 0);
        assert_eq!(cafe_invalido, true);
    }

    #[test]
    fn cafe_valido_test() {
        let cafe_valido = cafe_invalido(MAX_CAFE_POR_PEDIDO - 1, 0);
        assert_eq!(cafe_valido, false);
    }
}
