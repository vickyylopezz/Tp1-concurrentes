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
    espuma_lock: &Mutex<ContenedorEspuma>,
    espuma_cvar: &Condvar,
    fin_pedidos_espuma: Arc<AtomicBool>,
) {
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
                espuma_mut.espuma = E;
                espuma_mut.leche_consumida += E - espuma_mut.espuma;
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
