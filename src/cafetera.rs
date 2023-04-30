use crate::pedido::Pedido;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std_semaphore::Semaphore;
use tp1::constantes::{
    A, C, E, G, L, M, MOSTRAR_ESTADISTICAS, N, TIEMPO_AGUA_REPONER, TIEMPO_CAFE_REPONER,
    TIEMPO_ESPUMA_REPONER, TIEMPO_RECURSO_UNIDAD, VACIO, X,
};
use tp1::contenedores::{ContenedorAgua, ContenedorCacao, ContenedorCafe, ContenedorEspuma};
use tp1::error::CafeteraError;

pub struct Cafetera {
    dispensadores_semaforo: Arc<Semaphore>,
    dispensadores: Arc<RwLock<Vec<bool>>>,
    contenedor_cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    contenedor_agua: Arc<(Mutex<ContenedorAgua>, Condvar)>,
    contenedor_cacao: Arc<Mutex<ContenedorCacao>>,
    contenedor_espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    fin_pedidos: Arc<AtomicBool>,
    pedidos_completados: Arc<Mutex<i32>>,
}

impl Cafetera {
    pub fn new() -> Self {
        Cafetera {
            dispensadores_semaforo: Arc::new(Semaphore::new(N as isize)),
            dispensadores: Arc::new(RwLock::new(vec![false; N as usize])),
            contenedor_cafe: Arc::new((Mutex::new(ContenedorCafe::new()), Condvar::new())),
            contenedor_agua: Arc::new((Mutex::new(ContenedorAgua::new()), Condvar::new())),
            contenedor_cacao: Arc::new(Mutex::new(ContenedorCacao::new())),
            contenedor_espuma: Arc::new((Mutex::new(ContenedorEspuma::new()), Condvar::new())),
            fin_pedidos: Arc::new(AtomicBool::new(false)),
            pedidos_completados: Arc::new(Mutex::new(0)),
        }
    }

    /// Prepara los pedidos recibidos
    pub fn preparar_pedidos(&self, pedidos: Vec<Pedido>) {
        println!("{:?}", pedidos);
        let mut pedidos_handle = vec![];

        let cafe = self.contenedor_cafe.clone();
        let agua = self.contenedor_agua.clone();
        let espuma = self.contenedor_espuma.clone();

        let thread_rellenar = rellenar_contenedores(cafe, agua, espuma, &self.fin_pedidos);

        let pedidos_com = self.pedidos_completados.clone();
        let thread_estadisticas =
            self.mostrar_estadisticas(pedidos.len(), pedidos_com, &self.fin_pedidos);

        for id in 0..pedidos.len() {
            let semaforo_clone = self.dispensadores_semaforo.clone();
            let dispensadores_clone = self.dispensadores.clone();
            let pedidos_clone = pedidos.clone();

            let cafe_pedido = self.contenedor_cafe.clone();
            let agua_pedido = self.contenedor_agua.clone();
            let cacao_pedido = self.contenedor_cacao.clone();
            let espuma_pedido = self.contenedor_espuma.clone();

            let pedidos_com = self.pedidos_completados.clone();

            pedidos_handle.push(thread::spawn(move || {
                if pedido(
                    id as i32,
                    semaforo_clone,
                    dispensadores_clone,
                    pedidos_clone[id].clone(),
                    cafe_pedido,
                    agua_pedido,
                    cacao_pedido,
                    espuma_pedido,
                )
                .is_ok()
                {
                    if let Ok(mut pedidos_compeltos) = pedidos_com.lock() {
                        *pedidos_compeltos += 1;
                    }
                }
            }));
        }

        for pedido in pedidos_handle {
            pedido
                .join()
                .expect("Error al hacer join al thread del pedido");
        }

        if let Ok(pedidos_completos) = self.pedidos_completados.lock() {
            println!(
                "Cantidad de pedidos completados: {} de {}",
                pedidos_completos,
                pedidos.len()
            );
            println!(
                "Cantidad de pedidos no completados: {} de {}",
                pedidos.len() as i32 - *pedidos_completos,
                pedidos.len()
            )
        }

        println!("Terminaron todos los pedidos");
        self.fin_pedidos.store(true, Ordering::SeqCst);
        let (_, cafe_cvar) = &*self.contenedor_cafe;
        cafe_cvar.notify_all();
        let (_, agua_cvar) = &*self.contenedor_agua;
        agua_cvar.notify_all();
        let (_, espuma_cvar) = &*self.contenedor_espuma;
        espuma_cvar.notify_all();

        if let Ok(contenedores) = thread_rellenar {
            for contenedor in contenedores {
                contenedor
                    .join()
                    .expect("Error al hacer join al thread de rellenar recurso")
            }
        }

        thread_estadisticas
            .join()
            .expect("Error al hacer join al thread de las estadisticas")
    }

