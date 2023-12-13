//! Rust programmatic wrapper over HID messages sent and received by the PS5 DualSense controller. All communication is done via callback functions that are ran in a separate thread.
//! 
//! ## Usage
//! 
//! ### Read
//! 
//! Log details about the left and right sticks' positions
//! 
//! ```rs
//! use dualsense_rs::DualSense;
//! 
//! 
//! fn main() {
//!     let mut controller = DualSense::new();
//! 
//!     controller.on_left_pad_x_changed(&|lpx| println!("left pad x: {lpx}"));
//!     controller.on_left_pad_x_changed(&|lpx| {
//!         if lpx > 127 {
//!             println!("left pad x in right region: {lpx}")
//!         }
//!     });
//!    controller.on_left_pad_y_changed(&|lpy| println!("left pad y: {lpy}"));
//!    controller.on_right_pad_x_changed(&|rpx| println!("right pad x: {rpx}"));
//!    controller.on_right_pad_y_changed(&|rpy| println!("right pad y: {rpy}"));
//! 
//!     let handle = controller.run();
//!     handle.join().ok();
//! }
//! ```
//! 
//! Run the complete example with:
//! 
//! ```sh
//! cargo run --example log_all
//! ```

mod dualsense;

pub use dualsense::*;
