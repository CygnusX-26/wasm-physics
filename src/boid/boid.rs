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
    render_xy: Vec<f32>,
}

impl Boid {
    const AVOID_FACTOR: f32 = 0.05;
    const VISIBLE_RANGE: f32 = 40.0;
    const MATCHING_FACTOR: f32 = 0.04;
    const TURN_FACTOR: f32 = 0.4;
    const MAX_SPEED: f32 = 4.0;
    const MIN_SPEED: f32 = 0.8;
    const CENTERING_FACTOR: f32 = 0.0008;
    const PROTECTED_RANGE: f32 = 10.0;
    const START_SPEED: f32 = 1.5;
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: Boid::START_SPEED,
            vy: Boid::START_SPEED,
        }
    }

    pub fn update(&mut self, other_boids: &[Boid], width: f32, height: f32) {
        let mut close_dx = 0.0;
        let mut close_dy = 0.0;
        let mut xvel_avg = 0.0;
        let mut yvel_avg = 0.0;
        let mut xpos_avg = 0.0;
        let mut ypos_avg = 0.0;
        let mut neighboring_boids = 0;
        for boid in other_boids {
            let dx = self.x - boid.x;
            let dy = self.y - boid.y;
            if dx == 0.0 && dy == 0.0 {
                continue;
            }

            if dx.abs() < Boid::VISIBLE_RANGE && dy.abs() < Boid::VISIBLE_RANGE {
                let squared_distance = dx * dx + dy * dy;
                if squared_distance < Boid::PROTECTED_RANGE * Boid::PROTECTED_RANGE {
                    close_dx += dx;
                    close_dy += dy;
                } else if squared_distance < Boid::VISIBLE_RANGE * Boid::VISIBLE_RANGE {
                    xvel_avg += boid.vx;
                    yvel_avg += boid.vy;
                    xpos_avg += boid.x;
                    ypos_avg += boid.y;
                    neighboring_boids += 1;
                }
            }
        }

        if neighboring_boids > 0 {
            xvel_avg = xvel_avg / neighboring_boids as f32;
            yvel_avg = yvel_avg / neighboring_boids as f32;
            xpos_avg = xpos_avg / neighboring_boids as f32;
            ypos_avg = ypos_avg / neighboring_boids as f32;
            self.vx += (xvel_avg - self.vx) * Boid::MATCHING_FACTOR;
            self.vy += (yvel_avg - self.vy) * Boid::MATCHING_FACTOR;
            self.vx += (xpos_avg - self.x) * Boid::CENTERING_FACTOR;
            self.vy += (ypos_avg - self.y) * Boid::CENTERING_FACTOR;
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
        let speed = (self.vx.powi(2) + self.vy.powi(2)).sqrt();
        if speed > Boid::MAX_SPEED {
            self.vx = (self.vx / speed) * Boid::MAX_SPEED;
            self.vy = (self.vy / speed) * Boid::MAX_SPEED;
        } else if speed < Boid::MIN_SPEED {
            self.vx = (self.vx / speed) * Boid::MIN_SPEED;
            self.vy = (self.vy / speed) * Boid::MIN_SPEED;
        }

        self.x += self.vx;
        self.y += self.vy;
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, height: u32, num_boids: u32) -> Self {
        let boids = (0..num_boids)
            .map(|num| Boid::new(250.0, -20.0 * num as f32))
            .collect();

        Self {
            width,
            height,
            boids,
            render_xy: vec![0.0; (num_boids * 2) as usize],
        }
    }
    pub fn tick(&mut self) {
        let snapshot = self.boids.clone();

        for boid in &mut self.boids {
            boid.update(&snapshot, self.width as f32, self.height as f32);
        }

        for (i, boid) in self.boids.iter().enumerate() {
            self.render_xy[i * 2] = boid.x;
            self.render_xy[i * 2 + 1] = boid.y;
        }
    }

    pub fn render_xy_ptr(&self) -> *const f32 {
        self.render_xy.as_ptr()
    }

    pub fn render_xy_len(&self) -> usize {
        self.render_xy.len()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
