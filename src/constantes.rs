///Capacidad de contenedor de granos a moler
pub const G: i32 = 500;
///Capacidad de contenedor de granos molidos
pub const M: i32 = 30;
///Capacidad de contenedor de leche
pub const L: i32 = 500;
///Capacidad de contenedor de espuma
pub const E: i32 = 500;
///Capacidad de contenedor de cacao
pub const C: i32 = 10;
///Capacidad de contenedor de agua
pub const A: i32 = 40;
///Porcentaje a alertar cuando el contenedor esta por debajo
pub const X: i32 = 50;
///Capantidad de dispensadores
pub const N: i32 = 5;

//Tiempo de espera para reponer el contenedor de cafe molido
pub const TIEMPO_CAFE_REPONER: u64 = 5000;
//Tiempo de espera para reponer el contenedor de espuma
pub const TIEMPO_ESPUMA_REPONER: i32 = 100;
//Tiempo de espera por unidad de cafe, agua, cacao o espuma del pedido
pub const TIEMPO_RECURSO_UNIDAD: u64 = 50;
//Tiempo de espera para reponer el contenedor de agua
pub const TIEMPO_AGUA_REPONER: u64 = 5000;

/// Cantidad maxima de cafe por pedido (siempre debe ser menor a M)
pub const MAX_CAFE_POR_PEDIDO: i32 = 10;
/// Cantidad maxima de agua por pedido (siempre debe ser menor a A)
pub const MAX_AGUA_POR_PEDIDO: i32 = 15;
/// Cantidad maxima de cacao por pedido (siempre debe ser menor a C)
pub const MAX_CACAO_POR_PEDIDO: i32 = 8;
/// Cantidad minima de cualquier recurso por pedido
pub const MIN_CANTIDAD_POR_PEDIDO: i32 = 0;
/// Indica que el contenedor esta vacio cuando si cantidad es 0
pub const VACIO: i32 = 0;