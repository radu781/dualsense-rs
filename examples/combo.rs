use std::time::Duration;

use dualsense_rs::{
    properties::{
        analog_pad::DirectionQuadrant, combo_builder::SimultaneousCombo, property::ComboProperty,
        symbols::Symbols, traits::ComboAble,
    },
    DualSense,
};

fn cross_pressed(key: &ComboProperty) -> bool {
    match key {
        ComboProperty::Symbol(v) => matches!(v, Symbols::Cross),
        _ => false,
    }
}

fn lb_pressed(key: &ComboProperty) -> bool {
    matches!(key, ComboProperty::LB(true))
}

fn on_combo_done() {
    println!("Combo done");
}

fn main() {
    let mut controller = DualSense::default();

    let combo1 = SimultaneousCombo::default()
        .key(Box::new(cross_pressed))
        .key(Box::new(lb_pressed))
        .duration(Duration::from_millis(500))
        .done(Box::new(on_combo_done))
        .build();
    let combo2 = SimultaneousCombo::default()
        .key(Box::new(|k| match k {
            ComboProperty::LeftPad(lp) => lp.direction_quadrant() == DirectionQuadrant::North,
            _ => false,
        }))
        .key(Box::new(|k| match k {
            ComboProperty::RightPad(rp) => rp.direction_quadrant() == DirectionQuadrant::South,
            _ => false,
        }))
        .done(Box::new(|| println!("Left stick up and right stick down")))
        .build();

    let _ = controller.register_combo(combo1);
    let _ = controller.register_combo(combo2);

    let handle = controller.run();

    handle.join().ok();
}
