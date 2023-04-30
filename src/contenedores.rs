use crate::constantes::{A, C, E, G, L, M};

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
    /// Indica si es necesario recargar el contenedor de cafe molido
    pub necesito_cafe: bool,
}

impl ContenedorCafe {
    pub fn new() -> Self {
        ContenedorCafe {
            cafe_granos: G,
            cafe_molido: M,
            cafe_granos_consumido: 0,
            cafe_molido_consumido: 0,
            necesito_cafe: false,
        }
    }
}

impl Default for ContenedorCafe {
    fn default() -> Self {
        Self::new()
    }
}

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

#[derive(Debug)]
pub struct ContenedorCacao {
    /// Cantidad de cacao disponible
    pub cacao: i32,
    /// Cantidad de cacao consumido
    pub cacao_consumido: i32,
}

impl ContenedorCacao {
    pub fn new() -> Self {
        ContenedorCacao {
            cacao: C,
            cacao_consumido: 0,
        }
    }
}

impl Default for ContenedorCacao {
    fn default() -> Self {
        Self::new()
    }
}

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