    fn mostrar_estadisticas(
        &self,
        cant_pedidos_total: usize,
        cant_pedidos: Arc<Mutex<i32>>,
        fin_pedidos: &Arc<AtomicBool>,
    ) -> JoinHandle<()> {
        let contenedor_cafe = self.contenedor_cafe.clone();
        let contenedor_agua = self.contenedor_agua.clone();
        let contenedor_cacao = self.contenedor_cacao.clone();
        let contenedor_espuma = self.contenedor_espuma.clone();
        let fin_pedidos = fin_pedidos.clone();

        thread::spawn(move || loop {
            if fin_pedidos.load(Ordering::SeqCst) {
                break;
            }
            let mut cafe_molido = 0;
            let mut cafe_molido_consumido = 0;

            let mut cafe_granos = 0;
            let mut cafe_granos_consumido = 0;

            let mut agua_caliente = 0;
            let mut agua_caliente_consumida = 0;

            let mut cacao = 0;
            let mut cacao_consumido = 0;
            let mut espuma = 0;
            let mut leche = 0;
            let mut espuma_consumida = 0;
            let mut leche_consumida = 0;
            let (cafe_lock, _) = &*contenedor_cafe;
            if let Ok(cafe_mut) = cafe_lock.lock() {
                cafe_molido = cafe_mut.cafe_molido;
                cafe_granos = cafe_mut.cafe_granos;
                cafe_molido_consumido = cafe_mut.cafe_molido_consumido;
                cafe_granos_consumido = cafe_mut.cafe_granos_consumido;
            }
            let (agua_lock, _) = &*contenedor_agua;
            if let Ok(agua_mut) = agua_lock.lock() {
                agua_caliente = agua_mut.agua_caliente;
                agua_caliente_consumida = agua_mut.agua_caliente_consumida
            }

            if let Ok(cacao_mut) = contenedor_cacao.lock() {
                cacao = cacao_mut.cacao;
                cacao_consumido = cacao_mut.cacao_consumido;
            }
            let (espuma_lock, _) = &*contenedor_espuma;
            if let Ok(espuma_mut) = espuma_lock.lock() {
                espuma = espuma_mut.espuma;
                espuma_consumida = espuma_mut.espuma_consumida;
                leche = espuma_mut.leche;
                leche_consumida = espuma_mut.leche_consumida;
            }
            let mut pedidos_completados = 0;
            if let Ok(pedidos_compeltos) = cant_pedidos.lock() {
                pedidos_completados = *pedidos_compeltos;
            }

            println!("ESTADITICAS");
            println!("-------------------------------------");
            println!("Nivel contenedores -> cafe molido: {} de {}, cafe en granos: {} de {}, agua_caliente: {} de {}, cacao: {} de {} ,espuma: {} de {} y leche: {} de {}", cafe_molido, M, cafe_granos, G, agua_caliente, A, cacao, C, espuma, E, leche, L);
            println!("Consumido -> cafe_molido: {}, cafe granos: {}, agua caliente: {}, cacao: {}, espuma: {} y leche: {}", cafe_molido_consumido, cafe_granos_consumido, agua_caliente_consumida, cacao_consumido, espuma_consumida, leche_consumida);
            println!(
                "Cantidad de pedidos completados: {} de {}",
                pedidos_completados, cant_pedidos_total
            );
            println!("-------------------------------------");

            thread::sleep(Duration::from_millis(MOSTRAR_ESTADISTICAS));
        })
    }
}

/// Rellena el contenedor de cafe consumiendo el cafe en granos
fn rellenar_contenedor_cafe(cafe_lock: &Mutex<ContenedorCafe>, cafe_cvar: &Condvar, fin_pedidos_cafe: Arc<AtomicBool>) {
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

            if cafe_mut.cafe_granos <= M * X / 100 {
                println!("Cafe en granos por debajo del {}%", X);
            }
        }
        cafe_cvar.notify_one();
    }
}

