use std::time::{Duration, Instant};

use crate::properties::property::ComboProperty;

pub struct Combo {
    pub(crate) cbs: Vec<ComboRequirement>,
    pub(crate) on_success: Box<dyn Fn() + Send + Sync>,
    pub(crate) duration: Duration,
    pub(crate) id: ComboId,
    start_time: Instant,
}

impl Combo {
    pub(crate) fn new(
        cbs: Vec<Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>>,
        on_success: Box<dyn Fn() + Send + Sync>,
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
            id: ComboId { id: 0 },
            start_time: Instant::now(),
        }
    }

    pub(crate) fn with_id(mut self, id: ComboId) -> Self {
        self.id = id;
        self
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
    pub(crate) cb: Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>,
    pub(crate) satisfied: bool,
}

impl PartialEq for Combo {
    fn eq(&self, other: &Self) -> bool {
        self.id.id == other.id.id
    }
}

impl Eq for Combo {}

#[derive(Clone, Copy)]
pub struct ComboId {
    id: usize,
}

impl ComboId {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}
