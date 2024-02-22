/// Applied to all structs that can be normalized (analog sticks, triggers, touchpad)
pub trait Normalizable {
    fn normalize(&self) -> f32;
}
