use tp1::cafetera::Cafetera;
use tp1::{archivo::leer_por_pantalla, archivo::read_file_lines};

use tp1::chequeo_pedidos::pedidos;

// pub mod chequeo_pedidos {
//     use tp1::{
//         constantes::{
//             MAX_AGUA_POR_PEDIDO, MAX_CACAO_POR_PEDIDO, MAX_CAFE_POR_PEDIDO, MAX_ESPUMA_POR_PEDIDO,
//             MIN_CANTIDAD_POR_PEDIDO,
//         },
//         error::PedidoError,
//         pedido::Pedido,
//     };

//     /// Chequea que la cantidad de cafe del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
//     /// Ejemplo:
//     /// ```rust
//     /// cafe_invalido(cant_cafe_del_pedido,id_del_pedido);
//     /// ```
//     pub fn cafe_invalido(cantidad_cafe: i32, i: usize) -> bool {
//         if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_CAFE_POR_PEDIDO).contains(&cantidad_cafe) {
//             if cantidad_cafe > MAX_CAFE_POR_PEDIDO {
//                 println!(
//                     "La cantidad maxima de cafe por pedido es {}, pedido {} descartado",
//                     MAX_CAFE_POR_PEDIDO, i
//                 );
//                 return true;
//             } else {
//                 println!(
//                     "La cantidad minima de cafe por pedido es {}, pedido {} descartado",
//                     MIN_CANTIDAD_POR_PEDIDO, i
//                 );
//                 return true;
//             }
//         }
//         false
//     }

//     /// Chequea que la cantidad de agua del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
//     /// Ejemplo:
//     /// ```rust
//     /// agua_invalida(cant_agua_del_pedido,id_del_pedido);
//     /// ```
//     pub fn agua_invalida(cantidad_agua: i32, i: usize) -> bool {
//         if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_AGUA_POR_PEDIDO).contains(&cantidad_agua) {
//             if cantidad_agua > MAX_AGUA_POR_PEDIDO {
//                 println!(
//                     "La cantidad maxima de agua por pedido es {}, pedido {} descartado",
//                     MAX_AGUA_POR_PEDIDO, i
//                 );
//                 return true;
//             } else {
//                 println!(
//                     "La cantidad minima de agua por pedido es {}, pedido {} descartado",
//                     MIN_CANTIDAD_POR_PEDIDO, i
//                 );
//                 return true;
//             }
//         }
//         false
//     }

//     /// Chequea que la cantidad de cacao del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
//     /// Ejemplo:
//     /// ```rust
//     /// cacao_invalido(cant_cacao_del_pedido,id_del_pedido);
//     /// ```
//     pub fn cacao_invalido(cantidad_cacao: i32, i: usize) -> bool {
//         if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_CACAO_POR_PEDIDO).contains(&cantidad_cacao) {
//             if cantidad_cacao > MAX_CACAO_POR_PEDIDO {
//                 println!(
//                     "La cantidad maxima de cacao por pedido es {}, pedido {} descartado",
//                     MAX_CACAO_POR_PEDIDO, i
//                 );
//                 return true;
//             } else {
//                 println!(
//                     "La cantidad minima de cacao por pedido es {}, pedido {} descartado",
//                     MIN_CANTIDAD_POR_PEDIDO, i
//                 );
//                 return true;
//             }
//         }
//         false
//     }

//     /// Chequea que la cantidad de espuma del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
//     /// Ejemplo:
//     /// ```rust
//     /// espuma_invalido(cant_espuma_del_pedido,id_del_pedido);
//     /// ```
//     pub fn espuma_invalida(cantidad_espuma: i32, i: usize) -> bool {
//         if !(MIN_CANTIDAD_POR_PEDIDO..=MAX_ESPUMA_POR_PEDIDO).contains(&cantidad_espuma) {
//             if cantidad_espuma > MAX_ESPUMA_POR_PEDIDO {
//                 println!(
//                     "La cantidad maxima de espuma por pedido es {}, pedido {} descartado",
//                     MAX_ESPUMA_POR_PEDIDO, i
//                 );
//                 return true;
//             } else {
//                 println!(
//                     "La cantidad minima de espuma por pedido es {}, pedido {} descartado",
//                     MIN_CANTIDAD_POR_PEDIDO, i
//                 );
//                 return true;
//             }
//         }
//         false
//     }

