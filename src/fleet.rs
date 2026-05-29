use crate::types::*;

/// Fleet manager — discover nodes and find best candidates.
#[derive(Debug, Clone)]
pub struct FleetManager {
    nodes: Vec<FleetNode>,
}

impl FleetManager {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Simulate fleet discovery with sample nodes.
    pub fn discover(&self) -> FleetDiscovery {
        let nodes = vec![
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
        ];
        FleetDiscovery {
            nodes,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Sense manager — typed sense shadows and fusion.
#[derive(Debug, Clone)]
pub struct SenseManager {
    shadows: Vec<SenseShadow>,
}

impl SenseManager {
    pub fn new() -> Self {
        Self { shadows: Vec::new() }
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
