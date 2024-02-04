use std::{thread, time::Duration};

use dualsense_rs::DualSense;

fn main() {
    let mut controller = DualSense::new();

    // controller.on_left_pad_x_changed(&|lpx| println!("left pad x: {lpx}"));
    // controller.on_left_pad_x_changed(&|lpx| {
    //     if lpx > 127 {
    //         println!("left pad x in right region: {lpx}")
    //     }
    // });
    // controller.on_left_pad_y_changed(&|lpy| println!("left pad y: {lpy}"));
    // controller.on_right_pad_x_changed(&|rpx| println!("right pad x: {rpx}"));
    // controller.on_right_pad_y_changed(&|rpy| println!("right pad y: {rpy}"));

    // controller.on_l2_changed(&|l2| println!("right pad x: {l2}"));
    // controller.on_r2_changed(&|r2| println!("right pad y: {r2}"));

    // controller.on_symbols_changed(&|sym| println!("pressed symbol {}", sym as u8));
    // controller.on_l1_changed(&|pressed| println!("l1 {pressed}"));
    // controller.on_r1_changed(&|pressed| println!("r1 {pressed}"));
    // controller.on_l3_changed(&|pressed| println!("l3 {pressed}"));
    // controller.on_r3_changed(&|pressed| println!("r3 {pressed}"));
    // controller.on_options_changed(&|pressed| println!("options {pressed}"));
    // controller.on_share_changed(&|pressed| println!("share {pressed}"));
    //
    // controller.on_touchpad_changed(&|pressed| println!("touchpad {pressed}"));
    // controller.on_mute_changed(&|pressed| println!("mute {pressed}"));
    // controller.on_playstation_pressed(&|pressed| println!("ps {pressed}"));

    // controller.on_gyro_x_changed(&|val| println!("gyro x: {val}"));
    // controller.on_gyro_y_changed(&|val| println!("gyro y: {val}"));
    // controller.on_gyro_z_changed(&|val| println!("gyro z: {val}"));

    // controller.on_accel_x_changed(&|val| println!("accel x: {val}"));
    // controller.on_accel_y_changed(&|val| println!("accel y: {val}"));
    // controller.on_accel_z_changed(&|val| println!("accel z: {val}"));

    // controller.on_touchpad1_x_changed(&|val| println!("touchpad 1 x: {val}"));
    // controller.on_touchpad1_y_changed(&|val| println!("touchpad 1 y: {val}"));
    // controller.on_touchpoint2_x_changed(&|val| println!("touchpad 2 x: {val}"));
    // controller.on_touchpoint2_y_changed(&|val| println!("touchpad 2 y: {val}"));
    // controller.on_touchpoint1_id_changed(&|id| println!("touchpad 1 id: {id}"));
    // controller.on_touchpoint2_id_changed(&|id| println!("touchpad 2 id: {id}"));

    // controller.on_touchpad1_pressed(&|val| println!("touchpad 1 pressed: {val}"));
    // controller.on_touchpoint2_changed(&|val| println!("touchpad 2 pressed: {val}"));

    // controller.on_left_force_changed(&|val| println!("left force is {val}"));
    // controller.on_right_force_changed(&|val| println!("right force is {val}"));
    // controller.on_left_force_enabled(&|val| println!("left force on/off {val}"));
    // controller.on_right_force_enabled(&|val| println!("right force on/off {val}"));
    let mut val = 0;
    let handle = controller.run();
    // handle.join().ok();
    loop {
        controller.set_light_red(val);
        controller.set_light_green(255 - val);
        controller.set_light_blue(val);
        thread::sleep(Duration::from_millis(100));
        val = (val + 1) % 0xFF;
    }
}
