use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::utils::Utils;

#[derive(Clone)]
struct Boid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

struct WorldParams {
    pub avoid_factor: f32,
    pub visible_range: f32,
    pub matching_factor: f32,
    pub turn_factor: f32,
    pub max_speed: f32,
    pub min_speed: f32,
    pub centering_factor: f32,
    pub protected_range: f32,
    pub predator_turn_factor: f32,
    pub predator_range: f32,
}

struct Predator {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    boids: Vec<Boid>,
    render_xy: Vec<f32>,
    predator: Option<Predator>,
    params: WorldParams,
}

impl Predator {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: (Utils::js_random_f32() - 0.5) * 20.0,
            vy: (Utils::js_random_f32() - 0.5) * 20.0,
        }
    }

    pub fn update(
        &mut self,
        other_boids: &[Boid],
        width: f32,
        height: f32,
        predator: &Option<Predator>,
        params: &WorldParams,
    ) {
        let mut close_dx = 0.0;
        let mut close_dy = 0.0;
        let mut xvel_avg = 0.0;
        let mut yvel_avg = 0.0;
        let mut xpos_avg = 0.0;
        let mut ypos_avg = 0.0;
        let mut neighboring_boids = 0;
        let mut num_predators = 0;
        let mut predator_dx = 0.0;
        let mut predator_dy = 0.0;
        for boid in other_boids {
            let dx = self.x - boid.x;
            let dy = self.y - boid.y;
            if dx == 0.0 && dy == 0.0 {
                continue;
            }

            if dx.abs() < params.visible_range && dy.abs() < params.visible_range {
                let squared_distance = dx * dx + dy * dy;
                if squared_distance < params.protected_range.powi(2) {
                    close_dx += dx;
                    close_dy += dy;
                } else if squared_distance < params.visible_range.powi(2) {
                    xvel_avg += boid.vx;
                    yvel_avg += boid.vy;
                    xpos_avg += boid.x;
                    ypos_avg += boid.y;
                    neighboring_boids += 1;
                }
            }
        }

        if let Some(predator) = predator {
            let dx = self.x - predator.x;
            let dy = self.y - predator.y;

            if dx.abs() < params.predator_range && dy.abs() < params.predator_range {
                let squared_distance = dx * dx + dy * dy;

                if squared_distance < params.predator_range.powi(2) {
                    predator_dx += dx;
                    predator_dy += dy;
                    num_predators += 1;
                }
            }
        }

        if num_predators > 0 {
            if predator_dy > 0.0 {
                self.vy += params.predator_turn_factor;
            }
            if predator_dy < 0.0 {
                self.vy -= params.predator_turn_factor;
            }
            if predator_dx > 0.0 {
                self.vx += params.predator_turn_factor;
            }
            if predator_dx < 0.0 {
                self.vx -= params.predator_turn_factor;
            }
        }

        if neighboring_boids > 0 {
            xvel_avg = xvel_avg / neighboring_boids as f32;
            yvel_avg = yvel_avg / neighboring_boids as f32;
            xpos_avg = xpos_avg / neighboring_boids as f32;
            ypos_avg = ypos_avg / neighboring_boids as f32;
            self.vx += (xvel_avg - self.vx) * params.matching_factor;
            self.vy += (yvel_avg - self.vy) * params.matching_factor;
            self.vx += (xpos_avg - self.x) * params.centering_factor;
            self.vy += (ypos_avg - self.y) * params.centering_factor;
        }

        let margin_x = width * 0.15;
        let margin_y = height * 0.15;
        if self.x < 0.0 + margin_x {
            self.vx += params.turn_factor;
        }

        if self.y < 0.0 + margin_y {
            self.vy += params.turn_factor;
        }

        if self.x > width - margin_x {
            self.vx -= params.turn_factor;
        }

        if self.y > height - margin_y {
            self.vy -= params.turn_factor;
        }

        self.vx += close_dx * params.avoid_factor;
        self.vy += close_dy * params.avoid_factor;
        let speed = (self.vx.powi(2) + self.vy.powi(2)).sqrt();
        if speed > params.max_speed {
            self.vx = (self.vx / speed) * params.max_speed;
            self.vy = (self.vy / speed) * params.max_speed;
        } else if speed < params.min_speed {
            self.vx = (self.vx / speed) * params.min_speed;
            self.vy = (self.vy / speed) * params.min_speed;
        }

        self.x += self.vx;
        self.y += self.vy;
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(
        width: u32,
        height: u32,
        num_boids: u32,
        avoid_factor: f32,
        visible_range: f32,
        matching_factor: f32,
        turn_factor: f32,
        min_speed: f32,
        max_speed: f32,
        centering_factor: f32,
        protected_range: f32,
        predator_turn_factor: f32,
        predator_range: f32,
    ) -> Self {
        let boids = (0..num_boids)
            .map(|_| {
                Boid::new(
                    Utils::js_random_f32() * width as f32,
                    Utils::js_random_f32() * height as f32,
                )
            })
            .collect();
        Self {
            width,
            height,
            boids,
            render_xy: vec![0.0; (num_boids * 2) as usize],
            predator: None,
            params: WorldParams {
                avoid_factor,
                visible_range,
                matching_factor,
                turn_factor,
                max_speed,
                min_speed,
                centering_factor,
                protected_range,
                predator_turn_factor,
                predator_range,
            },
        }
    }
    pub fn tick(&mut self) {
        let snapshot = self.boids.clone();

        for boid in &mut self.boids {
            boid.update(
                &snapshot,
                self.width as f32,
                self.height as f32,
                &self.predator,
                &self.params,
            );
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

    pub fn set_predator_loc(&mut self, x: f32, y: f32) {
        if let Some(predator) = self.predator.as_mut() {
            predator.x = x;
            predator.y = y;
        } else {
            self.predator = Some(Predator::new(x, y));
        }
    }

    pub fn update_avoid_factor(&mut self, avoid_factor: f32) {
        self.params.avoid_factor = avoid_factor;
    }
    pub fn update_centering_factor(&mut self, centering_factor: f32) {
        self.params.centering_factor = centering_factor;
    }
    pub fn update_protected_range(&mut self, protected_range: f32) {
        self.params.protected_range = protected_range;
    }
}
