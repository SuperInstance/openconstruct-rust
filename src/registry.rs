use crate::types::{ModuleDescriptor, PolicyDecision, PolicyRule};

/// Module registry — load and filter modules.
#[derive(Debug, Clone)]
pub struct ModuleRegistry {
    modules: Vec<ModuleDescriptor>,
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    /// Load built-in default modules.
    pub fn load_defaults() -> Self {
        let modules = vec![
            ModuleDescriptor {
                name: "spectral-graph-core".into(),
                version: "1.0.0".into(),
                domain: "knowledge".into(),
                tags: vec!["graph".into(), "semantic".into()],
                description: "Core spectral graph engine for knowledge representation".into(),
                enabled: true,
            },
            ModuleDescriptor {
                name: "plato-room".into(),
                version: "2.1.0".into(),
                domain: "reasoning".into(),
                tags: vec!["dialogue".into(), "socratic".into()],
                description: "Socratic dialogue and reasoning engine".into(),
                enabled: true,
            },
            ModuleDescriptor {
                name: "sentinel-guard".into(),
                version: "0.9.3".into(),
                domain: "security".into(),
                tags: vec!["policy".into(), "guard".into()],
                description: "Policy enforcement and safety guard".into(),
                enabled: true,
            },
            ModuleDescriptor {
                name: "atlas-nav".into(),
                version: "1.2.0".into(),
                domain: "navigation".into(),
                tags: vec!["spatial".into(), "mapping".into()],
                description: "Spatial navigation and mapping".into(),
                enabled: true,
            },
            ModuleDescriptor {
                name: "echo-memory".into(),
                version: "3.0.1".into(),
                domain: "memory".into(),
                tags: vec!["recall".into(), "persistence".into()],
                description: "Persistent memory and recall system".into(),
                enabled: true,
            },
            ModuleDescriptor {
                name: "prism-vision".into(),
                version: "1.4.0".into(),
                domain: "perception".into(),
                tags: vec!["vision".into(), "camera".into()],
                description: "Computer vision and camera integration".into(),
                enabled: true,
            },
        ];
        Self { modules }
    }

    /// Find a module by name.
    pub fn find(&self, name: &str) -> Option<&ModuleDescriptor> {
        self.modules.iter().find(|m| m.name == name)
    }

    /// Filter modules by domain.
    pub fn filter_by_domain(&self, domain: &str) -> Vec<&ModuleDescriptor> {
        self.modules.iter().filter(|m| m.domain == domain).collect()
    }

    /// Filter modules by tag.
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&ModuleDescriptor> {
        self.modules
            .iter()
            .filter(|m| m.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// List all modules.
    pub fn list(&self) -> &[ModuleDescriptor] {
        &self.modules
    }

    pub fn add(&mut self, module: ModuleDescriptor) {
        self.modules.push(module);
    }
}

/// Policy engine — evaluate actions against loaded rules.
#[derive(Debug, Clone)]
pub struct PolicyEngine {
    rules: Vec<PolicyRule>,
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Load default security policy rules.
    pub fn load_defaults() -> Self {
        let rules = vec![
            PolicyRule {
                action_pattern: "vision.capture".into(),
                resource_pattern: "/dev/video*".into(),
                decision: PolicyDecision::Allow,
                description: "Allow camera capture".into(),
            },
            PolicyRule {
                action_pattern: "file.write".into(),
                resource_pattern: "/etc/*".into(),
                decision: PolicyDecision::Deny,
                description: "Deny writing to system config".into(),
            },
            PolicyRule {
                action_pattern: "network.connect".into(),
                resource_pattern: "*".into(),
                decision: PolicyDecision::Ask,
                description: "Ask before network connections".into(),
            },
            PolicyRule {
                action_pattern: "file.read".into(),
                resource_pattern: "*".into(),
                decision: PolicyDecision::Allow,
                description: "Allow reading files".into(),
            },
            PolicyRule {
                action_pattern: "system.shutdown".into(),
                resource_pattern: "*".into(),
                decision: PolicyDecision::Deny,
                description: "Deny system shutdown".into(),
            },
        ];
        Self { rules }
    }

    /// Evaluate an action against the policy rules.
    pub fn evaluate(&self, action: &str, resource: &str) -> PolicyDecision {
        for rule in &self.rules {
            if self.matches_pattern(&rule.action_pattern, action)
                && self.matches_pattern(&rule.resource_pattern, resource)
            {
                return rule.decision.clone();
            }
        }
        PolicyDecision::Ask // default: ask
    }

    fn matches_pattern(&self, pattern: &str, value: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        if let Some(prefix) = pattern.strip_suffix('*') {
            return value.starts_with(prefix);
        }
        pattern == value
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }

    pub fn rules(&self) -> &[PolicyRule] {
        &self.rules
    }
}
