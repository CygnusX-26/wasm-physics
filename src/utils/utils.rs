use js_sys::Math;

pub struct Utils {}

impl Utils {
    pub fn random_f32() -> f32 {
        Math::random() as f32
    }

    pub fn random_f64() -> f64 {
        Math::random()
    }
}
