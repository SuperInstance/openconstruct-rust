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
    let err = client.start().unwrap_err();
    // Renamed from the misleading `AlreadyComplete`; the session is started,
    // not finished.
    assert!(matches!(err, OpenConstructError::SessionAlreadyStarted));
}

#[test]
fn reset_clears_session_and_allows_restart() {
    let mut client = OpenConstructClient::builder().build();
    client.start().unwrap();
    let sid = client.session_id().unwrap().to_string();
    client.select_modules(&["spectral-graph-core"]).unwrap();
    client.choose_interface(InterfaceChoice::Cli).unwrap();

    let prev = client.reset();
    assert_eq!(prev.as_deref(), Some(sid.as_str()));
    assert!(client.session_id().is_none());
    assert!(client.phase().is_none());

    // After reset, a fresh session can be started without error.
    client.start().unwrap();
    assert_ne!(client.session_id(), Some(sid.as_str()));
    // Re-selecting modules without a fresh select now fails because state was
    // cleared — generate_config should refuse until modules are chosen again.
    assert!(client.generate_config().is_err());
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
fn module_filtering_by_tag() {
    let registry = openconstruct::registry::ModuleRegistry::load_defaults();
    let graph_modules = registry.filter_by_tag("graph");
    assert!(!graph_modules.is_empty());
    assert!(graph_modules
        .iter()
        .any(|m| m.name == "spectral-graph-core"));
}

#[test]
fn discover_fleet_without_session_errors() {
    let client = OpenConstructClient::builder().build();
    // Symmetric with policy_check_without_session_errors — discover_fleet also
    // requires a session and this branch was previously untested.
    assert!(client.discover_fleet().is_err());
}

#[test]
fn registry_find_missing_returns_none() {
    let registry = openconstruct::registry::ModuleRegistry::load_defaults();
    assert!(registry.find("does-not-exist").is_none());
    assert!(registry.find("spectral-graph-core").is_some());
}

#[test]
fn registry_add_and_new() {
    let mut registry = openconstruct::registry::ModuleRegistry::new();
    assert!(registry.list().is_empty());
    registry.add(ModuleDescriptor {
        name: "custom".into(),
        version: "0.1.0".into(),
        domain: "custom".into(),
        tags: vec!["t".into()],
        description: "custom module".into(),
        enabled: true,
    });
    assert_eq!(registry.list().len(), 1);
    assert!(registry.find("custom").is_some());
}

#[test]
fn custom_interface_choice_round_trips() {
    let choice = InterfaceChoice::Custom("slack".into());
    assert_eq!(choice.to_string(), "custom:slack");
    // A custom interface must survive a full onboarding flow and appear in the
    // generated agent card exactly as formatted by Display.
    let mut client = OpenConstructClient::builder().agent_name("a").build();
    client.start().unwrap();
    client.select_modules(&["spectral-graph-core"]).unwrap();
    client
        .choose_interface(InterfaceChoice::Custom("slack".into()))
        .unwrap();
    let config = client.generate_config().unwrap();
    assert_eq!(
        config.interface_choice,
        InterfaceChoice::Custom("slack".into())
    );
    assert_eq!(config.agent_card.interface, "custom:slack");
}

#[test]
fn config_json_is_valid_agent_card() {
    let mut client = OpenConstructClient::builder()
        .agent_name("json-agent")
        .model("glm-5.1")
        .capabilities(["code_generation"])
        .build();
    client.start().unwrap();
    client
        .select_modules(&["spectral-graph-core", "plato-room"])
        .unwrap();
    client.choose_interface(InterfaceChoice::Daemon).unwrap();
    let config = client.generate_config().unwrap();

    // config_json must be exactly the serialized AgentCard.
    let parsed: AgentCard =
        serde_json::from_str(&config.config_json).expect("config_json must deserialize");
    assert_eq!(parsed, config.agent_card);
}

#[test]
fn set_config_and_sense_mut() {
    let mut client = OpenConstructClient::builder().build();
    client.set_config("region", serde_json::json!("us-east"));
    // sense_mut() must be usable to add shadows through the client.
    client.start().unwrap();
    let shadow =
        client
            .sense_mut()
            .create_shadow("therm", SenseKind::Thermal, serde_json::json!(42.0));
    assert_eq!(shadow.source, "therm");
    assert_eq!(client.sense_mut().shadows().len(), 1);
}

#[test]
fn capabilities_builder_replaces_not_appends() {
    // Document the actual semantics: capabilities() replaces the full set.
    let c1 = OpenConstructClient::builder()
        .capabilities(["a"])
        .capabilities(["b", "c"])
        .build();
    assert_eq!(c1.capabilities, vec!["b", "c"]);
}
