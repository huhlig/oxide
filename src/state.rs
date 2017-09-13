//! Game Engine State

use event::Event;
use context::Context;
use std::time::Duration;

/// Engine State
pub trait State {
    /// Executed when the game state begins.
    fn on_start(&mut self, _ctx: &mut Context) {}

    /// Executed when the game state exits.
    fn on_stop(&mut self, _ctx: &mut Context) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_suspend(&mut self, _ctx: &mut Context) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, _ctx: &mut Context) {}

    /// Executed when an event arrives.
    fn on_event(&mut self, _ctx: &mut Context, _event: Event) -> Transition { Transition::None }

    /// Executed periodically by the application.
    fn on_update(&mut self, _ctx: &mut Context, _delta: Duration) -> Transition { Transition::None }
}

/// State transition Types.
pub enum Transition {
    /// No change.
    None,
    /// Pop the active state off the stack and resume the previous state on the stack or stop
    /// if there are none.
    Pop,
    /// Pause the active state and push a new state onto the stack.
    Push(Box<State>),
    /// Remove the current state on the stack and insert a different one.
    Switch(Box<State>),
    /// Pop all states and shut down.
    Shutdown,
}