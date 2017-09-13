//! Core Application Engine
//!

#![warn(dead_code)]

use state::State;

///
pub struct Application {
    state_stack: Vec<Box<State>>
}

///
pub struct ApplicationBuilder {
    application_name: String,
    application_major: u8,
    application_minor: u8,
    application_patch: u8,
    application_frametime: u8,
    initial_state: Option<Box<State>>,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }
    pub fn run(&mut self) {
        self.initialize();
        while true {}
        self.shutdown();
    }
    fn initialize(&mut self) {}
    fn shutdown(&mut self) {}
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        ApplicationBuilder {
            application_name: String::new(),
            application_major: 0,
            application_minor: 0,
            application_patch: 0,
            application_frametime: 1.0 / 60.0,
            initial_state: Option::None,
        }
    }
}

impl ApplicationBuilder {
    pub fn build(self) -> Application {
        Application {
            state_stack: vec!(self.initial_state.expect("Must have an initial state")),
        }
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.application_name = name;
        self
    }
    pub fn set_version(&mut self, major: u8, minor: u8, patch: u8) -> &mut Self {
        self.application_major = major;
        self.application_minor = minor;
        self.application_patch = patch;
        self
    }
    pub fn set_target_frametime(&mut self, target: f32) -> &mut Self {
        self.application_frametime =  target;
        self
    }
    pub fn set_target_fps(&mut self, target: u32) -> &mut Self {
        self.application_frametime = 1.0 / target;
        self
    }
}