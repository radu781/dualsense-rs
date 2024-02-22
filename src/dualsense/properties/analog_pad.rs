use super::traits::Normalizable;

const ANALOG_PAD_MIDDLE: u8 = u8::MAX / 2;
/// About 90
// const QUADRANT_ANGLE: u8 = (45_f64.cos() * PI / 180.0) as u8 * ANALOG_PAD_MIDDLE;
const QUADRANT_ANGLE: u8 = 90;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AnalogPad {
    x: Stick,
    y: Stick,
}

impl AnalogPad {
    pub(crate) fn new(x: u8, y: u8) -> Self {
        AnalogPad {
            x: Stick(x),
            y: Stick(y),
        }
    }

    ///  \N/
    /// W X E
    ///  /S\
    pub fn direction_quadrant(&self) -> DirectionQuadrant {
        if self.in_dead_zone() {
            DirectionQuadrant::DeadZone
        } else if Self::in_quadrant(self.x.0) {
            if self.y.0 >= ANALOG_PAD_MIDDLE {
                DirectionQuadrant::South
            } else {
                DirectionQuadrant::North
            }
        } else if Self::in_quadrant(self.y.0) {
            if self.x.0 >= ANALOG_PAD_MIDDLE {
                DirectionQuadrant::East
            } else {
                DirectionQuadrant::West
            }
        } else {
            unreachable!()
        }
    }

    /// Normalize values to the [-1, 1] interval
    pub fn normalize(&self) -> (f32, f32) {
        (self.x.normalize(), -self.y.normalize())
    }

    fn in_quadrant(value: u8) -> bool {
        (-180..=180).contains(&(value as i32 - 127))
    }

    fn in_dead_zone(&self) -> bool {
        (ANALOG_PAD_MIDDLE - 10..=ANALOG_PAD_MIDDLE + 10).contains(&self.x.0)
            && (ANALOG_PAD_MIDDLE - 10..=ANALOG_PAD_MIDDLE + 10).contains(&self.y.0)
    }
}

impl Default for AnalogPad {
    fn default() -> Self {
        Self {
            x: Stick(0),
            y: Stick(0),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Stick(pub u8);

impl Normalizable for Stick {
    fn normalize(&self) -> f32 {
        (2.0 * self.0 as f32) / 255.0 - 1.0
    }
}

#[derive(Debug)]
pub enum DirectionQuadrant {
    North,
    East,
    South,
    West,
    DeadZone,
}