//     /// Transforma cada pedido ingresado a un objeto del tipo Pedido y descarta los pedidos invalidos
//     pub fn pedidos(pedidos_archivo: Vec<Vec<i32>>) -> Result<Vec<Pedido>, PedidoError> {
//         let mut pedidos = Vec::<Pedido>::new();
//         for (i, pedido) in pedidos_archivo.into_iter().enumerate() {
//             if cafe_invalido(pedido[0], i)
//                 || agua_invalida(pedido[1], i)
//                 || cacao_invalido(pedido[2], i)
//                 || espuma_invalida(pedido[3], i)
//             {
//                 continue;
//             }
//             pedidos.push(Pedido {
//                 cafe_molido: pedido[0],
//                 agua_caliente: pedido[1],
//                 cacao: pedido[2],
//                 espuma: pedido[3],
//             })
//         }
//         if pedidos.is_empty() {
//             println!("No hay pedidos para procesar");
//             return Err(PedidoError::NoHayPedidos);
//         }
//         Ok(pedidos)
//     }
// }

fn main() {
    println!("Bienvenido!");
    println!("Ingrese el archivo con el pedido");
    let pedidos_archivo = read_file_lines(&leer_por_pantalla());
    if let Ok(archivo) = pedidos_archivo {
        if let Ok(pedidos) = pedidos(archivo) {
            Cafetera::new().preparar_pedidos(pedidos);
        }
    }
}

#[cfg(test)]
mod tests {
    use tp1::chequeo_pedidos::{
        agua_invalida, cacao_invalido, cafe_invalido, espuma_invalida, pedidos,
    };
    use tp1::{
        constantes::{
            MAX_AGUA_POR_PEDIDO, MAX_CACAO_POR_PEDIDO, MAX_CAFE_POR_PEDIDO, MAX_ESPUMA_POR_PEDIDO,
        },
        error::PedidoError,
        pedido::Pedido,
    };

    #[test]
    fn pedidos_ok_test() {
        let pedidos_archivo = vec![vec![2, 3, 4, 5], vec![5, 3, 2, 4], vec![4, 5, 2, 3]];
        let pedidos = pedidos(pedidos_archivo);

        let pedidos_expected = vec![
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
        for (i, pedido) in pedidos
            .expect("Error al obtener los pedidos")
            .into_iter()
            .enumerate()
        {
            assert_eq!(pedido, pedidos_expected[i]);
        }
    }

    #[test]
    fn pedidos_mal_descartados_test() {
        let pedidos_archivo = vec![vec![-4, 3, 4, 5], vec![5, -5, 2, 4], vec![4, 5, 2, 3]];
        let pedidos = pedidos(pedidos_archivo);

        let pedidos_expected = vec![Pedido {
            cafe_molido: 4,
            agua_caliente: 5,
            cacao: 2,
            espuma: 3,
        }];
        for (i, pedido) in pedidos
            .expect("Error al obtener los pedidos")
            .into_iter()
            .enumerate()
        {
            assert_eq!(pedido, pedidos_expected[i]);
        }
    }

    #[test]
    fn todos_pedidos_invalidos_test() {
        let pedidos_archivo = vec![
            vec![-4, 3, 4, 5],
            vec![5, -5, 2, 4],
            vec![4, 5, MAX_CACAO_POR_PEDIDO + 1, 3],
        ];
        let pedidos = pedidos(pedidos_archivo);

        let expected = Err(PedidoError::NoHayPedidos);

        assert_eq!(pedidos, expected);
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
