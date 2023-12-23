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
    controller.on_l1_changed(&|pressed| println!("l1 {pressed}"));
    controller.on_r1_changed(&|pressed| println!("r1 {pressed}"));
    controller.on_l3_changed(&|pressed| println!("l3 {pressed}"));
    controller.on_r3_changed(&|pressed| println!("r3 {pressed}"));
    controller.on_options_changed(&|pressed| println!("options {pressed}"));
    controller.on_share_changed(&|pressed| println!("share {pressed}"));

    let handle = controller.run();
    handle.join().ok();
}