/// Rellena el contenedor de agua consumiendo el agua de la red
fn rellenar_contenedor_agua(agua_lock: &Mutex<ContenedorAgua>, agua_cvar: &Condvar, fin_pedidos_agua: Arc<AtomicBool>){
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

/// Rellena el contenedor de espuma consumiendo el contenedor de leche
fn rellenar_contenedor_espuma(espuma_lock: &Mutex<ContenedorEspuma>, espuma_cvar: &Condvar, fin_pedidos_espuma: Arc<AtomicBool>) {
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

/// Rellena los contenedores de los recursos necesarios en los pedidos
fn rellenar_contenedores(
    cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    agua: Arc<(Mutex<ContenedorAgua>, Condvar)>,
    espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    fin_pedidos: &Arc<AtomicBool>,
) -> Result<Vec<JoinHandle<()>>, CafeteraError> {
    let fin_pedidos_cafe = fin_pedidos.clone();
    let cafe_thread = thread::spawn(move || {
        let (cafe_lock, cafe_cvar) = &*cafe;
        rellenar_contenedor_cafe(cafe_lock, cafe_cvar, fin_pedidos_cafe);
        println!("Fin thread rellenar cafe");
    });

    let fin_pedidos_agua = fin_pedidos.clone();
    let agua_thread = thread::spawn(move || {
        let (agua_lock, agua_cvar) = &*agua;
        rellenar_contenedor_agua(agua_lock, agua_cvar, fin_pedidos_agua);
        println!("Fin thread rellenar agua");
    });

    let fin_pedidos_espuma = fin_pedidos.clone();
    let espuma_thread = thread::spawn(move || {
        let (espuma_lock, espuma_cvar) = &*espuma;
        rellenar_contenedor_espuma(espuma_lock, espuma_cvar, fin_pedidos_espuma);
        println!("Fin thread rellenar espuma");
    });

    let threads = vec![cafe_thread, agua_thread, espuma_thread];
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
    cacao: Arc<Mutex<ContenedorCacao>>,
    espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
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
        (cont_cafe.cafe_molido < pedido.cafe_molido) && (cont_cafe.cafe_granos != VACIO)
    }) {
        cafe_mut.necesito_cafe = false;
        if cafe_mut.cafe_granos == VACIO && cafe_mut.cafe_molido < pedido.cafe_molido {
            println!(
                "[Pedido {}] Contenedor de cafe en granos vacio y no me alcanza el cafe molido",
                id
            );
            cafe_cvar.notify_all();
            if let Ok(mut dispensadores_mut) = dispensadores.write() {
                dispensadores_mut[num_dispensador as usize] = false;
            }
            return Err(CafeteraError::CafeInsuficiente);
        }
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
        thread::sleep(Duration::from_millis(
            TIEMPO_RECURSO_UNIDAD * pedido.agua_caliente as u64,
        ));
        agua_cvar.notify_all();
    }

    if let Ok(mut cacao_mut) = cacao.lock() {
        if cacao_mut.cacao < pedido.cacao {
            println!("[Pedido {}] No me alcanza el cacao", id);
            if let Ok(mut dispensadores_mut) = dispensadores.write() {
                dispensadores_mut[num_dispensador as usize] = false;
            }
            return Err(CafeteraError::CacaoInsuficiente);
        }

        println!("[Pedido {}] sirviendo cacao", id);
        cacao_mut.cacao_consumido += pedido.cacao;
        cacao_mut.cacao -= pedido.cacao;
        thread::sleep(Duration::from_millis(
            TIEMPO_RECURSO_UNIDAD * pedido.cacao as u64,
        ));
        if cacao_mut.cacao <= C * X / 100 {
            println!("Cacao por debajo del {}%", X);
        }
    }

    let (espuma_lock, espuma_cvar) = &*espuma;
    if let Ok(mut espuma_mut) = espuma_cvar.wait_while(espuma_lock.lock().unwrap(), |cont_espuma| {
        cont_espuma.necesito_espuma = true;
        espuma_cvar.notify_all();
        cont_espuma.espuma < pedido.espuma && cont_espuma.leche != VACIO
    }) {
        espuma_mut.necesito_espuma = false;
        if espuma_mut.leche == VACIO && espuma_mut.espuma < pedido.espuma {
            println!(
                "[Pedido {}] Contenedor de leche vacio y no me alcanza la espuma",
                id
            );
            espuma_cvar.notify_all();
            if let Ok(mut dispensadores_mut) = dispensadores.write() {
                dispensadores_mut[num_dispensador as usize] = false;
            }
            return Err(CafeteraError::EspumaInsuficiente);
        }
        println!("[Pedido {}] sirviendo espuma", id);
        espuma_mut.espuma_consumida += pedido.espuma;
        espuma_mut.espuma -= pedido.espuma;
        thread::sleep(Duration::from_millis(
            TIEMPO_RECURSO_UNIDAD * pedido.espuma as u64,
        ));
        espuma_cvar.notify_all();
    }

    if let Ok(mut dispensadores_mut) = dispensadores.write() {
        dispensadores_mut[num_dispensador as usize] = false;
    }

    println!("[Pedido {}] terminé", id);

    Ok(())
}
