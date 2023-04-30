use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

use crate::constantes::{A, TIEMPO_AGUA_REPONER};

#[derive(Debug)]
pub struct ContenedorAgua {
    /// Cantidad de agua caliente disponible
    pub agua_caliente: i32,
    /// Cantidad de agua caliente consumida
    pub agua_caliente_consumida: i32,
    /// Indica si es necesario recargar el contenedor de agua
    pub necesito_agua: bool,
}

impl ContenedorAgua {
    pub fn new() -> Self {
        ContenedorAgua {
            agua_caliente: A,
            agua_caliente_consumida: 0,
            necesito_agua: false,
        }
    }
}

impl Default for ContenedorAgua {
    fn default() -> Self {
        Self::new()
    }
}

/// Rellena el contenedor de agua consumiendo el agua de la red
pub fn rellenar_contenedor_agua(
    agua: Arc<(Mutex<ContenedorAgua>, Condvar)>,
    fin_pedidos_agua: Arc<AtomicBool>,
) {
    let (agua_lock, agua_cvar) = &*agua;
    loop {
        if let Ok(mut agua_mut) = agua_cvar.wait_while(agua_lock.lock().unwrap(), |cont_agua| {
            !cont_agua.necesito_agua && !fin_pedidos_agua.load(Ordering::SeqCst)
        }) {
            if fin_pedidos_agua.load(Ordering::SeqCst) {
                break;
            }

            println!("Recargando agua caliente");
            agua_mut.agua_caliente = A;
            thread::sleep(Duration::from_millis(TIEMPO_AGUA_REPONER));
            agua_mut.necesito_agua = false;
        }
        agua_cvar.notify_one();
    }
}
