//! This module contains actors.
//! To build a solid app, avoid communicating by sharing memory.
//! Focus on message passing instead.
mod services;
use std::sync::Arc;
use messages::prelude::Context;
use tokio::spawn;

use crate::configuration::Configuration;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

/// Creates and spawns the actors in the async system.
pub async fn create_actors() 
{
    // Though simple async tasks work, using the actor model
    // is highly recommended for state management
    // to achieve modularity and scalability in your app.
    // Actors keep ownership of their state and run in their own loops,
    // handling messages from other actors or external sources,
    // such as websockets or timers.
    // Create actor contexts.
    //TODO REMOVE UNWRAP!
    let cfg = Arc::new(Configuration::new().unwrap());
    let service_context = Context::new();
    let service_addr = service_context.address();
    let service_actor = services::ServicesActor::new(service_addr, cfg.clone());
    spawn(service_context.run(service_actor));
}
