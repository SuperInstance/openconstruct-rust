use crate::types::*;

/// Fleet manager — discover nodes and find best candidates.
///
/// Holds the known set of fleet nodes and produces a timestamped snapshot
/// via [`discover`]. [`new`] seeds the built-in sample fleet so that the
/// manager is useful out of the box; use [`with_nodes`] / [`add_node`] for a
/// custom topology.
#[derive(Debug, Clone)]
pub struct FleetManager {
    nodes: Vec<FleetNode>,
}

impl Default for FleetManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetManager {
    /// Create a manager preloaded with the built-in sample fleet.
    pub fn new() -> Self {
        Self {
            nodes: default_nodes(),
        }
    }

    /// Create a manager with an explicit set of nodes (no sample data).
    pub fn with_nodes(nodes: Vec<FleetNode>) -> Self {
        Self { nodes }
    }

    /// Add a node to the manager.
    pub fn add_node(&mut self, node: FleetNode) {
        self.nodes.push(node);
    }

    /// The nodes currently known to this manager.
    pub fn nodes(&self) -> &[FleetNode] {
        &self.nodes
    }

    /// Produce a timestamped snapshot of the current fleet.
    pub fn discover(&self) -> FleetDiscovery {
        FleetDiscovery {
            nodes: self.nodes.clone(),
            timestamp: chrono::Utc::now(),
        }
    }
}

fn default_nodes() -> Vec<FleetNode> {
    vec![
        FleetNode {
            id: "node-alpha".into(),
            name: "Alpha Inference".into(),
            address: "10.0.1.10:8080".into(),
            capabilities: vec!["inference".into(), "embedding".into()],
            load: 0.35,
            latency_ms: 12,
            online: true,
        },
        FleetNode {
            id: "node-beta".into(),
            name: "Beta Vision".into(),
            address: "10.0.1.11:8080".into(),
            capabilities: vec!["vision".into(), "ocr".into()],
            load: 0.60,
            latency_ms: 25,
            online: true,
        },
        FleetNode {
            id: "node-gamma".into(),
            name: "Gamma Storage".into(),
            address: "10.0.1.12:8080".into(),
            capabilities: vec!["storage".into(), "indexing".into()],
            load: 0.15,
            latency_ms: 8,
            online: true,
        },
        FleetNode {
            id: "node-delta".into(),
            name: "Delta Offline".into(),
            address: "10.0.1.13:8080".into(),
            capabilities: vec!["inference".into(), "training".into()],
            load: 1.0,
            latency_ms: 500,
            online: false,
        },
    ]
}

/// Sense manager — typed sense shadows and fusion.
#[derive(Debug, Clone)]
pub struct SenseManager {
    shadows: Vec<SenseShadow>,
}

impl Default for SenseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SenseManager {
    pub fn new() -> Self {
        Self {
            shadows: Vec::new(),
        }
    }

    /// Create a typed sense shadow.
    pub fn create_shadow(
        &mut self,
        source: &str,
        kind: SenseKind,
        value: serde_json::Value,
    ) -> SenseShadow {
        let shadow = SenseShadow {
            source: source.into(),
            kind,
            value,
            timestamp: chrono::Utc::now(),
        };
        self.shadows.push(shadow.clone());
        shadow
    }

    /// Fuse correlating shadows into a single fused sense.
    pub fn fuse(&self, correlation_id: &str) -> Option<FusedSense> {
        if self.shadows.len() < 2 {
            return None;
        }
        let sources: Vec<String> = self.shadows.iter().map(|s| s.source.clone()).collect();
        let fused = FusedSense {
            sources,
            correlation_id: correlation_id.into(),
            fused_value: serde_json::json!({ "fused": true }),
            confidence: 0.92,
            timestamp: chrono::Utc::now(),
        };
        Some(fused)
    }

