use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use ::rand::thread_rng;
use std::collections::HashMap;
use std::any::Any;

// -------------------- TRAIT GENERAL PARA ORGANISMOS --------------------

trait Organismo: Any {
    fn envejecer(&mut self);
    fn reproducirse(&self, rng: &mut ThreadRng) -> Vec<Box<dyn Organismo>>;
    fn peso(&self) -> f64;
    fn esta_vivo(&self) -> bool;
    fn nombre(&self) -> &str;
    fn posicion(&self) -> (f32, f32);
    fn set_posicion(&mut self, x: f32, y: f32);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// -------------------- AUXILIARES --------------------

fn crecimiento_gompertz(a: f64, b: f64, k: f64) -> impl Fn(u32) -> f64 {
    move |edad| {
        let t = edad as f64;
        a * (-b * (-k * t).exp()).exp()
    }
}

fn posicion_random() -> (f32, f32) {
    let mut rng = thread_rng();
    (
        rng.gen_range(50.0..750.0),
        rng.gen_range(50.0..550.0),
    )
}

fn distancia(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

// -------------------- CABRA --------------------

struct Cabra {
    edad: u32,
    peso: f64,
    energia: f64,
    crecimiento: Box<dyn Fn(u32) -> f64>,
    pos: (f32, f32),
    viva: bool,
    enferma: bool,
    dias_enferma: u32, 
}

impl Cabra {
    fn nueva() -> Self {
        Self {
            edad: 0,
            peso: 5.0,
            energia: 50.0, 
            crecimiento: Box::new(crecimiento_gompertz(75.0, 2.8, 0.01)),
            pos: posicion_random(),
            viva: true,
            enferma: false,
            dias_enferma: 0,
        }
    }

    fn buscar_y_comer(&mut self, plantas: &[Planta]) {
        if self.enferma {
            return; 
        }
        if let Some(planta) = plantas.iter().min_by(|a, b| {
            let da = distancia(self.pos, a.pos);
            let db = distancia(self.pos, b.pos);
            da.partial_cmp(&db).unwrap()
        }) {
            let d = distancia(self.pos, planta.pos);
            if d < 10.0 {
               
                self.energia = (self.energia + 10.0).min(100.0);
                println!("🐐 Cabra comió una planta, energía: {:.1}", self.energia);
            } else {
                
                let (px, py) = planta.pos;
                let dx = px - self.pos.0;
                let dy = py - self.pos.1;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 1.0 {
                    self.pos.0 += 2.0 * dx / dist;
                    self.pos.1 += 2.0 * dy / dist;
                }
            }
        }
    }
}

impl Organismo for Cabra {
    fn envejecer(&mut self) {
        self.edad += 1;
        self.peso = (self.crecimiento)(self.edad);
        if self.enferma {
            self.energia -= 0.5;
            self.dias_enferma += 1;
            if self.dias_enferma >= 5 {
                self.enferma = false;
                self.dias_enferma = 0;
            }
        } else {
            self.energia -= 0.2;
            if self.edad >= 10 {
                let mut rng = thread_rng();
                if rng.gen_bool(0.1) {
                    self.enferma = true;
                    self.dias_enferma = 0;
                }
            }
        }
        if self.energia <= 0.0 {
            self.viva = false;
        }
    }

    fn reproducirse(&self, rng: &mut ThreadRng) -> Vec<Box<dyn Organismo>> {
        let mut hijos = Vec::new();
        if self.edad > 15 && rng.gen_bool(0.05) && self.energia > 30.0 {
            let cantidad = rng.gen_range(1..=2);
            hijos.extend((0..cantidad).map(|_| Box::new(Cabra::nueva()) as Box<dyn Organismo>));
        }
        hijos
    }

    fn peso(&self) -> f64 {
        self.peso
    }

    fn esta_vivo(&self) -> bool {
        self.viva && self.edad < 30
    }

    fn nombre(&self) -> &str {
        "Cabra"
    }

    fn posicion(&self) -> (f32, f32) {
        self.pos
    }

    fn set_posicion(&mut self, x: f32, y: f32) {
        self.pos = (x, y);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// -------------------- PLANTA --------------------

struct Planta {
    edad: u32,
    pos: (f32, f32),
    viva: bool,
}

impl Planta {
    fn nueva() -> Self {
        Self {
            edad: 0,
            pos: posicion_random(),
            viva: true,
        }
    }
}

impl Organismo for Planta {
    fn envejecer(&mut self) {
        self.edad += 1;
    }

    fn reproducirse(&self, _rng: &mut ThreadRng) -> Vec<Box<dyn Organismo>> {
        Vec::new() 
    }

    fn peso(&self) -> f64 {
        0.5
    }

    fn esta_vivo(&self) -> bool {
        self.viva
    }

    fn nombre(&self) -> &str {
        "Planta"
    }

    fn posicion(&self) -> (f32, f32) {
        self.pos
    }

    fn set_posicion(&mut self, x: f32, y: f32) {
        self.pos = (x, y);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// -------------------- DEPREDADOR --------------------

struct Depredador {
    energia: f64,
    nivel_minimo: f64,
    nivel_optimo: f64,
    vivo: bool,
    pos: (f32, f32),
    velocidad: f32,
}

impl Depredador {
    fn nuevo() -> Self {
        Self {
            energia: 60.0,
            nivel_minimo: 30.0,
            nivel_optimo: 70.0,
            vivo: true,
            pos: (400.0, 300.0),
            velocidad: 15.5,
        }
    }

    fn objetivo_mas_cercano<'a>(&self, presas: &'a [Box<dyn Organismo>]) -> Option<(usize, &'a Box<dyn Organismo>)> {
        presas.iter()
            .enumerate()
            .filter(|(_, p)| p.nombre() == "Cabra")
            .min_by(|(_, a), (_, b)| {
                let da = distancia(self.pos, a.posicion());
                let db = distancia(self.pos, b.posicion());
                da.partial_cmp(&db).unwrap()
            })
    }

    fn puede_cazar(&self) -> f64 {
        let mut rng = thread_rng();
        rng.gen_range(self.nivel_minimo..=self.nivel_optimo)
    }

    fn mover_y_cazar(&mut self, presas: &mut Vec<Box<dyn Organismo>>) {
        let comida_necesaria = self.puede_cazar();
        if let Some((idx, presa)) = self.objetivo_mas_cercano(presas) {
            let (px, py) = presa.posicion();
            let (dx, dy) = (px - self.pos.0, py - self.pos.1);
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > 5.0 {
                self.pos.0 += self.velocidad * dx / dist;
                self.pos.1 += self.velocidad * dy / dist;
            } else {
                println!("🐺 Depredador cazó una cabra de {:.1} kg", presa.peso());
                self.energia += presa.peso();
                presas.remove(idx);
            }
        }
    }

    fn consumir(&mut self) {
        if self.energia >= self.nivel_optimo {
            self.energia -= 0.5;
        } else if self.energia >= self.nivel_minimo {
            self.energia -= 0.5;
        } else {
            self.vivo = false;
        }
    }
}

// -------------------- SIMULADOR --------------------

struct Simulador {
    poblacion: Vec<Box<dyn Organismo>>,
    plantas: Vec<Planta>,
    depredador: Depredador,
    dia: u32,
}

impl Simulador {
    fn new() -> Self {
        let mut plantas = Vec::new();
        for _ in 0..30 {
            plantas.push(Planta::nueva());
        }
        Self {
            poblacion: Vec::new(),
            plantas,
            depredador: Depredador::nuevo(),
            dia: 0,
        }
    }

    fn agregar(&mut self, organismo: Box<dyn Organismo>) {
        self.poblacion.push(organismo);
    }

    fn simular_dia(&mut self) {
        let mut rng = thread_rng();
        let mut nuevos = Vec::new();

        // Cabras envejecen y buscan plantas
        for org in self.poblacion.iter_mut() {
            if org.nombre() == "Cabra" {
                let cabra = org.as_any_mut().downcast_mut::<Cabra>().unwrap();
                cabra.buscar_y_comer(&self.plantas);
            }
        }

        // Envejecer y reproducirse
        self.poblacion.retain_mut(|org| {
            org.envejecer();
            nuevos.extend(org.reproducirse(&mut rng));
            org.esta_vivo()
        });
        self.poblacion.extend(nuevos);

        // Depredador caza
        if self.depredador.vivo {
            self.depredador.mover_y_cazar(&mut self.poblacion);
            self.depredador.consumir();
        }

        self.dia += 1;
    }

    fn resumen(&self) -> HashMap<&str, usize> {
        self.poblacion.iter().fold(HashMap::new(), |mut acc, org| {
            *acc.entry(org.nombre()).or_insert(0) += 1;
            acc
        })
    }
}

// -------------------- MAIN --------------------
#[macroquad::main("Depredador-Presa con Energía")]
async fn main() {
    let mut sim = Simulador::new();

    for _ in 0..10 {
        sim.agregar(Box::new(Cabra::nueva()));
    }

    let mut frame_count = 0;

    loop {
        clear_background(BLUE);

        frame_count += 1;
        if frame_count % 2 == 0 {
            sim.simular_dia();
        }

        // Dibujar cabras y plantas
        for org in &mut sim.poblacion {
            let (x, y) = org.posicion();
            if org.nombre() == "Cabra" {
                let cabra = org.as_any_mut().downcast_ref::<Cabra>().unwrap();
                if cabra.enferma {
                    draw_circle(x, y, 8.0, PURPLE); // Morado si está enferma
                } else {
                    draw_circle(x, y, 8.0, BROWN);
                }
            }
        }
        for planta in &sim.plantas {
            let (x, y) = planta.pos;
            draw_circle(x, y, 5.0, GREEN);
        }

        // Dibujar depredador
        if sim.depredador.vivo {
            let (x, y) = sim.depredador.pos;
            draw_circle(x, y, 12.0, RED);
        }

        // Estadísticas
        let resumen = sim.resumen();
        let mut y_text = 20.0;
        draw_text(&format!("Día: {}", sim.dia), 20.0, y_text, 24.0, BLACK);
        y_text += 30.0;
        for (nombre, cantidad) in resumen {
            draw_text(&format!("{}: {}", nombre, cantidad), 20.0, y_text, 20.0, DARKGRAY);
            y_text += 25.0;
        }
        draw_text(
            &format!(
                "Depredador: {} | Energía: {:.1}",
                if sim.depredador.vivo { "Vivo" } else { "Muerto" },
                sim.depredador.energia
            ),
            20.0,
            y_text,
            20.0,
            RED,
        );

        next_frame().await;
    }
}

const PURPLE: Color = Color::new(0.5, 0.0, 0.5, 1.0); // Definición de color morado