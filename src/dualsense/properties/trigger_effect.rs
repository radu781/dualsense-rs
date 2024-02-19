/// Vibration patterns
#[derive(Clone, Copy)]
pub enum TriggerEffect {
    Off,
    Mode1,
    Mode2,
    Mode1_4,
    Mode2_4,
    Mode1_20,
    Mode1_4_20,
    Mode2_4_20,
    Calibration,
}

impl TriggerEffect {
    pub(crate) fn byte(self) -> u8 {
        match self {
            TriggerEffect::Off => 0x00,
            TriggerEffect::Mode1 => 0x01,
            TriggerEffect::Mode2 => 0x02,
            TriggerEffect::Mode1_4 => 0x05,
            TriggerEffect::Mode2_4 => 0x06,
            TriggerEffect::Mode1_20 => 0x21,
            TriggerEffect::Mode1_4_20 => 0x25,
            TriggerEffect::Mode2_4_20 => 0x26,
            TriggerEffect::Calibration => 0xFC,
        }
    }
}
