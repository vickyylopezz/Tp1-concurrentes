#[cfg(test)]
mod tests {
    use tp1::chequeo_pedidos::pedidos;
    use tp1::error::PedidoError;
    use tp1::{archivo::read_file_lines, error::FileError};
    #[test]
    fn archivo_invalido_test() {
        assert_eq!(
            read_file_lines("pedidosError.txt"),
            Err(FileError::ArchivoInexistente)
        );
    }

    #[test]
    fn archivo_vacio_test() {
        let pedidos_expected = vec![];
        let pedidos_archivo = read_file_lines("tests/pedidosTest1.txt");

        assert_eq!(pedidos_archivo, Ok(pedidos_expected));

        if let Ok(pedidos_chequear) = pedidos_archivo {
            assert_eq!(pedidos(pedidos_chequear), Err(PedidoError::NoHayPedidos));
        }
    }

    #[test]
    fn archivo_con_pedidos_invalidos_test() {
        let pedidos_expected = vec![
            vec![-4, 5, 8, 9],
            vec![4, -9, 2, 1],
            vec![6, 7, -2, 7],
            vec![1, 2, 3, -4],
        ];
        let pedidos_archivo = read_file_lines("tests/pedidosTest2.txt");

        assert_eq!(pedidos_archivo, Ok(pedidos_expected));

        if let Ok(pedidos_chequear) = pedidos_archivo {
            assert_eq!(pedidos(pedidos_chequear), Err(PedidoError::NoHayPedidos));
        }
    }
}
