use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone)]
pub struct Boid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    boids: Vec<Boid>,
}

impl Boid {
    const AVOID_FACTOR: f32 = 0.05;
    const VISIBLE_RANGE: f32 = 20.0;
    const MATCHING_FACTOR: f32 = 0.05;
    const TURN_FACTOR: f32 = 0.2;
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
        }
    }

    pub fn update(&mut self, other_boids: &[Boid], width: f32, height: f32) {
        let mut close_dx = 0.0;
        let mut close_dy = 0.0;
        let mut xvel_avg = 0.0;
        let mut yvel_avg = 0.0;
        let mut neighboring_boids = 0;
        for boid in other_boids {
            close_dx += self.x - boid.x;
            close_dy += self.y - boid.y;

            let distance = ((self.x - boid.x).powi(2) + (self.y - boid.y).powi(2)).sqrt();

            if distance < Boid::VISIBLE_RANGE {
                xvel_avg += boid.vx;
                yvel_avg += boid.vy;
                neighboring_boids += 1;
            }
        }

        if neighboring_boids > 0 {
            xvel_avg = xvel_avg / neighboring_boids as f32;
            yvel_avg = yvel_avg / neighboring_boids as f32;
            self.vx += (xvel_avg - self.vx) * Boid::MATCHING_FACTOR;
            self.vy += (yvel_avg - self.vy) * Boid::MATCHING_FACTOR;
        }

        if self.x < 0.0 {
            self.vx += Boid::TURN_FACTOR;
        }

        if self.y < 0.0 {
            self.vy += Boid::TURN_FACTOR;
        }

        if self.x > width {
            self.vx -= Boid::TURN_FACTOR;
        }

        if self.y > height {
            self.vy -= Boid::TURN_FACTOR;
        }

        self.vx += close_dx * Boid::AVOID_FACTOR;
        self.vy += close_dy * Boid::AVOID_FACTOR;
    }
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;

        let boids = (0..20).map(|_| Boid::new(0.0, 0.0)).collect();

        Self {
            width,
            height,
            boids,
        }
    }
    pub fn tick(&mut self) {
        let snapshot = self.boids.clone();

        for boid in &mut self.boids {
            boid.update(&snapshot, self.width as f32, self.height as f32);
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
