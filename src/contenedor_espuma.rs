use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::constantes::{E, L, MAX_ESPUMA_POR_PEDIDO, TIEMPO_ESPUMA_REPONER, X};

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
    /// Indica si ya se terminaron de realizar todos los pedidos
    pub fin_pedidos: bool,
}
impl ContenedorEspuma {
    pub fn new() -> Self {
        ContenedorEspuma {
            espuma: E,
            leche: L,
            espuma_consumida: 0,
            leche_consumida: 0,
            fin_pedidos: false,
        }
    }
}

impl Default for ContenedorEspuma {
    fn default() -> Self {
        Self::new()
    }
}

/// Rellena el contenedor de espuma consumiendo el contenedor de leche
pub fn rellenar_contenedor_espuma(espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>) {
    let (espuma_lock, espuma_cvar) = &*espuma;
    loop {
        if let Ok(mut espuma_mut) = espuma_cvar
            .wait_while(espuma_lock.lock().unwrap(), |cont_espuma| {
                cont_espuma.espuma >= MAX_ESPUMA_POR_PEDIDO && !cont_espuma.fin_pedidos
            })
        {
            if espuma_mut.fin_pedidos {
                break;
            }

            if espuma_mut.leche >= E {
                println!("Recargando espuma");
                thread::sleep(Duration::from_millis(TIEMPO_ESPUMA_REPONER));
                espuma_mut.leche -= E - espuma_mut.espuma;
                espuma_mut.leche_consumida += E - espuma_mut.espuma;
                espuma_mut.espuma = E;
            } else {
                println!("Recargando espuma");
                thread::sleep(Duration::from_millis(TIEMPO_ESPUMA_REPONER));
                espuma_mut.espuma += espuma_mut.leche;
                espuma_mut.leche_consumida += espuma_mut.leche;
                espuma_mut.leche = 0;
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
            espuma_mut.espuma = 0;
        }

        let thread_espuma = thread::spawn(move || {
            rellenar_contenedor_espuma(espuma_clone);
        });

        if let Ok(mut espuma_mut) = espuma_cvar.wait(espuma_lock.lock().unwrap()) {
            espuma_mut.fin_pedidos = true;
            espuma_cvar.notify_all();
            assert_eq!(espuma_mut.espuma, E);
            assert_eq!(espuma_mut.leche, L - E);
            assert_eq!(espuma_mut.leche_consumida, E);
        };
        thread_espuma.join().expect("Error join thread espuma");
    }

    #[test]
    fn relleno_contenedor_espuma_poca_leche_test() {
        let espuma = Arc::new((Mutex::new(ContenedorEspuma::new()), Condvar::new()));
        let espuma_clone = espuma.clone();

        let (espuma_lock, espuma_cvar) = &*espuma;
        if let Ok(mut espuma_mut) = espuma_lock.lock() {
            espuma_mut.espuma = 0;
            espuma_mut.leche = E - 1;
        }

        let thread_espuma = thread::spawn(move || {
            rellenar_contenedor_espuma(espuma_clone);
        });

        if let Ok(mut espuma_mut) = espuma_cvar.wait(espuma_lock.lock().unwrap()) {
            espuma_mut.fin_pedidos = true;
            espuma_cvar.notify_all();
            assert_eq!(espuma_mut.espuma, E - 1);
            assert_eq!(espuma_mut.leche, 0);
            assert_eq!(espuma_mut.leche_consumida, E - 1);
        };
        thread_espuma.join().expect("Error join thread espuma");
    }
}
