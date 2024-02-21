use std::time::Duration;

use dualsense_rs::{
    properties::{property::ComboProperty, symbols::Symbols},
    DualSense,
};

fn cross_pressed(key: &ComboProperty) -> bool {
    match key {
        ComboProperty::Symbol(v) => matches!(v, Symbols::Cross),
        _ => false,
    }
}

fn lb_pressed(key: &ComboProperty) -> bool {
    matches!(key, ComboProperty::LB)
}

fn on_combo_done() {
    println!("Combo done");
}

fn main() {
    let mut controller = DualSense::default();
    let id = controller.register_combo(
        vec![Box::new(cross_pressed), Box::new(lb_pressed)],
        Duration::from_millis(500),
        Box::new(on_combo_done),
    );

    controller.on_symbols_changed(Box::new(move |sym|{}));
    let handle = controller.run();

    handle.join().ok();
}
