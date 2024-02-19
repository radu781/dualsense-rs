use std::{thread::sleep, time::Duration};

use dualsense_rs::DualSense;

fn main() {
    let mut controller = DualSense::default();

    let _handle = controller.run();
    controller.set_light_red(128);

    let mut idx = 0;
    loop {
        println!("Light for player {idx}");
        controller.set_player_number(idx);
        controller.set_mute(idx % 2 == 1);
        sleep(Duration::from_millis(5000));
        idx = (idx + 1) % 6;
    }
}
