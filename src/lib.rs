pub mod agent_engine;
pub mod channels;
pub mod chat_commands;
pub mod clawhub;
pub mod codex_auth;
pub mod config;
pub mod doctor;
pub mod embedding;
pub mod gateway;
pub mod hooks;
pub mod llm;
pub mod mcp;
pub mod memory_backend;
pub mod otlp;
pub mod plugins;
pub(crate) mod run_control;
pub mod runtime;
pub mod scheduler;
pub mod setup;
pub mod setup_def;
pub mod skills;
pub mod tools;
pub mod web;

pub use channels::discord;
pub use channels::telegram;
pub use loongclaw_app::builtin_skills;
pub use loongclaw_app::logging;
pub use loongclaw_app::transcribe;
pub use loongclaw_channels::channel;
pub use loongclaw_channels::channel_adapter;
pub use loongclaw_core::error;
pub use loongclaw_core::llm_types;
pub use loongclaw_core::text;
pub use loongclaw_storage::db;
pub use loongclaw_storage::memory;
pub use loongclaw_storage::memory_quality;
pub use loongclaw_tools::sandbox;

#[cfg(test)]
pub mod test_support {
    use std::sync::{Mutex, MutexGuard, OnceLock};

    pub fn env_lock() -> MutexGuard<'static, ()> {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock poisoned")
    }
}
