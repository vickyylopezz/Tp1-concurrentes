use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

use crate::constantes::{G, M, TIEMPO_CAFE_REPONER, X};

#[derive(Debug)]
pub struct ContenedorCafe {
    /// Cantidad de cafe en granos disponible
    pub cafe_granos: i32,
    /// Cantidad de cafe molido disponible
    pub cafe_molido: i32,
    /// Cantidad de cafe en granos consumido
    pub cafe_granos_consumido: i32,
    /// Cantidad de cafe molido consumido
    pub cafe_molido_consumido: i32,
    /// Indica si es necesario recargar el contenedor de cafe molido
    pub necesito_cafe: bool,
}

impl ContenedorCafe {
    pub fn new() -> Self {
        ContenedorCafe {
            cafe_granos: G,
            cafe_molido: M,
            cafe_granos_consumido: 0,
            cafe_molido_consumido: 0,
            necesito_cafe: false,
        }
    }
}

impl Default for ContenedorCafe {
    fn default() -> Self {
        Self::new()
    }
}

/// Rellena el contenedor de cafe consumiendo el cafe en granos
pub fn rellenar_contenedor_cafe(
    cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    fin_pedidos_cafe: Arc<AtomicBool>,
) {
    let (cafe_lock, cafe_cvar) = &*cafe;
    loop {
        if let Ok(mut cafe_mut) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont_cafe| {
            !cont_cafe.necesito_cafe && !fin_pedidos_cafe.load(Ordering::SeqCst)
        }) {
            if fin_pedidos_cafe.load(Ordering::SeqCst) {
                break;
            }

            if cafe_mut.cafe_granos >= M {
                println!("Recargando cafe molido");
                thread::sleep(Duration::from_millis(TIEMPO_CAFE_REPONER));
                cafe_mut.cafe_granos -= M - cafe_mut.cafe_molido;
                cafe_mut.cafe_granos_consumido += M - cafe_mut.cafe_molido;
                cafe_mut.cafe_molido = M;
                cafe_mut.necesito_cafe = false;
            } else {
                println!("Recargando cafe molido");
                thread::sleep(Duration::from_millis(TIEMPO_CAFE_REPONER));
                cafe_mut.cafe_molido += cafe_mut.cafe_granos;
                cafe_mut.cafe_granos_consumido += cafe_mut.cafe_granos;
                cafe_mut.cafe_granos = 0;
                cafe_mut.necesito_cafe = false;
                println!("Contenedor de cafe en granos vacio");
                cafe_cvar.notify_all();
                break;
            }

            if cafe_mut.cafe_granos <= G * X / 100 {
                println!("Cafe en granos por debajo del {}%", X);
            }
        }
        cafe_cvar.notify_one();
    }
}