    pub fn shadows(&self) -> &[SenseShadow] {
        &self.shadows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_seeds_default_nodes() {
        let mgr = FleetManager::new();
        // discover() must snapshot the stored nodes, not a fresh hardcoded set.
        assert_eq!(mgr.nodes().len(), mgr.discover().nodes.len());
        assert!(mgr.nodes().iter().any(|n| n.id == "node-alpha"));
    }

    #[test]
    fn with_nodes_round_trip() {
        let mgr = FleetManager::with_nodes(vec![]);
        assert!(mgr.nodes().is_empty());
        assert!(mgr.discover().nodes.is_empty());
    }

    #[test]
    fn add_node_appears_in_discovery() {
        let mut mgr = FleetManager::with_nodes(vec![]);
        mgr.add_node(FleetNode {
            id: "solo".into(),
            name: "Solo".into(),
            address: "1.2.3.4:9".into(),
            capabilities: vec!["x".into()],
            load: 0.0,
            latency_ms: 1,
            online: true,
        });
        let d = mgr.discover();
        assert_eq!(d.nodes.len(), 1);
        assert_eq!(d.nodes[0].id, "solo");
    }

    #[test]
    fn discover_snapshots_state_with_fresh_timestamp() {
        // Two calls return equal node sets but the snapshot is a clone, so
        // mutating one does not affect the other.
        let mgr = FleetManager::new();
        let d1 = mgr.discover();
        let d2 = mgr.discover();
        assert_eq!(d1.nodes.len(), d2.nodes.len());
        // Timestamps are non-decreasing across calls.
        assert!(d2.timestamp >= d1.timestamp);
    }

    #[test]
    fn best_node_for_skips_offline_even_when_only_match() {
        // Delta is the only node advertising "training" but it is offline.
        // best_node_for must refuse to return an offline node.
        let mgr = FleetManager::new();
        let discovery = mgr.discover();
        let err = discovery.best_node_for("training").unwrap_err();
        assert!(matches!(
            err,
            crate::OpenConstructError::FleetNoMatch { .. }
        ));
    }

    #[test]
    fn best_node_for_picks_lower_score() {
        // Two online nodes both advertise "shared". The one with the lower
        // combined load + latency*0.01 score must win.
        let mgr = FleetManager::with_nodes(vec![
            FleetNode {
                id: "heavy".into(),
                name: "Heavy".into(),
                address: "1.1.1.1:1".into(),
                capabilities: vec!["shared".into()],
                load: 0.9,
                latency_ms: 200,
                online: true,
            },
            FleetNode {
                id: "light".into(),
                name: "Light".into(),
                address: "2.2.2.2:2".into(),
                capabilities: vec!["shared".into()],
                load: 0.1,
                latency_ms: 5,
                online: true,
            },
        ]);
        let discovery = mgr.discover();
        let best = discovery.best_node_for("shared").unwrap();
        assert_eq!(best.id, "light");
    }

    #[test]
    fn sense_fuse_needs_two_shadows() {
        let mut sm = SenseManager::new();
        assert!(sm.fuse("c").is_none());
        sm.create_shadow("a", SenseKind::Vision, serde_json::json!(1));
        assert_eq!(sm.shadows().len(), 1);
        assert!(sm.fuse("c").is_none());
        sm.create_shadow("b", SenseKind::Audio, serde_json::json!(2));
        let fused = sm.fuse("corr-1").expect("two shadows should fuse");
        assert_eq!(fused.correlation_id, "corr-1");
        assert_eq!(fused.sources, vec!["a".to_string(), "b".to_string()]);
        assert!(fused.confidence > 0.0);
    }

    #[test]
    fn sense_create_shadow_records_kind_and_value() {
        let mut sm = SenseManager::new();
        let shadow = sm.create_shadow(
            "cam",
            SenseKind::Custom("depth".into()),
            serde_json::json!({"d": 3}),
        );
        assert_eq!(shadow.source, "cam");
        assert_eq!(shadow.kind, SenseKind::Custom("depth".into()));
        assert_eq!(sm.shadows().len(), 1);
    }
}
