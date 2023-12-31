use hidapi::{HidApi, HidDevice};
use std::{
    collections::HashMap,
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use crate::properties::{dpad::DPad, property::Property, symbols::Symbols, valuetype::ValueType};

const VENDOR_ID: u16 = 1356;
const PRODUCT_ID: u16 = 3302;
const PACKET_SIZE: usize = 64;

type CBFunction = Box<dyn Fn(ValueType) + Send>;
/// Main struct used for interacting with the controller
pub struct DualSense {
    device: HidDevice,
    callbacks: HashMap<Property, Vec<CBFunction>>,
    cache: HashMap<Property, ValueType>,
}

impl DualSense {
    pub fn new() -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open(VENDOR_ID, PRODUCT_ID).unwrap();
        Self {
            device,
            callbacks: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    /// Start listening to HID packets from the controller
    pub fn run(mut self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            let mut buf = [0u8; PACKET_SIZE];
            let bytes_read = self.device.read(&mut buf);
            match bytes_read {
                Ok(PACKET_SIZE) => {}
                Ok(actual_size) => {
                    eprintln!("Packet size mismatch, ignoring values ({actual_size})");
                    continue;
                }
                Err(e) => {
                    eprintln!("Error on read, ignoring values {e}");
                    continue;
                }
            }

            self.packet_received(&buf);
            sleep(Duration::from_millis(200));
        })
    }

    #[allow(dead_code)]
    fn write(&mut self, _data: u8) {
        // self.device.write(data);
    }

    /// Provide a callback to be called when the left stick's x coordinate changes
    /// left: 0x00, right: 0xFF
    pub fn on_left_pad_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::LeftPadX, cb);
    }

    /// Provide a callback to be called when the left stick's y coordinate changes
    /// up: 0x00, down: 0xFF
    pub fn on_left_pad_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::LeftPadY, cb);
    }

    /// Provide a callback to be called when the right stick's x coordinate changes
    pub fn on_right_pad_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::RightPadX, cb);
    }

    /// Provide a callback to be called when the right stick's y coordinate changes
    pub fn on_right_pad_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::RightPadY, cb);
    }

    /// Provide a callback to be called when the L1 button is pressed
    pub fn on_l1_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::L1, cb);
    }

    /// Provide a callback to be called when the R1 button is pressed
    pub fn on_r1_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::R1, cb);
    }

    /// Provide a callback to be called when the L2 button value changes
    pub fn on_l2_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::L2, cb);
    }

    /// Provide a callback to be called when the R2 button value changes
    pub fn on_r2_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::R2, cb);
    }

    /// Provide a callback to be called when the L3 button is pressed
    pub fn on_l3_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::L3, cb);
    }

    /// Provide a callback to be called when the R3 button is pressed
    pub fn on_r3_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::R3, cb);
    }

    /// Provide a callback to be called when the options button is pressed
    pub fn on_share_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::Share, cb);
    }

    /// Provide a callback to be called when the options button is pressed
    pub fn on_options_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::Options, cb);
    }

    /// Provide a callback to be called when any dpad button is pressed
    pub fn on_dpad_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(DPad) + Send + Sync,
    {
        self.register_dpad(Property::DPad, cb);
    }

    /// Provide a callback to be called when any symbol button is pressed
    pub fn on_symbols_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(Symbols) + Send + Sync,
    {
        self.register_symbols(Property::Symbols, cb);
    }

    /// Provide a callback to be called when the mute button is pressed
    pub fn on_mute_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::Mute, cb);
    }

    /// Provide a callback to be called when the touchpad is pressed
    pub fn on_touchpad_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::TouchPad, cb);
    }

    /// Provide a callback to be called when the playstation button is pressed
    pub fn on_playstation_pressed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::PlayStation, cb);
    }

    /// Provide a callback to be called when the gyroscope X axis is changed
    pub fn on_gyro_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::GyroscopeX, cb);
    }

    /// Provide a callback to be called when the gyroscope Y axis is changed
    pub fn on_gyro_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::GyroscopeY, cb);
    }

    /// Provide a callback to be called when the gyroscope Z axis is changed
    pub fn on_gyro_z_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::GyroscopeZ, cb);
    }

    /// Provide a callback to be called when the acceleration X axis is changed
    pub fn on_accel_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::AccelerationX, cb);
    }

    /// Provide a callback to be called when the acceleration Y axis is changed
    pub fn on_accel_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::AccelerationY, cb);
    }

    /// Provide a callback to be called when the acceleration Z axis is changed
    pub fn on_accel_z_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::AccelerationZ, cb);
    }

    /// Provide a callback to be called when the touchpad is touched
    pub fn on_touchpad1_pressed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::TouchPadFinger1Active, cb);
    }

    /// Provide a callback to be called when the touchpad is touched with the second finger
    pub fn on_touchpad2_pressed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(Property::TouchPadFinger2Active, cb);
    }

    /// Provide a callback to be called when the touchpad ID changes
    pub fn on_touchpad1_id_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::TouchPad1Id, cb);
    }
    /// Provide a callback to be called when the touchpad ID changes
    pub fn on_touchpad2_id_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::TouchPad2Id, cb);
    }
    /// Provide a callback to be called when the touchpad input from the first finger
    /// on the X axis is changed
    pub fn on_touchpad1_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::TouchPad1X, cb);
    }

    /// Provide a callback to be called when the touchpad input from the first finger
    /// on the Y axis is changed
    pub fn on_touchpad1_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::TouchPad1Y, cb);
    }

    /// Provide a callback to be called when the touchpad input from the second finger
    /// on the X axis is changed
    pub fn on_touchpad2_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::TouchPad2X, cb);
    }

    /// Provide a callback to be called when the touchpad input from the second finger
    /// on the Y axis is changed
    pub fn on_touchpad2_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(Property::TouchPad2Y, cb);
    }

    fn register_u8<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_u8())));
    }

    fn register_u16<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_u16())));
    }

    fn register_dpad<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(DPad) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_dpad())));
    }

    fn register_symbols<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(Symbols) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_symbol())));
    }

    fn register_bool<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_bool())));
    }

    fn packet_received(&mut self, data: &[u8; 64]) {
        self.callbacks.iter().for_each(|(prop, cbs)| {
            let new_val = Self::extract_bytes(prop, data);
            let mut update = false;

            match self.cache.get_mut(prop) {
                Some(old_val) if old_val != &new_val => {
                    update = true;
                }
                None => {
                    update = true;
                }
                _ => {}
            }
            if update {
                self.cache.insert(*prop, new_val);
                cbs.iter().for_each(|cb| cb(new_val));
            }
        })
    }

    #[allow(dead_code)]
    fn debug_print_packet(data: &[u8; PACKET_SIZE]) {
        data.chunks(8).for_each(|w| {
            for b in w {
                print!("{:#04x} ", b)
            }
            println!();
        });
        println!()
    }

    fn extract_bytes(prop: &Property, data: &[u8; 64]) -> ValueType {
        if prop.offset().bits == (0..8) {
            prop.convert(&data.as_slice()[prop.offset().bytes])
        } else if prop.offset().bytes.count() == 1 {
            let mut out = 0u8;
            let byte = prop.offset().bytes.start;
            let val = data.as_slice()[byte];

            for i in prop.offset().bits {
                let offset = i - prop.offset().bits.start;
                let current_bit = (val & (1 << i)) >> i;
                out |= current_bit << offset;
            }
            prop.convert(&[out])
        } else {
            todo!()
        }
    }
}

impl Default for DualSense {
    fn default() -> Self {
        DualSense::new()
    }
}
