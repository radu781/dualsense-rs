use super::traits::Normalizable;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Trigger(u8);

impl Trigger {
    pub(crate)fn new(value: u8) -> Self {
        Self { 0: value }
    }
}
impl Normalizable for Trigger {
    fn normalize(&self) -> f32 {
        self.0 as f32 / 255.0
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Self { 0: 0 }
    }
}
