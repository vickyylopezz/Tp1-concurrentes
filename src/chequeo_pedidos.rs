use crate::{
    constantes::{
        MAX_AGUA_POR_PEDIDO, MAX_CACAO_POR_PEDIDO, MAX_CAFE_POR_PEDIDO, MAX_ESPUMA_POR_PEDIDO,
        MIN_CANTIDAD_POR_PEDIDO,
    },
    error::PedidoError,
    pedido::Pedido,
};

/// Chequea que la cantidad de cafe del pedido sea valida y se encuentre dentro del minimo y del maximo permitido
pub fn cafe_invalido(cantidad_cafe: i32, i: usize) -> bool {
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
pub fn agua_invalida(cantidad_agua: i32, i: usize) -> bool {
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
pub fn cacao_invalido(cantidad_cacao: i32, i: usize) -> bool {
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
pub fn espuma_invalida(cantidad_espuma: i32, i: usize) -> bool {
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
pub fn pedidos(pedidos_archivo: Vec<Vec<i32>>) -> Result<Vec<Pedido>, PedidoError> {
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
    if pedidos.is_empty() {
        println!("No hay pedidos para procesar");
        return Err(PedidoError::NoHayPedidos);
    }
    Ok(pedidos)
}
