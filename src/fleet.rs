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
