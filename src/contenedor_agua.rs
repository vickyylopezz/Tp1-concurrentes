use crate::constantes::{A, MAX_AGUA_POR_PEDIDO, TIEMPO_AGUA_REPONER};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

#[derive(Debug)]
pub struct ContenedorAgua {
    /// Cantidad de agua caliente disponible
    pub agua_caliente: i32,
    /// Cantidad de agua caliente consumida
    pub agua_caliente_consumida: i32,
}

impl ContenedorAgua {
    pub fn new() -> Self {
        ContenedorAgua {
            agua_caliente: A,
            agua_caliente_consumida: 0,
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
            cont_agua.agua_caliente >= MAX_AGUA_POR_PEDIDO
                && !fin_pedidos_agua.load(Ordering::SeqCst)
        }) {
            if fin_pedidos_agua.load(Ordering::SeqCst) {
                break;
            }

            println!("Recargando agua caliente");
            agua_mut.agua_caliente = A;
            thread::sleep(Duration::from_millis(TIEMPO_AGUA_REPONER));
        }
        agua_cvar.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relleno_contenedor_agua_test() {
        let agua = Arc::new((Mutex::new(ContenedorAgua::new()), Condvar::new()));
        let agua_clone = agua.clone();

        let (agua_lock, agua_cvar) = &*agua;
        if let Ok(mut agua_mut) = agua_lock.lock() {
            agua_mut.agua_caliente = 0;
        }

        let fin_pedidos = Arc::new(AtomicBool::new(false));
        let fin_pedidos_clone = fin_pedidos.clone();

        let thread_agua = thread::spawn(move || {
            rellenar_contenedor_agua(agua_clone, fin_pedidos_clone);
        });

        if let Ok(agua_mut) = agua_cvar.wait(agua_lock.lock().unwrap()) {
            fin_pedidos.store(true, Ordering::SeqCst);
            agua_cvar.notify_all();
            assert_eq!(agua_mut.agua_caliente, A);
        };
        thread_agua.join().expect("Error join thread agua");
    }
}
