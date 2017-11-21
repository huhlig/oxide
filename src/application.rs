//! Core Application Engine
//!

#![warn(dead_code)]

use state::State;
use std;

/// Runtime
pub struct Application {
    application_version: Version,
    application_name: String,
    state_stack: Vec<Box<State>>,
    events_loop: winit::EventsLoop,
}

impl Application {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::default()
    }
    pub fn run(&mut self) {
        self.initialize();
        let frame_start_time = std::time::Instant::now();

        loop {
            let update_start_time = std::time::Instant::now();
            let elapsed_frame_time = frame_start_time.elapsed();
        }
        self.shutdown();
    }
    fn initialize(&mut self) {}
    fn shutdown(&mut self) {}
}

/// Builder for Runtime
pub struct ApplicationBuilder {
    application_major: u8,
    application_minor: u8,
    application_patch: u8,
    application_name: String,
    target_frametime: u8,
    initial_state: Option<Box<State>>,
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        ApplicationBuilder {
            application_major: 0,
            application_minor: 0,
            application_patch: 0,
            application_name: String::new(),
            target_frametime: 1000 / 60,
            initial_state: Option::None,
        }
    }
}

impl ApplicationBuilder {
    pub fn build(self) -> Application {
        Application {
            application_version: Version {
                major: self.application_major,
                minor: self.application_minor,
                patch: self.application_patch,
            },
            application_name: self.application_name,
            events_loop: winit::EventsLoop::new(),
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
        self.application_frametime = target;
        self
    }
    pub fn set_target_fps(&mut self, target: u32) -> &mut Self {
        self.application_frametime = 1000.0 / target;
        self
    }
    pub fn create_window(&mut self, width: usize, height: usize, title: &[str]) -> &mut self {

    }
}