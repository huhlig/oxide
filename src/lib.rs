//! Oxide Roguelike Framework
//!
extern crate winit;

pub mod runtime;
pub mod context;
pub mod event;
pub mod state;

#[derive(Debug)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{})", self.major, self.minor, self.patch)
    }
}

/// Oxide Version Constant
pub const VERSION: Version = Version {
    major: env!("CARGO_PKG_VERSION_MAJOR"),
    minor: env!("CARGO_PKG_VERSION_MINOR"),
    patch: env!("CARGO_PKG_VERSION_PATCH"),
};