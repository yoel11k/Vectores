const MAX: usize = 200;

struct Vectores {
    //Definir atributos del objeto.
    dimension: usize,
    elemento: [u64;MAX],
}

impl Vectores {
    //Constructor
    fn new() -> Vectores {
        Vectores {
            dimension: 0,
            elemento: [0;MAX],
        }
    }

    fn dim(&self) -> usize {
        self.dimension
    }

    fn add_element(&mut self, e:u64){
        if self.dimension < MAX {
            self.elemento[self.dimension] = e;
            self.dimension += 1;
        }
    }

    fn mostrar(&self) {
        for i in 0..self.dimension { //Cuando hago uso de for, siempre va hasta el ultimo elemento - 1.
            println!(" Elemento[{}] = {}", i, self.elemento[i]);
        }

    }
    fn eliminar_un_elemento_dado_por_el_usuario(&mut self, e:u64) {
        let mut i = 0;
        while i < self.dimension {
            if self.elemento[i] == e {
                for j in i..self.dimension-1 {
                    self.elemento[j] = self.elemento[j+1];
                }
                self.dimension -= 1;
            } else {
                i += 1;
            }
        }
    }
        // Ordena todo el vector de menor a mayor (burbuja).
    fn ordenar_ascendente(&mut self) {
        for i in 0..self.dimension {
            for j in 0..self.dimension - 1 - i {
                if self.elemento[j] > self.elemento[j + 1] {
                    let aux = self.elemento[j];
                    self.elemento[j] = self.elemento[j + 1];
                    self.elemento[j + 1] = aux;
                }
            }
        }
    }
 
    // Ejercicio 1: impares a la izquierda y pares a la derecha,
    // cada grupo ordenado ascendentemente.
    fn ordenar_impares_pares(&mut self) {
        if self.dimension == 0 {
            return;
        }
        self.ordenar_ascendente();
 
        let mut temp = [0u64; MAX];
        let mut k = 0;
 
        // Primero los impares (ya salen ordenados).
        for i in 0..self.dimension {
            if self.elemento[i] % 2 != 0 {
                temp[k] = self.elemento[i];
                k += 1;
            }
        }
        // Luego los pares.
        for i in 0..self.dimension {
            if self.elemento[i] % 2 == 0 {
                temp[k] = self.elemento[i];
                k += 1;
            }
        }
 
        for i in 0..self.dimension {
            self.elemento[i] = temp[i];
        }
    }
 
    // Ejercicio 2: ordenar "de adentro hacia afuera".
    // El menor queda en el centro (posicion n/2) y los siguientes
    // se colocan alternando: izquierda, derecha, izquierda, derecha...
    fn ordenar_adentro_afuera(&mut self) {
        let n = self.dimension;
        if n == 0 {
            return;
        }
        self.ordenar_ascendente();
 
        let mut temp = [0u64; MAX];
        let centro = n / 2;
        let mut izq = centro;
        let mut der = centro;
 
        temp[centro] = self.elemento[0];
        for k in 1..n {
            if k % 2 == 1 {
                izq -= 1;
                temp[izq] = self.elemento[k];
            } else {
                der += 1;
                temp[der] = self.elemento[k];
            }
        }
 
        for i in 0..n {
            self.elemento[i] = temp[i];
        }
    }

}

fn cargar(datos: &[u64]) -> Vectores {
    let mut v = Vectores::new();
    for &d in datos {
        v.add_element(d);
    }
    v
}


fn main() {
    let mut v = Vectores::new(); //Instancia

    v.add_element(3);
    v.add_element(1);
    v.add_element(5);
    v.add_element(7);
    v.add_element(0);
    v.add_element(13);
    v.add_element(18);
    v.add_element(2);
    v.add_element(4);

    println!("-------------------");
    v.mostrar();
    println!("-------------------");

    println!("La cantidad de elementos es (Dimension): {}", v.dim());

    v.eliminar_un_elemento_dado_por_el_usuario(5);
     println!("-------------------");
    v.mostrar();
    println!("-------------------");
    
    println!("=== Ejercicio 1: impares | pares ===");
    let mut v = cargar(&[5, 4, 3, 2, 1]);
    v.mostrar();
    v.ordenar_impares_pares();
    v.mostrar();
 
    println!("\n=== Ejercicio 2: de adentro hacia afuera ===");
    let mut v = cargar(&[1, 6, 5, 2, 0, 4, 3]);
    v.mostrar();
    v.ordenar_adentro_afuera();
    v.mostrar();
 
    let mut v = cargar(&[7, 0, 3, 4, 1, 8]);
    v.mostrar();
    v.ordenar_adentro_afuera();
    v.mostrar();
 
    println!("\nDimension: {}", v.dim());

}


