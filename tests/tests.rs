#[cfg(test)]
mod tests {
    use std::vec;

    use tp1::cafetera::Cafetera;
    use tp1::chequeo_pedidos::pedidos;
    use tp1::constantes::{C, E, G, L, M};
    use tp1::error::PedidoError;
    use tp1::pedido::Pedido;
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

    #[test]
    fn archivo_con_un_pedido_test() {
        let pedido_expected = vec![vec![4, 7, 6, 2]];
        let pedidos_archivo = read_file_lines("tests/pedidosTest3.txt");

        assert_eq!(pedidos_archivo, Ok(pedido_expected));

        let pedidos_armar = pedidos(pedidos_archivo.expect("Error leer archivo"));
        assert_eq!(
            pedidos_armar,
            Ok(vec![Pedido {
                cafe_molido: 4,
                agua_caliente: 7,
                cacao: 6,
                espuma: 2
            }])
        );

        let cafetera = Cafetera::new();
        let pedidos_preparar = pedidos_armar.expect("Error en pedidos");

        cafetera.preparar_pedidos(pedidos_preparar.clone());

        estado_correcto_cafetera(cafetera, pedidos_preparar)
    }

    #[test]
    fn archivo_con_varios_pedidos_test() {
        let pedidos_archivo = read_file_lines("tests/pedidosTest4.txt");

        let pedidos_armar = pedidos(pedidos_archivo.expect("Error leer archivo"));

        let cafetera = Cafetera::new();
        let pedidos_preparar = pedidos_armar.expect("Error en pedidos");

        cafetera.preparar_pedidos(pedidos_preparar.clone());

        estado_correcto_cafetera(cafetera, pedidos_preparar)
    }

    fn estado_correcto_cafetera(cafetera: Cafetera, pedidos: Vec<Pedido>) {
        let mut cafe_total = 0;
        let mut agua_total = 0;
        let mut cacao_total = 0;
        let mut espuma_total = 0;
        for pedido in pedidos.clone() {
            cafe_total += pedido.cafe_molido;
            agua_total += pedido.agua_caliente;
            cacao_total += pedido.cacao;
            espuma_total += pedido.espuma;
        }

        if let Ok(cont_cafe) = cafetera.contenedor_cafe.0.lock() {
            assert_eq!(
                cont_cafe.cafe_molido,
                M - (cafe_total - cont_cafe.cafe_granos_consumido)
            );
            assert_eq!(
                cont_cafe.cafe_granos,
                G - (cafe_total - (M - cont_cafe.cafe_molido))
            );
            assert_eq!(cont_cafe.cafe_molido_consumido, cafe_total);
            assert_eq!(
                cont_cafe.cafe_granos_consumido,
                cont_cafe.cafe_molido_consumido + cont_cafe.cafe_molido - M
            )
        }

        if let Ok(cont_agua) = cafetera.contenedor_agua.0.lock() {
            assert_eq!(cont_agua.agua_caliente_consumida, agua_total);
        }

        if let Ok(cont_cacao) = cafetera.contenedor_cacao.lock() {
            assert_eq!(cont_cacao.cacao, C - cacao_total);
            assert_eq!(cont_cacao.cacao_consumido, cacao_total);
        }

        if let Ok(cont_espuma) = cafetera.contenedor_espuma.0.lock() {
            assert_eq!(
                cont_espuma.espuma,
                E - (espuma_total - cont_espuma.leche_consumida)
            );
            assert_eq!(cont_espuma.espuma_consumida, espuma_total);
            assert_eq!(
                cont_espuma.leche,
                L - (espuma_total - (E - cont_espuma.espuma))
            );
            assert_eq!(
                cont_espuma.leche_consumida,
                cont_espuma.espuma_consumida + cont_espuma.espuma - E
            );
        }

        assert_eq!(
            *cafetera
                .pedidos_completados
                .lock()
                .expect("Error en pedidos"),
            pedidos.len() as i32
        )
    }
}
