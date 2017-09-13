//! Oxide Roguelike Framework
//!
extern crate winit;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod app;
pub mod context;
pub mod event;
pub mod state;
