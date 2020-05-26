// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// utilities for tegne

#![feature(proc_macro_hygiene)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    box_pointers
)]

mod controller;
mod image;
mod window;

pub use controller::Controller;
pub use image::read_image;
pub use window::Events;
pub use window::Key;
pub use window::Window;
pub use window::WindowOptions;
