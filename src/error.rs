use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenConstructError {
    #[error("session not started — call start() first")]
    SessionNotStarted,

    #[error("onboarding already complete")]
    AlreadyComplete,

    #[error("module not found: {name}")]
    ModuleNotFound { name: String },

    #[error("no fleet node found with capability: {capability}")]
    FleetNoMatch { capability: String },

    #[error("policy denied action: {action} on {resource}")]
    PolicyDenied { action: String, resource: String },

    #[error("interface choice not set")]
    InterfaceNotSet,

    #[error("modules not selected")]
    ModulesNotSelected,

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, OpenConstructError>;
