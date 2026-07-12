use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenConstructError {
    #[error("session already started — call reset() to begin again")]
    SessionAlreadyStarted,

    #[error("session not started — call start() first")]
    SessionNotStarted,

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

impl OpenConstructError {
    /// Returns the legacy name for `SessionAlreadyStarted`.
    ///
    /// Historically this condition was reported as `AlreadyComplete`, which is
    /// misleading: the session is merely already started, not finished. New
    /// code should match on [`OpenConstructError::SessionAlreadyStarted`].
    #[deprecated(note = "use SessionAlreadyStarted")]
    pub const ALREADY_COMPLETE_NAME: &'static str = "SessionAlreadyStarted";
}

pub type Result<T> = std::result::Result<T, OpenConstructError>;
