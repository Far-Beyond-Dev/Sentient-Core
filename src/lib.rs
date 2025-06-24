use async_trait::async_trait;
use horizon_event_system::{
    create_simple_plugin, current_timestamp, EventSystem, LogLevel,
    PluginError, ServerContext, SimplePlugin,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info, warn};

/// SentientCore Plugin
pub struct SentientCorePlugin {
    name: String,
}

impl SentientCorePlugin {
    pub fn new() -> Self {
        info!("🔧 SentientCorePlugin: Creating new instance");
        Self {
            name: "Sentient-Core".to_string(),
        }
    }
}

#[async_trait]
impl SimplePlugin for SentientCorePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    async fn register_handlers(&mut self, _events: Arc<EventSystem>) -> Result<(), PluginError> {
        info!("🔧 SentientCorePlugin: Registering event handlers...");
        
        // TODO: Register your event handlers here
        // Example:
        // register_handlers!(events; core {
        //     "your_event" => |event: serde_json::Value| {
        //         info!("Received event: {:?}", event);
        //         Ok(())
        //     }
        // })?;
        
        info!("🔧 SentientCorePlugin: ✅ All handlers registered successfully!");
        Ok(())
    }

    async fn on_init(&mut self, context: Arc<dyn ServerContext>) -> Result<(), PluginError> {
        context.log(
            LogLevel::Info,
            "🔧 SentientCorePlugin: Starting up!",
        );

        // TODO: Add your initialization logic here
        
        info!("🔧 SentientCorePlugin: ✅ Initialization complete!");
        Ok(())
    }

    async fn on_shutdown(&mut self, context: Arc<dyn ServerContext>) -> Result<(), PluginError> {
        context.log(
            LogLevel::Info,
            "🔧 SentientCorePlugin: Shutting down!",
        );

        // TODO: Add your cleanup logic here

        info!("🔧 SentientCorePlugin: ✅ Shutdown complete!");
        Ok(())
    }
}

// Create the plugin using the macro
create_simple_plugin!(SentientCorePlugin);
