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

/// Main struct used for interacting with the controller
pub struct DualSense {
    device: HidDevice,
    callbacks: HashMap<Property, Vec<Box<dyn Fn(ValueType) + Send>>>,
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
            sleep(Duration::from_millis(1000));
        })
    }

    /// Provide a callback to be called when the left stick's x coordinate changes
    pub fn on_left_pad_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::LeftPadX, cb);
    }

    /// Provide a callback to be called when the left stick's y coordinate changes
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

    /// Provide a callback to be called when the L2 button changes
    pub fn on_l2_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::L2, cb);
    }

    /// Provide a callback to be called when the R2 button changes
    pub fn on_r2_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(Property::R2, cb);
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

    pub fn on_up_pressed<F>(&mut self, cb: &'static F)
    where
        F: Fn(DPad) + Send + Sync,
    {
        unimplemented!("trying to figure out how to send `cb` into register_dpad")
    }

    pub fn on_upright_pressed<F>(&mut self, cb: &'static F)
    where
        F: Fn(DPad) + Send + Sync,
    {
        unimplemented!("trying to figure out how to send `cb` into register_dpad")
    }

    fn register_u8<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_insert_with(|| vec![])
            .push(Box::new(move |x| cb(x.to_u8())));
    }

    fn register_dpad<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(DPad) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_insert_with(|| vec![])
            .push(Box::new(move |x| cb(x.to_dpad())));
    }

    fn register_symbols<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(Symbols) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_insert_with(|| vec![])
            .push(Box::new(move |x| cb(x.to_symbol())));
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

    fn extract_bytes(prop: &Property, data: &[u8; 64]) -> ValueType {
        if prop.offset().bit == (0..8) {
            prop.convert(&data.as_slice()[prop.offset().byte])
        } else if prop.offset().byte.count() == 1 {
            let mut out = 0u8;
            let byte = prop.offset().byte.start;
            let val = data.as_slice()[byte];

            for i in prop.offset().bit {
                let offset = i - prop.offset().bit.start;
                let current_bit = (val & (1 << i)) >> i;
                out = out | (current_bit << offset);
            }
            prop.convert(&[out])
        } else {
            todo!()
        }
    }
}
