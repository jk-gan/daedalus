// use super::context::Context;
// use std::{any::Any, time::Duration};

// pub trait EngineSubsystem {
//     fn as_any(&self) -> &dyn Any;

//     // Runs when the subsystems need to initialize.
//     fn on_init(&mut self, context: &Context) {}

//     // Runs after the subsystems have initialized. Useful, if a particular subsystem needs to use another, initialized subsystem.
//     fn on_post_init(&self) {}

//     // Runs once evry frame and before OnTick().
//     fn on_pre_tick(&self) {}

//     // Runs every frame.
//     fn on_tick(&self, delta_time: Duration) {}

//     // Runs every frame and after OnTick().
//     fn on_post_tick(&self) {}

//     // Runs when the subsystems need to shutdown.
//     fn on_shutdown(&self) {}
// }
