//! Rust programmatic wrapper over HID messages sent and received by the PS5 DualSense controller.
//!
//! ## Usage
//!
//! ### Read
//!
//! Rust programmatic wrapper over HID messages sent and received by the PS5 DualSense controller.
//! Allows support for simultaneous key presses or combos that require consecutive presses in a given
//! time interval(wip).
//!
//! Log details about the left and right sticks' positions:
//!
//! ```rust
//! let mut controller = DualSense::default();
//!
//! controller.on_left_pad_x_changed(&|lpx| println!("left pad x: {lpx}"));
//! controller.on_left_pad_x_changed(&|lpx| {
//!     if lpx > 127 {
//!         println!("left pad x in right region: {lpx}")
//!     }
//! });
//! controller.on_left_pad_y_changed(&|lpy| println!("left pad y: {lpy}"));
//! controller.on_right_pad_x_changed(&|rpx| println!("right pad x: {rpx}"));
//!
//! let handle = controller.run();
//! // can also add after the `.run()` call
//! controller.on_right_pad_y_changed(&|rpy| println!("right pad y: {rpy}"));
//! // make sure to join the thread so the program doesn't stop immediately
//! handle.join().ok();
//! ```
//!
//! ### Write
//!
//! Output values are stored in an internal cache and will be sent in the following read/write cycle. Packets will
//! not be sent if nothing changed.
//!
//! ```rust
//! let mut controller = DualSense::default();
//! controller.set_left_trigger_effect(TriggerEffect::Mode1);
//!
//! let handle = controller.run();
//! controller.set_light_red(255);
//! handle.join().ok();
//! ```
//!
//! ## Examples
//!
//! Try examples for:
//!
//! - log_all -> log values for all buttons, analog sticks and touchpad
//! - sensors -> log values for the gyroscope and accelerometer
//! - triggers -> log values for the triggers and change their modes
//!
//! ```sh
//! cargo run --example <example>
//! ```

mod dualsense;

pub use dualsense::*;
