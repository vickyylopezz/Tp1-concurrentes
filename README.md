# Trabajo Practico: CoffeeGPT
## Correr el programa:
```
cargo run
```

Y al momento donde se muestre:

```
Ingrese el archivo con el pedido
```
Ingresar:
```
<nombre_archivo>
```
<nombre_archivo> es la ruta del archivo con los pedidos a preparar. Por ejemplo: 
```
pedido.txt
```

El archivo debe ser del formato:
```
<cantidad_de_cafe>,<cantidad_de_agua>,<cantidad_de_cacao>,<cantidad_de_espuma>
```
Y estas cantidades deben estar dentro del rango 0..maximo, donde el maximo de cada recurso se encuentra definido en el archivo constantes.rs

Por ejemplo:
```
4,5,6,2
```

## Correr los tests:
```
cargo test
```

## Modelo
### Cafetera
La cafetera cuenta con N dispensadores, un contenedor de cafe, un contenedor de agua, un contenedor de cadao y un contenedor de espuma. Adicionalmente indica si ya se terminaron de procesar todos los pedidos y cuantos fueran completados exitosamente. 

### Dispensadores
La logica de los dispensadores fue planteada con semaforos para poder asignarlos concurrentemente a los distintos pedidos a medida que se van liberando. Este modelo fue seguido del ejemplo de la practica de los cajeros y las personas que desean ingresar a estos, donde se planteaba: 
"En un banco, al mediodía, se juntan 30 personas que necesitan realizar operaciones en los 4 cajeros automáticos disponibles."
Este ejemplo fue adaptado para la implementacion de los dispensadores donde las "personas" harian de "pedidos" y los "cajeros" de "dispensadores".

### Contenedores
Los contenedores de agua, cafe y espuma fueron implementados de la misma manera. Se utilizó un Arc con un Mutex y una Conditional Variable. El Mutex para poder acceder al contenedor y sus atributos de a un thread a la vez y la Conditional Variable para poder indicar cuando el recurso es insuficiente y se necesita recargar el contenedor. Mientras que no se necesita recargar el contenedor, se hace un wait_while para evitar utilizar un busy wait.

En cambio, el contenedor de cacao solo se necesita un Arc(Mutex) ya que no se recarga, lo que hay de cacao es lo que se puede utilizar. No es necesario tener una Conditional Variable para ver cuando se necesita cacao.

## Hipotesis y supuestos:
- Los contenedores de cafe en granos, cacao y leche no se recargan. Una vez que se vacian, los pedidos que necesiten de esos recursos no podran prepararse.
- Al iniciar el programa todos los contenedores arrancan llenos.
- En el caso de que un pedido no se pueda completar por la falta de algun recurso, se descartará y se seguirá con el siguiente. 

## Modificaciones:
- Cambio logica de recargar contenedores removiendo el flag de necesito_xx. Ahora se recargará el contenedor cuando se llegue a un limite, este limite esta dado por la cantidad maxima de recurso que puede tener el pedido. De esta forma, si el contenedor se encuentra por debajo de esta cantidad maxima, es probable que la cafetera pueda solo completar el pedido que esta preparando y ninguno otro, o tambien que no pueda ni prepararlo. De esta forma, se recarga el contenedor y la cafetera se puede asegurar que siempre va a tener recurso disponible. 
- Agrego que solo se sirva el recurso y el pedido lo necesita, es decir, esa cantidad es mayor que cero.