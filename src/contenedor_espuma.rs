use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

use crate::constantes::{E, L, TIEMPO_ESPUMA_REPONER, X};

#[derive(Debug)]
pub struct ContenedorEspuma {
    /// Cantidad de espuma disponible
    pub espuma: i32,
    /// Cantidad de leche disponible
    pub leche: i32,
    /// Cantidad de espuma consumida
    pub espuma_consumida: i32,
    /// Cantidad de leche consumida
    pub leche_consumida: i32,
    /// Indica si es necesario recargar el contenedor de espuma
    pub necesito_espuma: bool,
}
impl ContenedorEspuma {
    pub fn new() -> Self {
        ContenedorEspuma {
            espuma: E,
            leche: L,
            espuma_consumida: 0,
            leche_consumida: 0,
            necesito_espuma: false,
        }
    }
}

impl Default for ContenedorEspuma {
    fn default() -> Self {
        Self::new()
    }
}

/// Rellena el contenedor de espuma consumiendo el contenedor de leche
pub fn rellenar_contenedor_espuma(
    espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    fin_pedidos_espuma: Arc<AtomicBool>,
) {
    let (espuma_lock, espuma_cvar) = &*espuma;
    loop {
        if let Ok(mut espuma_mut) =
            espuma_cvar.wait_while(espuma_lock.lock().unwrap(), |cont_espuma| {
                !cont_espuma.necesito_espuma && !fin_pedidos_espuma.load(Ordering::SeqCst)
            })
        {
            if fin_pedidos_espuma.load(Ordering::SeqCst) {
                break;
            }

            if espuma_mut.leche >= E {
                println!("Recargando espuma");
                thread::sleep(Duration::from_millis(TIEMPO_ESPUMA_REPONER));
                espuma_mut.leche -= E - espuma_mut.espuma;
                espuma_mut.leche_consumida += E - espuma_mut.espuma;
                espuma_mut.espuma = E;
                espuma_mut.necesito_espuma = false;
            } else {
                println!("Recargando espuma");
                thread::sleep(Duration::from_millis(TIEMPO_ESPUMA_REPONER));
                espuma_mut.espuma += espuma_mut.leche;
                espuma_mut.leche_consumida += espuma_mut.leche;
                espuma_mut.leche = 0;
                espuma_mut.necesito_espuma = false;
                println!("Contenedor de leche vacio");
                espuma_cvar.notify_all();
                break;
            }

            if espuma_mut.leche <= L * X / 100 {
                println!("Leche por debajo del {}%", X);
            }
        }
        espuma_cvar.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relleno_contenedor_espuma_test() {
        let espuma = Arc::new((Mutex::new(ContenedorEspuma::new()), Condvar::new()));
        let espuma_clone = espuma.clone();

        let (espuma_lock, espuma_cvar) = &*espuma;
        if let Ok(mut espuma_mut) = espuma_lock.lock() {
            espuma_mut.necesito_espuma = true;
            espuma_mut.espuma = 0;
        }

        let fin_pedidos = Arc::new(AtomicBool::new(false));
        let fin_pedidos_clone = fin_pedidos.clone();

        let thread_espuma = thread::spawn(move || {
            rellenar_contenedor_espuma(espuma_clone, fin_pedidos_clone);
        });

        if let Ok(espuma_mut) = espuma_cvar.wait(espuma_lock.lock().unwrap()) {
            fin_pedidos.store(true, Ordering::SeqCst);
            espuma_cvar.notify_all();
            assert_eq!(espuma_mut.espuma, E);
            assert_eq!(espuma_mut.leche, L - E);
            assert_eq!(espuma_mut.leche_consumida, E);
            assert_eq!(espuma_mut.necesito_espuma, false);
        };
        thread_espuma.join().expect("Error join thread espuma");
    }

    #[test]
    fn relleno_contenedor_espuma_poca_leche_test() {
        let espuma = Arc::new((Mutex::new(ContenedorEspuma::new()), Condvar::new()));
        let espuma_clone = espuma.clone();

        let (espuma_lock, espuma_cvar) = &*espuma;
        if let Ok(mut espuma_mut) = espuma_lock.lock() {
            espuma_mut.necesito_espuma = true;
            espuma_mut.espuma = 0;
            espuma_mut.leche = E - 1;
        }

        let fin_pedidos = Arc::new(AtomicBool::new(false));
        let fin_pedidos_clone = fin_pedidos.clone();

        let thread_espuma = thread::spawn(move || {
            rellenar_contenedor_espuma(espuma_clone, fin_pedidos_clone);
        });

        if let Ok(espuma_mut) = espuma_cvar.wait(espuma_lock.lock().unwrap()) {
            fin_pedidos.store(true, Ordering::SeqCst);
            espuma_cvar.notify_all();
            assert_eq!(espuma_mut.espuma, E - 1);
            assert_eq!(espuma_mut.leche, 0);
            assert_eq!(espuma_mut.leche_consumida, E - 1);
            assert_eq!(espuma_mut.necesito_espuma, false);
        };
        thread_espuma.join().expect("Error join thread espuma");
    }
}
