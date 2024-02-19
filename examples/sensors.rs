use dualsense_rs::DualSense;

fn main() {
    let mut controller = DualSense::default();

    controller.on_gyro_x_changed(&|val| println!("gyro x: {val}"));
    controller.on_gyro_y_changed(&|val| println!("gyro y: {val}"));
    controller.on_gyro_z_changed(&|val| println!("gyro z: {val}"));

    controller.on_accel_x_changed(&|val| println!("accel x: {val}"));
    controller.on_accel_y_changed(&|val| println!("accel y: {val}"));
    controller.on_accel_z_changed(&|val| println!("accel z: {val}"));

    let handle = controller.run();
    handle.join().ok();
}
