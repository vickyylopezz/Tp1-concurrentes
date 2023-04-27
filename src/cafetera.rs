use crate::pedido::Pedido;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std_semaphore::Semaphore;
use tp1::constantes::{M, N, TIEMPO_CAFE_REPONER, TIEMPO_RECURSO_UNIDAD, X, A, TIEMPO_AGUA_REPONER};
use tp1::contenedores::{ContenedorAgua, ContenedorCacao, ContenedorCafe, ContenedorEspuma, self};
use tp1::error::CafeteraError;

pub struct Cafetera {
    dispensadores_semaforo: Arc<Semaphore>,
    dispensadores: Arc<RwLock<Vec<bool>>>,
    contenedor_cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    contenedor_agua: Arc<(Mutex<ContenedorAgua>, Condvar)>,
    contenedor_cacao: Arc<(Mutex<ContenedorCacao>, Condvar)>,
    contenedor_espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    fin_pedidos: Arc<AtomicBool>,
}

impl Cafetera {
    pub fn new() -> Self {
        Cafetera {
            dispensadores_semaforo: Arc::new(Semaphore::new(N as isize)),
            dispensadores: Arc::new(RwLock::new(vec![false; N as usize])),
            contenedor_cafe: Arc::new((Mutex::new(ContenedorCafe::new()), Condvar::new())),
            contenedor_agua: Arc::new((Mutex::new(ContenedorAgua::new()), Condvar::new())),
            contenedor_cacao: Arc::new((Mutex::new(ContenedorCacao::new()), Condvar::new())),
            contenedor_espuma: Arc::new((Mutex::new(ContenedorEspuma::new()), Condvar::new())),
            fin_pedidos: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Prepara los pedidos recibidos
    pub fn preparar_pedidos(&self, pedidos: Vec<Pedido>) {
        println!("{:?}", pedidos);
        let mut pedidos_handle = vec![];

        let cafe = self.contenedor_cafe.clone();
        let agua = self.contenedor_agua.clone();
        let cacao = self.contenedor_cacao.clone();
        let espuma = self.contenedor_espuma.clone();

        let thread_rellenar = rellenar_contenedores(cafe, agua, cacao, espuma, &self.fin_pedidos);
        
        for id in 0..pedidos.len() {
            let semaforo_clone = self.dispensadores_semaforo.clone();
            let dispensadores_clone = self.dispensadores.clone();
            let pedidos_clone = pedidos.clone();

            let cafe_pedido = self.contenedor_cafe.clone();
            let agua_pedido = self.contenedor_agua.clone();
            let cacao_pedido = self.contenedor_cacao.clone();
            let espuma_pedido = self.contenedor_espuma.clone();

            pedidos_handle.push(thread::spawn(move || {
                pedido(
                    id as i32,
                    semaforo_clone,
                    dispensadores_clone,
                    pedidos_clone[id].clone(),
                    cafe_pedido,
                    agua_pedido,
                    cacao_pedido,
                    espuma_pedido,
                );
            }));
        }

        for pedido in pedidos_handle {
            pedido
                .join()
                .expect("Error al hacer join al thread del pedido");
        }

        println!("Terminaron todos los pedidos");
        self.fin_pedidos.store(true, Ordering::SeqCst);
        let (_, cafe_cvar) = &*self.contenedor_cafe;
        cafe_cvar.notify_all();
        let (_, agua_cvar) = &*self.contenedor_agua;
        agua_cvar.notify_all();

        if let Ok(contenedores) = thread_rellenar {
            for contenedor in contenedores {
                contenedor.join()
                .expect("Error al hacer join al thread de rellenar cafe")
            }
        }
        
    }
}

fn rellenar_contenedores(
    cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    agua: Arc<(Mutex<ContenedorAgua>, Condvar)>,
    _cacao: Arc<(Mutex<ContenedorCacao>, Condvar)>,
    _espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    fin_pedidos: &Arc<AtomicBool>,
) -> Result<Vec<JoinHandle<()>>, CafeteraError> {
    let fin_pedidos_cafe = fin_pedidos.clone();
    let cafe_thread = thread::spawn(move || {
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
                    cafe_mut.cafe_molido = M;
                    cafe_mut.necesito_cafe = false;
                } else {
                    println!("Recargando cafe molido");
                    thread::sleep(Duration::from_millis(TIEMPO_CAFE_REPONER));
                    cafe_mut.cafe_molido += cafe_mut.cafe_granos;
                    cafe_mut.cafe_granos = 0;
                    cafe_mut.necesito_cafe = false;
                    cafe_mut.vacio = true;
                    println!("Contenedor de cafe en granos vacio");
                    cafe_cvar.notify_all();
                    break;
                }

                if cafe_mut.cafe_granos <= M * X / 100 {
                    println!("Cafe en granos por debajo del {}%", X);
                }
            }
            cafe_cvar.notify_one();
        }
        println!("Fin thread rellenar cafe");
    });

    let fin_pedidos_agua = fin_pedidos.clone();
    let agua_thread = thread::spawn(move || {
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
        println!("Fin thread rellenar agua");
    });

    let threads = vec![cafe_thread, agua_thread];
    Ok(threads)
}

