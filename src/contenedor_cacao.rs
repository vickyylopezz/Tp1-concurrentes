use crate::constantes::C;

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
