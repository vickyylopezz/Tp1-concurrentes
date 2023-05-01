#[derive(Debug, PartialEq, Eq)]
pub enum CafeteraError {
    CacaoInsuficiente,
    CafeInsuficiente,
    EspumaInsuficiente,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PedidoError {
    NoHayPedidos,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileError {
    ArchivoInexistente,
}
