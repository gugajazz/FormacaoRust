/*

Crie structs que representem várias formas geométricas
(Quadrado, circulo, elipse, Triângulo, Cubo, Cilindro, Esfera).
Depois, utilize os enums para juntar todas as formas em apenas um tipo, Forma.
Este tipo deve ser capaz de calcular a área e perímetro e volume.

*/

struct Quadrado {
    lado: u32,
}
impl Quadrado {
    fn area(&self) -> u32 {
        self.lado * self.lado
    }
    fn perimetro(&self) -> u32 {
        4 * self.lado
    }
    fn volume(&self) -> u32 {
        self.lado * self.lado * self.lado
    }
}

struct Circulo {
    raio: u32,
}
impl Circulo {
    fn area(&self) -> f32 {
        3.14 * (self.raio * self.raio) as f32
    }
    fn perimetro(&self) -> f32 {
        2.0 * 3.14 * self.raio as f32
    }
    fn volume(&self) -> f32 {
        (4.0 / 3.0) * 3.14 * (self.raio * self.raio * self.raio) as f32
    }
}
struct Elipse {
    raio_menor: u32,
    raio_maior: u32,
}
impl Elipse {
    fn area(&self) -> f32 {
        3.14 * self.raio_menor as f32 * self.raio_maior as f32
    }
    fn perimetro(&self) -> f32 {
        2.0 * 3.14 * ((self.raio_menor as f32 + self.raio_maior as f32) / 2.0)
    }
    fn volume(&self) -> f32 {
        (4.0 / 3.0) * 3.14 * (self.raio_menor * self.raio_maior * self.raio_maior) as f32
    }
}
struct Triangulo {
    base: u32,
    altura: u32,
}
impl Triangulo {
    fn area(&self) -> f32 {
        (self.base * self.altura) as f32 / 2.0
    }
    fn perimetro(&self) -> u32 {
        self.base
            + self.altura
            + ((self.base * self.base + self.altura * self.altura) as f32).sqrt() as u32
    }
    fn volume(&self) -> u32 {
        (self.base * self.altura * self.base) / 3
    }
}
struct Cubo {
    lado: u32,
}
impl Cubo {
    fn area(&self) -> u32 {
        6 * self.lado * self.lado
    }
    fn perimetro(&self) -> u32 {
        12 * self.lado
    }
    fn volume(&self) -> u32 {
        self.lado * self.lado * self.lado
    }
}
struct Cilindro {
    raio: u32,
    altura: u32,
}
impl Cilindro {
    fn area(&self) -> f32 {
        2.0 * 3.14 * self.raio as f32 * (self.altura as f32 + self.raio as f32)
    }
    fn perimetro(&self) -> f32 {
        2.0 * 3.14 * self.raio as f32
    }
    fn volume(&self) -> f32 {
        3.14 * (self.raio * self.raio) as f32 * self.altura as f32
    }
}
struct Esfera {
    raio: u32,
}
impl Esfera {
    fn area(&self) -> f32 {
        4.0 * 3.14 * (self.raio * self.raio) as f32
    }
    fn perimetro(&self) -> f32 {
        2.0 * 3.14 * self.raio as f32
    }
    fn volume(&self) -> f32 {
        (4.0 / 3.0) * 3.14 * (self.raio * self.raio * self.raio) as f32
    }
}

enum Forma {
    Quadrado(Quadrado),
    Circulo(Circulo),
    Elipse(Elipse),
    Triangulo(Triangulo),
    Cubo(Cubo),
    Cilindro(Cilindro),
    Esfera(Esfera),
}

fn main() {
    let quadrado = Quadrado { lado: 4 };
    let circulo = Circulo { raio: 3 };
    let elipse = Elipse {
        raio_menor: 2,
        raio_maior: 4,
    };
    let triangulo = Triangulo { base: 3, altura: 4 };
    let cubo = Cubo { lado: 5 };
    let cilindro = Cilindro { raio: 2, altura: 5 };
    let esfera = Esfera { raio: 3 };

    let mut formas = Vec::new();
    formas.push(Forma::Quadrado(quadrado));
    formas.push(Forma::Circulo(circulo));
    formas.push(Forma::Elipse(elipse));
    formas.push(Forma::Triangulo(triangulo));
    formas.push(Forma::Cubo(cubo));
    formas.push(Forma::Cilindro(cilindro));
    formas.push(Forma::Esfera(esfera));

    for forma in formas {
        match forma {
            Forma::Quadrado(q) => println!(
                "quadrado: Área: {}  Perimetro: {}    Volume: {}",
                q.area(),
                q.perimetro(),
                q.volume()
            ),
            Forma::Circulo(c) => println!(
                "círculo: Área: {}    Perimetro: {}    Volume: {}",
                c.area(),
                c.perimetro(),
                c.volume()
            ),
            Forma::Elipse(e) => println!(
                "elipse: Área: {}  Perimetro: {}    Volume: {}",
                e.area(),
                e.perimetro(),
                e.volume()
            ),
            Forma::Triangulo(t) => println!(
                "triângulo: Área: {}    Perimetro: {}    Volume: {}",
                t.area(),
                t.perimetro(),
                t.volume()
            ),
            Forma::Cubo(c) => println!(
                "cubo: Área: {}  Perimetro: {}    Volume: {}",
                c.area(),
                c.perimetro(),
                c.volume()
            ),
            Forma::Cilindro(c) => println!(
                "cilindro: Área: {}  Perimetro: {}    Volume: {}",
                c.area(),
                c.perimetro(),
                c.volume()
            ),
            Forma::Esfera(e) => println!(
                "esfera: Área: {}  Perimetro: {}    Volume: {}",
                e.area(),
                e.perimetro(),
                e.volume()
            ),
        }
    }
}
