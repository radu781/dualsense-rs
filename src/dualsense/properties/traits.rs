use std::time::Duration;

use crate::combo::Combo;

use super::property::ComboProperty;

/// Applied to all structs that can be normalized (analog sticks, triggers, touchpad)
pub trait Normalizable {
    fn normalize(&self) -> f32;
}

pub trait ComboAble {
    /// Register a key for the combo
    fn key(self, key: Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>) -> Self;

    /// Add maximum duration until the next key or until all keys are pressed. If it is exceeded, the combo progress
    /// is reset
    fn duration(self, duration: Duration) -> Self;

    /// What to do when the combo is done
    fn done(self, cb: Box<dyn Fn() + Send + Sync>) -> Self;

    /// Time until the combo can be used again, the callback given to `done` will not be called if the combo is
    /// successful but this duration hasn't passed
    fn cooldown(self, duration: Duration) -> Self;

    /// Make the combo object and use it to register
    fn build(self) -> Combo;
}
