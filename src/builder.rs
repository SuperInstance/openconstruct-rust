use crate::OpenConstructClient;

/// Builder for `OpenConstructClient`.
#[derive(Debug, Default)]
pub struct OpenConstructClientBuilder {
    agent_name: Option<String>,
    model: Option<String>,
    capabilities: Vec<String>,
}

impl OpenConstructClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn agent_name(mut self, name: &str) -> Self {
        self.agent_name = Some(name.to_string());
        self
    }

    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn capabilities<I, S>(mut self, caps: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.capabilities = caps.into_iter().map(|s| s.as_ref().to_string()).collect();
        self
    }

    pub fn build(self) -> OpenConstructClient {
        OpenConstructClient::new(
            self.agent_name.unwrap_or_else(|| "default-agent".into()),
            self.model.unwrap_or_else(|| "glm-5.1".into()),
            self.capabilities,
        )
    }
}
