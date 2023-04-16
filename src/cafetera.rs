use crate::pedido::Pedido;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;

use tp1::constantes::N;
//use tp1::contenedores::ContenedorAgua;
//use tp1::contenedores::ContenedorCacao;
//use tp1::contenedores::ContenedorCafe;
//use tp1::contenedores::ContenedorEspuma;

pub struct Cafetera {
    dispensadores_semaforo: Arc<Semaphore>,
    dispensadores: Arc<RwLock<Vec<bool>>>,
    //contenedor_cafe: Arc<Mutex<ContenedorCafe>>,
    //contenedor_agua: Arc<Mutex<ContenedorAgua>>,
    //contenedor_cacao: Arc<Mutex<ContenedorCacao>>,
    //contenedor_espuma: Arc<Mutex<ContenedorEspuma>>
}

impl Cafetera {
    pub fn new() -> Self {
        Cafetera {
            dispensadores_semaforo: Arc::new(Semaphore::new(N as isize)),
            dispensadores: Arc::new(RwLock::new(vec![false; N as usize])),
            //contenedor_cafe: Arc::new(Mutex::new(ContenedorCafe::new())),
            //contenedor_agua: Arc::new(Mutex::new(ContenedorAgua::new())),
            //contenedor_cacao: Arc::new(Mutex::new(ContenedorCacao::new())),
            //contenedor_espuma: Arc::new(Mutex::new(ContenedorEspuma::new())),
        }
    }

    pub fn preparar_pedidos(self, pedidos: Vec<Pedido>) {
        println!("{:?}", pedidos);
        let mut pedidos_handle = vec![];
        for id in 0..pedidos.len() {
            let semaforo_clone = self.dispensadores_semaforo.clone();
            let dispensadores_clone = self.dispensadores.clone();
            pedidos_handle.push(thread::spawn(move || {
                pedido(id as i32, semaforo_clone, dispensadores_clone)
            }))
        }

        for pedido in pedidos_handle {
            pedido.join().expect("Error al hacer join al thread del pedido");;
        }
    }
}

fn pedido(id: i32, sem: Arc<Semaphore>, dispensadores: Arc<RwLock<Vec<bool>>>) {
    let _access = sem.access();
    let mut num_dispensador: i32 = -1;
    if let Ok(mut dispensadores_mut) = dispensadores.write() {
        for i in 0..dispensadores_mut.len() {
            if !dispensadores_mut[i] {
                num_dispensador = i as i32;
                dispensadores_mut[i] = true;
                break;
            }
        }
    }
    println!("[Pedido {}] usando dispensador {}", id, num_dispensador);
    //usar dispensador
    thread::sleep(Duration::from_millis(thread_rng().gen_range(5000, 10000)));
    if let Ok(mut dispensadores_mut) = dispensadores.write() {
        dispensadores_mut[num_dispensador as usize] = false;
    }
    println!("[Pedido {}] terminé", id);
}
