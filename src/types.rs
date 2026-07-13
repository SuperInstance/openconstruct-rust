use serde::{Deserialize, Serialize};

/// The interface choice for the agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceChoice {
    Cli,
    Web,
    Daemon,
    Discord,
    Telegram,
    Custom(String),
}

impl std::fmt::Display for InterfaceChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterfaceChoice::Cli => write!(f, "cli"),
            InterfaceChoice::Web => write!(f, "web"),
            InterfaceChoice::Daemon => write!(f, "daemon"),
            InterfaceChoice::Discord => write!(f, "discord"),
            InterfaceChoice::Telegram => write!(f, "telegram"),
            InterfaceChoice::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// A module descriptor from the registry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleDescriptor {
    pub name: String,
    pub version: String,
    pub domain: String,
    pub tags: Vec<String>,
    pub description: String,
    pub enabled: bool,
}

/// Policy decision returned by the policy engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecision {
    Allow,
    Deny,
    Ask,
}

/// A policy rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyRule {
    pub action_pattern: String,
    pub resource_pattern: String,
    pub decision: PolicyDecision,
    pub description: String,
}

/// A fleet node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetNode {
    pub id: String,
    pub name: String,
    pub address: String,
    pub capabilities: Vec<String>,
    pub load: f64,
    pub latency_ms: u64,
    pub online: bool,
}

/// The result of fleet discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetDiscovery {
    pub nodes: Vec<FleetNode>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl FleetDiscovery {
    pub fn best_node_for(&self, capability: &str) -> Result<&FleetNode, crate::OpenConstructError> {
        self.nodes
            .iter()
            .filter(|n| n.online && n.capabilities.iter().any(|c| c == capability))
            .min_by(|a, b| {
                (a.load + a.latency_ms as f64 * 0.01)
                    .partial_cmp(&(b.load + b.latency_ms as f64 * 0.01))
                    .unwrap()
            })
            .ok_or(crate::OpenConstructError::FleetNoMatch {
                capability: capability.to_string(),
            })
    }
}

/// A sense shadow — typed sensor reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseShadow {
    pub source: String,
    pub kind: SenseKind,
    pub value: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Types of sense inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SenseKind {
    Vision,
    Audio,
    Thermal,
    Lidar,
    Text,
    Metric,
    Custom(String),
}

/// Sense fusion correlator output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusedSense {
    pub sources: Vec<String>,
    pub correlation_id: String,
    pub fused_value: serde_json::Value,
    pub confidence: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Agent card produced by the onboarding flow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentCard {
    pub name: String,
    pub model: String,
    pub capabilities: Vec<String>,
    pub modules: Vec<String>,
    pub interface: String,
    pub session_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// The final onboarding configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingConfig {
    pub agent_card: AgentCard,
    pub selected_modules: Vec<ModuleDescriptor>,
    pub interface_choice: InterfaceChoice,
    pub config_json: String,
}

/// Session info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub agent_name: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub phase: SessionPhase,
}

/// Phases of the onboarding session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionPhase {
    Init,
    Modules,
    Interface,
    Policy,
    Complete,
}
