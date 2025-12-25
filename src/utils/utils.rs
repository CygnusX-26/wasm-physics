use js_sys::Math;

pub struct Utils {}

impl Utils {
    pub fn js_random_f32() -> f32 {
        Math::random() as f32
    }
}
