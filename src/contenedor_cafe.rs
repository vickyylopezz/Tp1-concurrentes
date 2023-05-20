use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::constantes::{G, M, MAX_CAFE_POR_PEDIDO, TIEMPO_CAFE_REPONER, X};

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
    /// Indica si ya se terminaron de realizar todos los pedidos
    pub fin_pedidos: bool,
}

impl ContenedorCafe {
    pub fn new() -> Self {
        ContenedorCafe {
            cafe_granos: G,
            cafe_molido: M,
            cafe_granos_consumido: 0,
            cafe_molido_consumido: 0,
            fin_pedidos: false,
        }
    }
}

impl Default for ContenedorCafe {
    fn default() -> Self {
        Self::new()
    }
}

/// Rellena el contenedor de cafe consumiendo el cafe en granos
pub fn rellenar_contenedor_cafe(cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>) {
    let (cafe_lock, cafe_cvar) = &*cafe;
    loop {
        if let Ok(mut cafe_mut) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont_cafe| {
            cont_cafe.cafe_molido >= MAX_CAFE_POR_PEDIDO && !cont_cafe.fin_pedidos
        }) {
            if cafe_mut.fin_pedidos {
                break;
            }

            if cafe_mut.cafe_granos >= M {
                println!("Recargando cafe molido");
                thread::sleep(Duration::from_millis(TIEMPO_CAFE_REPONER));
                cafe_mut.cafe_granos -= M - cafe_mut.cafe_molido;
                cafe_mut.cafe_granos_consumido += M - cafe_mut.cafe_molido;
                cafe_mut.cafe_molido = M;
            } else {
                println!("Recargando cafe molido");
                thread::sleep(Duration::from_millis(TIEMPO_CAFE_REPONER));
                cafe_mut.cafe_molido += cafe_mut.cafe_granos;
                cafe_mut.cafe_granos_consumido += cafe_mut.cafe_granos;
                cafe_mut.cafe_granos = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relleno_contenedor_cafe_test() {
        let cafe = Arc::new((Mutex::new(ContenedorCafe::new()), Condvar::new()));
        let cafe_clone = cafe.clone();

        let (cafe_lock, cafe_cvar) = &*cafe;
        if let Ok(mut cafe_mut) = cafe_lock.lock() {
            cafe_mut.cafe_molido = 0;
        }

        let thread_cafe = thread::spawn(move || {
            rellenar_contenedor_cafe(cafe_clone);
        });

        if let Ok(mut cafe_mut) = cafe_cvar.wait(cafe_lock.lock().unwrap()) {
            cafe_mut.fin_pedidos = true;
            cafe_cvar.notify_all();
            assert_eq!(cafe_mut.cafe_molido, M);
            assert_eq!(cafe_mut.cafe_granos, G - M);
            assert_eq!(cafe_mut.cafe_granos_consumido, M);
        };
        thread_cafe.join().expect("Error join thread cafe");
    }

    #[test]
    fn relleno_contenedor_cafe_poco_cafe_granos_test() {
        let cafe = Arc::new((Mutex::new(ContenedorCafe::new()), Condvar::new()));
        let cafe_clone = cafe.clone();

        let (cafe_lock, cafe_cvar) = &*cafe;
        if let Ok(mut cafe_mut) = cafe_lock.lock() {
            cafe_mut.cafe_molido = 0;
            cafe_mut.cafe_granos = M - 1;
        }

        let thread_cafe = thread::spawn(move || {
            rellenar_contenedor_cafe(cafe_clone);
        });

        if let Ok(mut cafe_mut) = cafe_cvar.wait(cafe_lock.lock().unwrap()) {
            cafe_mut.fin_pedidos = true;
            cafe_cvar.notify_all();
            assert_eq!(cafe_mut.cafe_molido, M - 1);
            assert_eq!(cafe_mut.cafe_granos, 0);
            assert_eq!(cafe_mut.cafe_granos_consumido, M - 1);
        };
        thread_cafe.join().expect("Error join thread cafe");
    }
}
