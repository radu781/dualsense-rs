use hidapi::{HidApi, HidDevice};
use std::{
    collections::HashMap,
    ffi::CString,
    sync::{Arc, Mutex},
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use crate::{
    combo::{Combo, ComboId},
    properties::{
        analog_pad::AnalogPad,
        dpad::DPad,
        property::{ComboProperty, InputProperty, OutputProperty},
        symbols::Symbols,
        trigger::Trigger,
        trigger_effect::TriggerEffect,
        valuetype::ValueType,
    },
};

const VENDOR_ID: u16 = 1356;
const PRODUCT_ID: u16 = 3302;
const PACKET_SIZE: usize = 64;

type CBFunction = Box<dyn FnMut(ValueType) + Send>;
type Artex<T> = Arc<Mutex<T>>;

/// Main struct used for interacting with the controller. Everything is thread safe to allow reading, writing,
/// setting callbacks after the `Self::run` method is called and to send data.
pub struct DualSense {
    device: Artex<HidDevice>,
    callbacks: Artex<HashMap<InputProperty, Vec<CBFunction>>>,
    callbacks_v2: Artex<HashMap<ComboProperty, Vec<CBFunction>>>,
    callback_cache_v2: Artex<HashMap<ComboProperty, ValueType>>,
    output_cache: Artex<HashMap<OutputProperty, u8>>,
    output_cache_changed: Artex<bool>,
    combos: Artex<Vec<Combo>>,
}

impl DualSense {
    pub fn new_serial(vendor_id: u16, product_id: u16, serial: &str) -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open_serial(vendor_id, product_id, serial).unwrap();
        Self::new_with_device(device)
    }

    pub fn new_path(path: &str) -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open_path(&CString::new(path).unwrap()).unwrap();
        Self::new_with_device(device)
    }

    pub fn list_devices() -> Vec<hidapi::DeviceInfo> {
        let api = HidApi::new().unwrap();
        let mut out = Vec::new();
        for device in api.device_list() {
            out.push(device.clone());
        }
        out
    }

    fn new_with_device(device: HidDevice) -> Self {
        Self {
            device: Arc::new(Mutex::new(device)),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            callback_cache_v2: Arc::new(Mutex::new(HashMap::new())),
            output_cache: Arc::new(Mutex::new(HashMap::new())),
            output_cache_changed: Arc::new(Mutex::new(false)),
            combos: Arc::new(Mutex::new(Vec::new())),
            callbacks_v2: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start listening to HID packets from the controller
    pub fn run(&mut self) -> JoinHandle<()> {
        let device = Arc::clone(&self.device);
        let callbacks_v2 = Arc::clone(&self.callbacks_v2);
        let cache_v2 = Arc::clone(&self.callback_cache_v2);
        let output_cache = Arc::clone(&self.output_cache);
        let output_cache_changed = Arc::clone(&self.output_cache_changed);
        let combos = Arc::clone(&self.combos);

        thread::spawn(move || loop {
            let mut buf = [0u8; PACKET_SIZE];
            let bytes_read = device.lock().unwrap().read(&mut buf);
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

            Self::packet_received_v2(
                &mut callbacks_v2.lock().unwrap(),
                &mut cache_v2.lock().unwrap(),
                &mut combos.lock().unwrap(),
                &buf,
            );
            if *output_cache_changed.lock().unwrap() {
                Self::write(&device.lock().unwrap(), &output_cache.lock().unwrap());
                *output_cache_changed.lock().unwrap() = false;
            }
            sleep(Duration::from_millis(50));
        })
    }

    pub fn set_light_red(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::Red, value);
    }

    pub fn set_light_green(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::Green, value);
    }

    pub fn set_light_blue(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::Blue, value);
    }

    /// Provide a callback to be called when the left stick's coordinates change
    pub fn on_left_pad_changed(&mut self, cb: Box<dyn FnMut(AnalogPad) + Send>) {
        self.register_pad(ComboProperty::LeftPad(AnalogPad::default()), cb);
    }

    /// Provide a callback to be called when the right stick's coordinates change
    pub fn on_right_pad_changed(&mut self, cb: Box<dyn FnMut(AnalogPad) + Send>) {
        self.register_pad(ComboProperty::RightPad(AnalogPad::default()), cb);
    }

    /// Provide a callback to be called when the L1 button is pressed
    pub fn on_l1_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::L1, cb);
    }

    /// Provide a callback to be called when the R1 button is pressed
    pub fn on_r1_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::R1, cb);
    }

    /// Provide a callback to be called when the L2 button value changes
    pub fn on_l2_changed(&mut self, cb: Box<dyn FnMut(Trigger) + Send>) {
        self.register_trigger(ComboProperty::LT(Trigger::default()), cb);
    }

    /// Provide a callback to be called when the R2 button value changes
    pub fn on_r2_changed(&mut self, cb: Box<dyn FnMut(Trigger) + Send>) {
        self.register_trigger(ComboProperty::RT(Trigger::default()), cb);
    }

    /// Provide a callback to be called when the L3 button is pressed
    pub fn on_l3_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::L3, cb);
    }

    /// Provide a callback to be called when the R3 button is pressed
    pub fn on_r3_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::R3, cb);
    }

    /// Provide a callback to be called when the options button is pressed
    pub fn on_share_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::Share, cb);
    }

    /// Provide a callback to be called when the options button is pressed
    pub fn on_options_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::Options, cb);
    }

    /// Provide a callback to be called when any dpad button is pressed
    pub fn on_dpad_changed(&mut self, cb: Box<dyn FnMut(DPad) + Send>) {
        self.register_dpad(ComboProperty::DPad(DPad::default()), cb);
    }

    /// Provide a callback to be called when any symbol button is pressed
    pub fn on_symbols_changed(&mut self, cb: Box<dyn FnMut(Symbols) + Send>) {
        self.register_symbols(ComboProperty::Symbol(Symbols::default()), cb);
    }

    /// Provide a callback to be called when the mute button is pressed
    pub fn on_mute_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::Mute, cb);
    }

    /// Provide a callback to be called when the touchpad is pressed
    pub fn on_touchpad_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::TouchPad, cb);
    }

    /// Provide a callback to be called when the playstation button is pressed
    pub fn on_playstation_pressed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::PlayStation, cb);
    }

    /// Provide a callback to be called when the gyroscope X axis is changed
    pub fn on_gyro_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(i16) + Send + Sync,
    {
        self.register_i16(InputProperty::GyroscopeX, cb);
    }

    /// Provide a callback to be called when the gyroscope Y axis is changed
    pub fn on_gyro_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(i16) + Send + Sync,
    {
        self.register_i16(InputProperty::GyroscopeY, cb);
    }

    /// Provide a callback to be called when the gyroscope Z axis is changed
    pub fn on_gyro_z_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(i16) + Send + Sync,
    {
        self.register_i16(InputProperty::GyroscopeZ, cb);
    }

    /// Provide a callback to be called when the acceleration X axis is changed
    pub fn on_accel_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(i16) + Send + Sync,
    {
        self.register_i16(InputProperty::AccelerationX, cb);
    }

    /// Provide a callback to be called when the acceleration Y axis is changed
    pub fn on_accel_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(i16) + Send + Sync,
    {
        self.register_i16(InputProperty::AccelerationY, cb);
    }

    /// Provide a callback to be called when the acceleration Z axis is changed
    pub fn on_accel_z_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(i16) + Send + Sync,
    {
        self.register_i16(InputProperty::AccelerationZ, cb);
    }

    /// Provide a callback to be called when the touchpad is touched
    pub fn on_touchpad1_pressed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::TouchPadFinger1Active, cb);
    }

    /// Provide a callback to be called when the touchpad is touched with the second finger
    pub fn on_touchpoint2_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::TouchPadFinger2Active, cb);
    }

    /// Provide a callback to be called when the touchpad ID changes
    pub fn on_touchpoint1_id_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(InputProperty::TouchPad1Id, cb);
    }
    /// Provide a callback to be called when the touchpad ID changes
    pub fn on_touchpoint2_id_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(InputProperty::TouchPad2Id, cb);
    }
    /// Provide a callback to be called when the touchpad input from the first finger
    /// on the X axis is changed
    pub fn on_touchpad1_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(InputProperty::TouchPad1X, cb);
    }

    /// Provide a callback to be called when the touchpad input from the first finger
    /// on the Y axis is changed
    pub fn on_touchpad1_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(InputProperty::TouchPad1Y, cb);
    }

    /// Provide a callback to be called when the touchpad input from the second finger
    /// on the X axis is changed
    pub fn on_touchpoint2_x_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(InputProperty::TouchPad2X, cb);
    }

    /// Provide a callback to be called when the touchpad input from the second finger
    /// on the Y axis is changed
    pub fn on_touchpoint2_y_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.register_u16(InputProperty::TouchPad2Y, cb);
    }

    /// Provide a callback to be called when the left stick force trigger is active
    pub fn on_left_force_enabled<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::L2FeedbackOn, cb);
    }

    /// Provide a callback to be called when the right stick force trigger is active
    pub fn on_right_force_enabled<F>(&mut self, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.register_bool(InputProperty::R2FeedbackOn, cb);
    }

    /// Provide a callback to be called when the left stick force amount changed
    pub fn on_left_force_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(InputProperty::L2FeedbackValue, cb);
    }

    /// Provide a callback to be called when the right stick force trigger is active
    pub fn on_right_force_changed<F>(&mut self, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.register_u8(InputProperty::R2FeedbackValue, cb);
    }

    fn register_pad(&mut self, prop: ComboProperty, mut cb: Box<dyn FnMut(AnalogPad) + Send>) {
        self.callbacks_v2
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_analog())));
    }

    fn register_symbols(&mut self, prop: ComboProperty, mut cb: Box<dyn FnMut(Symbols) + Send>) {
        self.callbacks_v2
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_symbol())));
    }

    fn register_trigger(&mut self, prop: ComboProperty, mut cb: Box<dyn FnMut(Trigger) + Send>) {
        self.callbacks_v2
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_trigger())));
    }

    fn register_u8<F>(&mut self, prop: InputProperty, cb: &'static F)
    where
        F: Fn(u8) + Send + Sync,
    {
        self.callbacks
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_u8())));
    }

    fn register_u16<F>(&mut self, prop: InputProperty, cb: &'static F)
    where
        F: Fn(u16) + Send + Sync,
    {
        self.callbacks
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_u16())));
    }

    fn register_i16<F>(&mut self, prop: InputProperty, cb: &'static F)
    where
        F: Fn(i16) + Send + Sync,
    {
        self.callbacks
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_i16())));
    }

    fn register_dpad(&mut self, prop: ComboProperty, mut cb: Box<dyn FnMut(DPad) + Send>) {
        self.callbacks_v2
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_dpad())));
    }

    fn register_bool<F>(&mut self, prop: InputProperty, cb: &'static F)
    where
        F: Fn(bool) + Send + Sync,
    {
        self.callbacks
            .lock()
            .unwrap()
            .entry(prop)
            .or_default()
            .push(Box::new(move |x| cb(x.to_bool())));
    }

    fn packet_received_v2(
        callbacks: &mut HashMap<ComboProperty, Vec<CBFunction>>,
        cache: &mut HashMap<ComboProperty, ValueType>,
        combos: &mut [Combo],
        data: &[u8; 64],
    ) {
        callbacks.iter_mut().for_each(|(prop, cbs)| {
            let new_val = Self::extract_bytes_v2(prop, data);
            let mut update = false;
            match cache.get_mut(prop) {
                Some(old_val) if old_val != &new_val => {
                    update = true;
                }
                None => {
                    update = true;
                }
                _ => {}
            }

            if update {
                cache.insert(*prop, new_val);
                cbs.iter_mut().for_each(|cb| cb(new_val));
                for combo in combos.iter_mut() {
                    combo.next_input(prop);
                }
            }
        })
    }

    fn packet_received(
        callbacks: &mut HashMap<InputProperty, Vec<CBFunction>>,
        cache: &mut HashMap<InputProperty, ValueType>,
        combos: &mut [Combo],
        data: &[u8; 64],
    ) {
        callbacks.iter_mut().for_each(|(prop, cbs)| {
            let new_val = Self::extract_bytes(prop, data);
            let mut update = false;

            match cache.get_mut(prop) {
                Some(old_val) if old_val != &new_val => {
                    update = true;
                }
                None => {
                    update = true;
                }
                _ => {}
            }
            if update {
                cache.insert(*prop, new_val);
                cbs.iter_mut().for_each(|cb| cb(new_val));
                for combo in combos.iter_mut() {
                    combo.next_input(&ComboProperty::Symbol(Symbols::Cross));
                    combo.next_input(&ComboProperty::LB);
                }
            }
        })
    }

    fn write(device: &HidDevice, output_cache: &HashMap<OutputProperty, u8>) {
        let mut data = [0_u8; 48];
        data[0] = 0x02;
        data[1] = 0xFF;
        data[2] = 0xF7;
        data[40] = 0x02;
        data[41] = 0x02;

        for (property, value) in output_cache.iter() {
            data[property.byte()] = *value;
        }
        device.write(&data).ok();
    }

    /// Set the trigger effect for the left trigger
    pub fn set_left_trigger_effect(&mut self, trigger_mode: TriggerEffect) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectMode, trigger_mode.byte());
        *self.output_cache_changed.lock().unwrap() = true;
    }

    pub fn set_left_start_of_resistance(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectParameter1, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Depends on the trigger mode
    pub fn set_left_param2(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectParameter2, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode2: the force exerted
    pub fn set_left_param3(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectParameter3, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: the force exerted near release state
    pub fn set_left_param4(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectParameter4, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: the force exerted near middle
    pub fn set_left_param5(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectParameter5, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: the force exerted at pressed state
    pub fn set_left_param6(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectParameter6, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: actuation hertz frequency
    pub fn set_left_param7(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::LeftEffectParameter7, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Set the trigger effect for the right trigger
    pub fn set_right_trigger_effect(&mut self, trigger_mode: TriggerEffect) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectMode, trigger_mode.byte());
        *self.output_cache_changed.lock().unwrap() = true;
    }

    pub fn set_right_start_of_resistance(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectParameter1, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Depends on the trigger mode
    pub fn set_right_param2(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectParameter2, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode2: the force exerted
    pub fn set_right_param3(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectParameter3, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: the force exerted near release state
    pub fn set_right_param4(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectParameter4, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: the force exerted near middle
    pub fn set_right_param5(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectParameter5, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: the force exerted at pressed state
    pub fn set_right_param6(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectParameter6, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Only for mode4|20: actuation hertz frequency
    pub fn set_right_param7(&mut self, value: u8) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::RightEffectParameter7, value);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Set the light corresponding to the player number this controller belongs to (white light
    /// under the touchpad). Allowed range: 0-5. 0 means no light. Led placement:
    /// - 0: xxxxxxx
    /// - 1: xxxOxxx
    /// - 2: xxOxOxx
    /// - 3: xOxOxOx
    /// - 4: OxOxOxO
    /// - 5: xOxxxOx
    pub fn set_player_number(&mut self, value: u8) {
        if !(0..=5).contains(&value) {
            return;
        }
        let to_send = match value {
            0 => 0x00,
            1 => 0x04,
            2 => 0x0A,
            3 => 0x15,
            4 => 0x1B,
            5 => 0x10,
            _ => 0x04,
        };
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::PlayerLight, to_send);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    pub fn set_mute(&mut self, value: bool) {
        self.output_cache
            .lock()
            .unwrap()
            .insert(OutputProperty::Mute, value as u8);
        *self.output_cache_changed.lock().unwrap() = true;
    }

    /// Returns an id to unassign this combo in the future
    pub fn register_combo(
        &mut self,
        keys: Vec<Box<dyn Fn(&ComboProperty) -> bool + Send + Sync>>,
        duration: Duration,
        cb: Box<dyn Fn() + Send + Sync>,
    ) -> ComboId {
        let id = self.combos.lock().unwrap().len();
        self.combos
            .lock()
            .unwrap()
            .push(Combo::new(keys, cb, id, duration));
        ComboId::new(id)
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

    fn extract_bytes_v2(prop: &ComboProperty, data: &[u8; 64]) -> ValueType {
        if prop.offset().is_whole_byte() {
            prop.convert(&data.as_slice()[prop.offset().bytes])
        } else if prop.offset().is_single_byte() {
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
            unreachable!()
        }
    }

    fn extract_bytes(prop: &InputProperty, data: &[u8; 64]) -> ValueType {
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
        let api = HidApi::new().unwrap();
        let device = api.open(VENDOR_ID, PRODUCT_ID).unwrap();
        Self::new_with_device(device)
    }
}
