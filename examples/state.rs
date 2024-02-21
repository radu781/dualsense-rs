use dualsense_rs::{
    properties::symbols::Symbols,
    DualSense,
};

#[derive(Default, Debug)]
struct Counter {
    crosses: u32,
    triangles: u32,
    squares: u32,
    circles: u32,
}

fn main() {
    let mut controller = DualSense::default();

    let mut counter = Counter::default();
    controller.on_symbols_changed(Box::new(move |sym| {
        match sym {
            Symbols::Square => counter.squares += 1,
            Symbols::Cross => counter.crosses += 1,
            Symbols::Circle => counter.circles += 1,
            Symbols::Triangle => counter.triangles += 1,
            Symbols::None => {}
        };
        if sym != Symbols::None {
            println!("{:?}", counter);
        }
    }));
    
    let handle = controller.run();
    handle.join().ok();
}
