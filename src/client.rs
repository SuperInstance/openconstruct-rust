use crate::error::{OpenConstructError, Result};
use crate::registry::{ModuleRegistry, PolicyEngine};
use crate::types::*;
use crate::{FleetManager, SenseManager};
use std::collections::HashMap;

/// The main OpenConstruct client.
#[derive(Debug)]
pub struct OpenConstructClient {
    pub agent_name: String,
    pub model: String,
    pub capabilities: Vec<String>,
    session: Option<Session>,
    selected_modules: Vec<ModuleDescriptor>,
    interface_choice: Option<InterfaceChoice>,
    registry: ModuleRegistry,
    policy: PolicyEngine,
    fleet: FleetManager,
    sense: SenseManager,
    extra_config: HashMap<String, serde_json::Value>,
}

impl OpenConstructClient {
    /// Create a new builder.
    pub fn builder() -> crate::builder::OpenConstructClientBuilder {
        crate::builder::OpenConstructClientBuilder::new()
    }

    pub(crate) fn new(agent_name: String, model: String, capabilities: Vec<String>) -> Self {
        Self {
            agent_name,
            model,
            capabilities,
            session: None,
            selected_modules: Vec::new(),
            interface_choice: None,
            registry: ModuleRegistry::load_defaults(),
            policy: PolicyEngine::load_defaults(),
            fleet: FleetManager::new(),
            sense: SenseManager::new(),
            extra_config: HashMap::new(),
        }
    }

    /// Start the onboarding session.
    pub fn start(&mut self) -> Result<()> {
        if self.session.is_some() {
            return Err(OpenConstructError::AlreadyComplete);
        }
        self.session = Some(Session {
            id: uuid::Uuid::new_v4().to_string(),
            agent_name: self.agent_name.clone(),
            started_at: chrono::Utc::now(),
            phase: SessionPhase::Init,
        });
        Ok(())
    }

    /// Get the session ID (if started).
    pub fn session_id(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.id.as_str())
    }

    /// Get the current session phase.
    pub fn phase(&self) -> Option<&SessionPhase> {
        self.session.as_ref().map(|s| &s.phase)
    }

    /// Select modules by name.
    pub fn select_modules(&mut self, names: &[&str]) -> Result<()> {
        let session = self
            .session
            .as_mut()
            .ok_or(OpenConstructError::SessionNotStarted)?;
        let mut selected = Vec::new();
        for name in names {
            let module = self.registry.find(name).cloned().ok_or_else(|| {
                OpenConstructError::ModuleNotFound {
                    name: name.to_string(),
                }
            })?;
            selected.push(module);
        }
        self.selected_modules = selected;
        session.phase = SessionPhase::Modules;
        Ok(())
    }

    /// Choose the interface.
    pub fn choose_interface(&mut self, choice: InterfaceChoice) -> Result<()> {
        let session = self
            .session
            .as_mut()
            .ok_or(OpenConstructError::SessionNotStarted)?;
        self.interface_choice = Some(choice);
        session.phase = SessionPhase::Interface;
        Ok(())
    }

    /// Generate the final onboarding configuration.
    pub fn generate_config(&self) -> Result<OnboardingConfig> {
        let session = self
            .session
            .as_ref()
            .ok_or(OpenConstructError::SessionNotStarted)?;
        if self.selected_modules.is_empty() {
            return Err(OpenConstructError::ModulesNotSelected);
        }
        let interface = self
            .interface_choice
            .as_ref()
            .ok_or(OpenConstructError::InterfaceNotSet)?;

        let agent_card = AgentCard {
            name: self.agent_name.clone(),
            model: self.model.clone(),
            capabilities: self.capabilities.clone(),
            modules: self
                .selected_modules
                .iter()
                .map(|m| m.name.clone())
                .collect(),
            interface: interface.to_string(),
            session_id: session.id.clone(),
            created_at: chrono::Utc::now(),
        };

        let config_json = serde_json::to_string_pretty(&agent_card)?;

        Ok(OnboardingConfig {
            agent_card,
            selected_modules: self.selected_modules.clone(),
            interface_choice: interface.clone(),
            config_json,
        })
    }

    /// Discover fleet nodes.
    pub fn discover_fleet(&self) -> Result<FleetDiscovery> {
        self.session
            .as_ref()
            .ok_or(OpenConstructError::SessionNotStarted)?;
        Ok(self.fleet.discover())
    }

    /// Check a policy action.
    pub fn policy_check(&self, action: &str, resource: &str) -> Result<PolicyDecision> {
        self.session
            .as_ref()
            .ok_or(OpenConstructError::SessionNotStarted)?;
        Ok(self.policy.evaluate(action, resource))
    }

    /// Access the module registry.
    pub fn registry(&self) -> &ModuleRegistry {
        &self.registry
    }

    /// Access the sense manager.
    pub fn sense_mut(&mut self) -> &mut SenseManager {
        &mut self.sense
    }

    /// Set extra config.
    pub fn set_config(&mut self, key: &str, value: serde_json::Value) {
        self.extra_config.insert(key.to_string(), value);
    }
}
