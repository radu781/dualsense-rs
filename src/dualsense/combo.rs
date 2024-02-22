use std::time::{Duration, Instant};

use crate::properties::property::ComboProperty;

pub(crate) struct Combo {
    pub(crate) cbs: Vec<ComboRequirement>,
    pub(crate) on_success: Box<dyn Fn() + Send + Sync>,
    pub(crate) duration: Duration,
    pub(crate) id: usize,
    start_time: Instant,
}

impl Combo {
    pub(crate) fn new(
        cbs: Vec<Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>>,
        on_success: Box<dyn Fn() + Send + Sync>,
        index: usize,
        duration: Duration,
    ) -> Self {
        let callbacks = cbs
            .into_iter()
            .map(|cb| ComboRequirement {
                cb,
                satisfied: false,
            })
            .collect::<Vec<_>>();
        Self {
            cbs: callbacks,
            on_success,
            duration,
            id: index,
            start_time: Instant::now(),
        }
    }

    pub(crate) fn next_input(&mut self, property: &ComboProperty) {
        for cb in self.cbs.iter_mut() {
            if (cb.cb)(property) {
                cb.satisfied = true;
            }
        }
        if self.satisfied() == 1 {
            self.start_time = Instant::now();
        } else if self.satisfied() == self.cbs.len() {
            if Instant::now().duration_since(self.start_time) <= self.duration {
                (self.on_success)();
            }
            self.reset();
        }
    }

    fn satisfied(&self) -> usize {
        self.cbs.iter().filter(|req| req.satisfied).count()
    }

    fn reset(&mut self) {
        self.cbs.iter_mut().for_each(|req| req.satisfied = false)
    }
}

pub(crate) struct ComboRequirement {
    cb: Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>,
    satisfied: bool,
}

impl PartialEq for Combo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Combo {}

pub struct ComboId {
    id: usize,
}

impl ComboId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
