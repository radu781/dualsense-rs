/// Cycle through all trigger effects
use std::{thread::sleep, time::Duration};

use dualsense_rs::{properties::trigger_effect::TriggerEffect, DualSense};

fn main() {
    let mut controller = DualSense::default();

    controller.on_left_force_changed(&|val| println!("left force is {val}"));
    controller.on_right_force_changed(&|val| println!("right force is {val}"));
    controller.on_left_force_enabled(&|val| println!("left force on/off {val}"));
    controller.on_right_force_enabled(&|val| println!("right force on/off {val}"));

    let _handle = controller.run();
    controller.set_left_start_of_resistance(1);
    controller.set_left_param2(100);
    controller.set_left_param3(50);
    controller.set_left_param4(150);
    controller.set_left_param5(200);
    controller.set_left_param6(200);
    controller.set_left_param7(200);

    let effects = [
        TriggerEffect::Mode1,
        TriggerEffect::Mode2,
        TriggerEffect::Mode1_4,
        TriggerEffect::Mode2_4,
        TriggerEffect::Mode1_20,
        TriggerEffect::Mode1_4_20,
        TriggerEffect::Mode2_4_20,
    ];
    let mut idx = 0;
    loop {
        let i = idx % effects.len();
        controller.set_left_trigger_effect(effects[i]);
        println!("Set trigger to {i}");
        sleep(Duration::from_millis(5000));
        idx += 1;
    }
}
