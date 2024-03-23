use std::time::Duration;

use crate::combo::Combo;

use super::{property::ComboProperty, traits::ComboAble};

pub struct SimultaneousCombo {
    pub(crate) cbs: Vec<Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>>,
    pub(crate) on_success: Box<dyn Fn() + Send + Sync>,
    pub(crate) duration: Duration,
}

impl ComboAble for SimultaneousCombo {
    fn key(mut self, key: Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>) -> Self {
        self.cbs.push(key);
        self
    }

    fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
    

    fn done(mut self, cb: Box<dyn Fn() + Send + Sync>) -> Self {
        self.on_success = cb;
        self
    }
    
    

    fn build(self) -> Combo {
        Combo::new(self.cbs, self.on_success, self.duration)
    }
    
    fn cooldown(self, duration: Duration) -> Self {
        todo!()
    }
}

impl Default for SimultaneousCombo {
    fn default() -> Self {
        Self {
            cbs: Default::default(),
            on_success: Box::new(|| {}),
            duration: Duration::from_millis(250),
        }
    }
}

pub struct SequentialCombo {}

impl SequentialCombo {}
