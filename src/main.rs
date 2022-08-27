//ejemplo de programa con structs

#[derive(Debug)] //indicamos que funcione la impresion en debug
struct Rectangulo {
    base:u32,
    altura:u32
}

//como la funcion get_area_vx esta completamente relacionada a nuestro sctruct Rectangulo
//es decir solo sirve para rectangulo, la podemos convertir en un metodo mediante la sintaxis del metodo

//todo lo que este en impl rectangulo, es perteneciente a rectangulo
//y solo puede ser invocado por instancias de Rectangulo

//pueden existir multiples bloques impl, pero generalmente solo enciertos escenarios brinda utilidad extra (generics and traits)
impl Rectangulo {
    //la signature de los metodos debe de llevar self, que es un alias de la instancia que llama al metodo
    //self es un shorhand de self: self
    //usamos una referencia a self, por que solo queremos leer la instacia
    //si queremos modificar los campos de self, tendriamos que indicar que la referencia es mutable &mut self
    //usar solo self (owner) es poco comun, generalmente se usa cuando se quiere convertir la instancia en otra cosa y se quiere
    //prevenir que se use la instacia original nuevamente
    fn get_area(&self) -> u32 {
        self.base * self.altura
    }

    //los metodos tambien pueden recibir parametros
    //los parametros siempre se deben declarar despues de self
    
    //el siguiente metodo tiene como parametro otra instancia de Rectangulo
    //y compara sus medidas con las de self
    fn can_fit(&self, other: &Rectangulo) -> bool {
        self.base > other.base && self.altura > other.altura
    }

    //todas las funciones que estan definidas en un bloque impl se conocen como funciones asociadas
    //pueden existir funciones que tienen a self (por lo tanto no son metodos) y su funcion es que pueden ser llamadas sin una instancia
    //una practica comun es llamarlas new y permiten generar instancias (aunque new no es parte de Rust)

    //funcion que devuelve una instancia de Rectangulo que tiene la misma base y altura (cuadrado)
    //permite obtener la instancia sin tener que escribir el mismo valor para ambas medidas

    fn square(side_size: u32) -> Self { //recordemos que Self es un alias del struct que esta despues del impl en donde se declara la funcion
        Self { //Escribir Rectangulo es valido, pero si el nombre del struct cambia, tambien tendriamos que replicar el cambio
            base: side_size,
            altura: side_size,
        }
    }
}

fn main() {
    //vamos a obtener el area de un rectangulo dadas sus medidas en pixeles
    //creamos la instancia y la enviamos al metodo que espera una referencia del tipo Rectangulo
    let rect = Rectangulo {
        base: 10,
        altura: 20,
    };

    //enviamos referencia de instancia
    let area = get_area_v3(&rect);

    println!("un rectangulo de {}px x {}px tiene una area de {}px", rect.base,rect.altura,area);

    //la macro println! llama la implementacion de Display de las variables
    //los tipos primitivos implementan ese trait de manera automatica (ya que solo existe una forma de mostarlos en pantalla)
    //los struct no implementan Display, por que hay varias posibilidades a la hora de mostrar
    //aqui es donde es util usar los formatos de Debug y Debug con pretty-format con indicando {:?} o {:#?} respectivamente
    //nos permite saber de manera rapida que valores tiene una instancia (para detectar bugs o depurar de manera rapida)

    println!("la instancia rect en este momento: {:#?}", rect); //para que funcione la funcion de Debug debemos indicar el derive explicitamente en el struct

    //alternativamente podemos usar la macro dbg!()
    //esta si toma el ownership (a diferencia de println! que toma referencias)
    //ademas de que usa el stream de la consola de errores, en lugar del stream de la consola de salida estandar
    //muestra el archivo y la linea donde se llamo la macro

    dbg!(&rect); //mandamos la referencia para que no tome el ownership

    //toma el ownership, pero tambien lo retorna.
    //el ownership de altura se asigna como si no se hubiera llamado a dbg!()

    let rect2 = Rectangulo {
        base: 10,
        altura: dbg!(2*5),//dbg!() toma el ownership, pero lo asigna posteriormente a altura, por lo que altura conserva el ownership
    };

    let rect3 = Rectangulo {
        base: 2,
        altura: 2,
    };

    //calculo de area con metodo

    let otra_area = rect2.get_area(); //no es necesario poner la instancia como argumento
    dbg!(&otra_area);
    println!("rect puede contener a rect3? {}",rect.can_fit(&rect3));

    let cuadrado = Rectangulo::square(4);
    dbg!(&cuadrado);
}

//la primera version de la funcion solo recibe los valores
//esta signature no es muy descriptiva, ni asociada a un rectangulo
fn get_area_v1(base: u32, altura: u32) -> u32 {
    base * altura
}

// version con tuple, permite que solo un valor sea pasado
//no es muy verbose
//el orden de los valores puede ser importante y este codigo tiene tendencia a errores
fn get_area_v2(rectangulo: (u32, u32)) -> u32 {
    rectangulo.0 * rectangulo.1
}

//version con struct que recibe una referencia de una instancia Rectangulo
//es mas clara y se entiende que el contexto del uso de los valores
fn get_area_v3(rect: &Rectangulo) -> u32 {
    //acceder a los campos de una referencia a una instacia no los mueve.
    rect.base * rect.altura
}

//nota:
//en C/C++ existe el operador -> que permite invocar un metodo desde
//una referencia, el cual internamente hace una des-referenciacion
//referencia -> saludar() es igual a (*referencia).saludar()

//En Rust no existe esa funcionalidad por que usa refencia y des-refencia automaticas
//significa que Rust automaticamente añade & o &mut u object para hacer match con la signature
//es lo mismo p1.distance(&p2) y (&p1).distance(&p2)
