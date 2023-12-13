// 0x01      0x80   0x81    0x82    0x7d    0x00  0x00 0x38
// reportID lstickX lstickY rstickX rstickY 8keys lrso
// 0x08 0x00 0x00 0x00 0x7e 0x88 0xac 0x64
// 0xfb 0xff 0x00 0x00 0x01 0x00 0x72 0x00
// 0xa3 0x1f 0xad 0x05 0xca 0xd3 0x9b 0xaf
// 0x0d 0x81 0xa0 0x70 0x43 0x80 0x00 0x00
// 0x00 0x2c 0x09 0x09 0x00 0x00 0x00 0x00
// 0x00 0x52 0xec 0x9b 0xaf 0x10 0x08 0x00
// 0xf8 0x4d 0xa2 0x26 0x0f 0x19 0x79 0x5a

use hidapi::{HidApi, HidDevice};
use std::{
    collections::HashMap,
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use crate::property_helpers::{Property, ValueType};

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
            sleep(Duration::from_millis(100));
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


    fn register_u8<F>(&mut self, prop: Property, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.callbacks
            .entry(prop)
            .or_insert_with(|| vec![])
            .push(Box::new(move |x| cb(x.to_u8())));
    }

    fn packet_received(&mut self, data: &[u8; 64]) {
        self.callbacks.iter().for_each(|(prop, cbs)| {
            let new_val = prop.convert(&data.as_slice()[prop.offset().byte]);
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
}
