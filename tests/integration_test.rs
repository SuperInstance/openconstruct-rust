use openconstruct::*;

#[test]
fn builder_creates_client() {
    let client = OpenConstructClient::builder()
        .agent_name("test-agent")
        .model("glm-5.1")
        .capabilities(["code_generation", "web_search"])
        .build();
    assert_eq!(client.agent_name, "test-agent");
    assert_eq!(client.model, "glm-5.1");
    assert_eq!(client.capabilities, vec!["code_generation", "web_search"]);
}

#[test]
fn builder_defaults() {
    let client = OpenConstructClient::builder().build();
    assert_eq!(client.agent_name, "default-agent");
    assert_eq!(client.model, "glm-5.1");
    assert!(client.capabilities.is_empty());
}

#[test]
fn start_creates_session() {
    let mut client = OpenConstructClient::builder().agent_name("s1").build();
    assert!(client.session_id().is_none());
    client.start().unwrap();
    assert!(client.session_id().is_some());
    let sid = client.session_id().unwrap();
    assert!(!sid.is_empty());
}

#[test]
fn unique_session_ids() {
    let mut c1 = OpenConstructClient::builder().agent_name("a").build();
    let mut c2 = OpenConstructClient::builder().agent_name("b").build();
    c1.start().unwrap();
    c2.start().unwrap();
    assert_ne!(c1.session_id(), c2.session_id());
}

#[test]
fn start_twice_errors() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    assert!(client.start().is_err());
}

#[test]
fn select_modules() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    client
        .select_modules(&["spectral-graph-core", "plato-room"])
        .unwrap();
    assert_eq!(client.phase().unwrap(), &SessionPhase::Modules);
}

#[test]
fn select_missing_module_errors() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    assert!(client.select_modules(&["nonexistent-module"]).is_err());
}

#[test]
fn select_modules_without_session_errors() {
    let mut client = OpenConstructClient::builder().build();
    assert!(client.select_modules(&["spectral-graph-core"]).is_err());
}

#[test]
fn choose_interface() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    client.choose_interface(InterfaceChoice::Cli).unwrap();
    assert_eq!(client.phase().unwrap(), &SessionPhase::Interface);
}

#[test]
fn generate_config() {
    let mut client = OpenConstructClient::builder()
        .agent_name("my-agent")
        .model("glm-5.1")
        .capabilities(["code_generation", "web_search"])
        .build();
    client.start().unwrap();
    client
        .select_modules(&["spectral-graph-core", "plato-room"])
        .unwrap();
    client.choose_interface(InterfaceChoice::Cli).unwrap();
    let config = client.generate_config().unwrap();

    assert_eq!(config.agent_card.name, "my-agent");
    assert_eq!(config.agent_card.model, "glm-5.1");
    assert_eq!(config.agent_card.modules.len(), 2);
    assert_eq!(config.interface_choice, InterfaceChoice::Cli);
    assert!(config.config_json.contains("my-agent"));
}

#[test]
fn generate_config_without_modules_errors() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    client.choose_interface(InterfaceChoice::Web).unwrap();
    assert!(client.generate_config().is_err());
}

#[test]
fn generate_config_without_interface_errors() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    client.select_modules(&["spectral-graph-core"]).unwrap();
    assert!(client.generate_config().is_err());
}

#[test]
fn full_lifecycle() {
    let mut client = OpenConstructClient::builder()
        .agent_name("lifecycle-agent")
        .model("glm-5.1")
        .capabilities(["reasoning", "vision"])
        .build();

    client.start().unwrap();
    let sid = client.session_id().unwrap().to_string();

    client
        .select_modules(&["spectral-graph-core", "echo-memory", "prism-vision"])
        .unwrap();
    client.choose_interface(InterfaceChoice::Discord).unwrap();

    let config = client.generate_config().unwrap();
    assert_eq!(config.agent_card.name, "lifecycle-agent");
    assert_eq!(config.agent_card.modules.len(), 3);
    assert_eq!(config.agent_card.interface, "discord");
    assert_eq!(config.agent_card.session_id, sid);
    assert!(!config.config_json.is_empty());

    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&config.config_json).unwrap();
    assert_eq!(parsed["name"], "lifecycle-agent");
}

#[test]
fn fleet_discovery() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    let fleet = client.discover_fleet().unwrap();
    assert!(!fleet.nodes.is_empty());
    assert!(fleet.nodes.iter().any(|n| n.name == "Alpha Inference"));
}

#[test]
fn fleet_best_node_for_inference() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    let fleet = client.discover_fleet().unwrap();
    let best = fleet.best_node_for("inference").unwrap();
    assert!(best.capabilities.contains(&"inference".to_string()));
    assert!(best.online);
    // Should prefer Alpha over Delta (Delta is offline)
    assert_eq!(best.id, "node-alpha");
}

#[test]
fn fleet_best_node_for_missing_capability() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    let fleet = client.discover_fleet().unwrap();
    assert!(fleet.best_node_for("nonexistent").is_err());
}

#[test]
fn policy_check_allow() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    let decision = client
        .policy_check("vision.capture", "/dev/video0")
        .unwrap();
    assert_eq!(decision, PolicyDecision::Allow);
}

#[test]
fn policy_check_deny() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    let decision = client.policy_check("file.write", "/etc/passwd").unwrap();
    assert_eq!(decision, PolicyDecision::Deny);
}

#[test]
fn policy_check_ask() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    let decision = client
        .policy_check("network.connect", "tcp://example.com")
        .unwrap();
    assert_eq!(decision, PolicyDecision::Ask);
}

#[test]
fn policy_check_without_session_errors() {
    let client = OpenConstructClient::builder().build();
    assert!(client.policy_check("file.read", "/tmp/test").is_err());
}

#[test]
fn module_filtering_by_domain() {
    let registry = openconstruct::registry::ModuleRegistry::load_defaults();
    let perception = registry.filter_by_domain("perception");
    assert!(!perception.is_empty());
    assert!(perception.iter().any(|m| m.name == "prism-vision"));
}

#[test]
fn module_filtering_by_tag() {
    let registry = openconstruct::registry::ModuleRegistry::load_defaults();
    let graph_modules = registry.filter_by_tag("graph");
    assert!(!graph_modules.is_empty());
    assert!(graph_modules
        .iter()
        .any(|m| m.name == "spectral-graph-core"));
}
