use crate::{
    database::models::{ProxyHostModel, UpstreamModel},
    proxy::upstream::Upstream,
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ProxyHost {
    pub domain: String,
    pub certificate_name: Option<String>,
    pub upstreams: Vec<Upstream>,
}

impl ProxyHost {
    pub fn new(model: ProxyHostModel, upstream_models: Vec<UpstreamModel>) -> Result<Self> {
        let upstreams = upstream_models
            .into_iter()
            .map(Upstream::try_from)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            domain: model.domain,
            certificate_name: model.certificate_name,
            upstreams,
        })
    }

    pub fn select_upstream(&self, index: usize) -> Option<Upstream> {
        self.upstreams.get(index % self.upstreams.len()).cloned()
    }

    pub fn update_upstream(&mut self, upstream_model: &UpstreamModel) -> Result<()> {
        if let Some(existing) = self
            .upstreams
            .iter_mut()
            .find(|u| u.name == upstream_model.name)
        {
            *existing = Upstream::try_from(upstream_model.clone())?;
        }

        Ok(())
    }
}