/// Se le asigna un dispensador al pedido y se lo prepara
fn pedido(
    id: i32,
    sem: Arc<Semaphore>,
    dispensadores: Arc<RwLock<Vec<bool>>>,
    pedido: Pedido,
    cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    agua: Arc<(Mutex<ContenedorAgua>, Condvar)>,
    _cacao: Arc<(Mutex<ContenedorCacao>, Condvar)>,
    _espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
) -> Result<(), CafeteraError> {
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

    let (cafe_lock, cafe_cvar) = &*cafe;
    if let Ok(mut cafe_mut) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont_cafe| {
        cont_cafe.necesito_cafe = true;
        cafe_cvar.notify_all();
        (cont_cafe.cafe_molido < pedido.cafe_molido) && (!cont_cafe.vacio)
    }) {
        cafe_mut.necesito_cafe = false;
        if cafe_mut.vacio && cafe_mut.cafe_molido < pedido.cafe_molido {
            println!(
                "[Pedido {}] Contenedor de cafe en granos vacio y no me alcanza el cafe molido",
                id
            );
            cafe_cvar.notify_all();
            return Err(CafeteraError::CafeInsuficiente);
        }
        cafe_mut.necesito_cafe = false;
        cafe_mut.cafe_molido_consumido += pedido.cafe_molido;
        cafe_mut.cafe_molido -= pedido.cafe_molido;
        println!("[Pedido {}] sirviendo cafe", id);
        thread::sleep(Duration::from_millis(
            TIEMPO_RECURSO_UNIDAD * pedido.cafe_molido as u64,
        ));

        cafe_cvar.notify_all();
    }

    let (agua_lock, agua_cvar) = &*agua;
    if let Ok(mut agua_mut) = agua_cvar.wait_while(agua_lock.lock().unwrap(), |cont_agua| {
        cont_agua.necesito_agua = true;
        agua_cvar.notify_all();
        cont_agua.agua_caliente < pedido.agua_caliente
    }) {
        agua_mut.necesito_agua = false;


        agua_mut.agua_caliente_consumida += pedido.agua_caliente;
        agua_mut.agua_caliente -= pedido.agua_caliente;
        println!("[Pedido {}] sirviendo agua", id);
        thread::sleep(Duration::from_millis(TIEMPO_RECURSO_UNIDAD*pedido.agua_caliente as u64));
        agua_cvar.notify_all();
    }

    /*let (cacao_lock, cacao_cvar) = &*cacao;
    if let Ok(mut cacao_mut) = cacao_cvar.wait_while(cacao_lock.lock().unwrap(), |cont_cacao| {
        cont_cacao.cacao < pedido.cacao
    }) {
        println!("[Pedido {}] sirviendo cacao", id);
        cacao_mut.cacao_consumido += pedido.cacao;
        cacao_mut.cacao -= pedido.cacao;
        thread::sleep(Duration::from_millis(TIEMPO_RECURSO_UNIDAD*pedido.cacao as u64));
        cacao_cvar.notify_all();
    } else {
        println!("[Pedido {}] cacao insuficiente, no se puede terminar el pedido", id);
        return Err(CafeteraError::CacaoInsuficiente)
    }

    let (espuma_lock, espuma_cvar) = &*espuma;
    if let Ok(mut espuma_mut) = espuma_cvar.wait_while(espuma_lock.lock().unwrap(), |cont_espuma| {
        cont_espuma.espuma < pedido.espuma
    }) {
        println!("[Pedido {}] sirviendo espuma", id);
        espuma_mut.espuma_consumida += pedido.espuma;
        espuma_mut.espuma -= pedido.espuma;
        thread::sleep(Duration::from_millis(TIEMPO_RECURSO_UNIDAD*pedido.espuma as u64));
        espuma_cvar.notify_all()
    } else if let Ok(mut espuma_mut) = espuma_lock.lock() {
        println!("[Pedido {}] espuma insuficiente, recargando", id);
        thread::sleep(Duration::from_millis(TIEMPO_ESPUMA_REPONER as u64));
        espuma_mut.leche -= M - espuma_mut.espuma;
        espuma_mut.espuma = A;
    }*/

    /*while !cafe_servido & !agua_servida{
        if !cafe_servido {
            if let Ok(mut cafe_mut) = cafe.write(){
                if cafe_mut.cafe_molido >= pedido.cafe_molido {
                    println!("[Pedido {}] sirviendo cafe", id);
                    cafe_mut.cafe_molido_consumido += pedido.cafe_molido;
                    cafe_mut.cafe_molido -= pedido.cafe_molido;
                    thread::sleep(Duration::from_millis(TIEMPO_RECURSO_UNIDAD*pedido.cafe_molido as u64));
                    cafe_servido = true
                } else {
                    println!("[Pedido {}] cafe molido insuficiente, recargando", id);
                    thread::sleep(Duration::from_millis(TIEMPO_CAFE_REPONER));
                    cafe_mut.cafe_granos -= M - cafe_mut.cafe_molido;
                    cafe_mut.cafe_molido = M;
                }
            }
        }

        if !agua_servida {
            if let Ok(mut agua_mut) = agua.write(){
                if agua_mut.agua_caliente >= pedido.agua_caliente {
                    println!("[Pedido {}] sirviendo agua", id);
                    agua_mut.agua_caliente_consumida += pedido.agua_caliente;
                    agua_mut.agua_caliente -= pedido.agua_caliente;
                    thread::sleep(Duration::from_millis(TIEMPO_RECURSO_UNIDAD*pedido.agua_caliente as u64));
                    agua_servida = true
                } else {
                    println!("[Pedido {}] cafe molido insuficiente, recargando", id);
                    thread::sleep(Duration::from_millis(TIEMPO_AGUA_REPONER as u64));
                    agua_mut.agua_caliente = A;
                }
            }
        }

    }*/

    if let Ok(mut dispensadores_mut) = dispensadores.write() {
        dispensadores_mut[num_dispensador as usize] = false;
    }
    println!("[Pedido {}] terminé", id);

    Ok(())
}
