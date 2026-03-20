use std::net::IpAddr;

use crate::{
    database::models::{AccessListModel, AccessListRuleModel, ProxyHostModel, UpstreamModel},
    proxy::{
        accees_list::{AccessList, RuleAction},
        upstream::Upstream,
    },
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ProxyHost {
    pub domain: String,
    pub certificate_name: Option<String>,
    pub upstreams: Vec<Upstream>,
    pub access_list: Option<AccessList>,
}

impl ProxyHost {
    pub fn new(
        model: ProxyHostModel,
        upstream_models: Vec<UpstreamModel>,
        access_list: Option<(AccessListModel, Vec<AccessListRuleModel>)>,
    ) -> Result<Self> {
        let upstreams = upstream_models
            .into_iter()
            .map(Upstream::try_from)
            .collect::<Result<Vec<_>>>()?;

        let access_list = access_list
            .map(|(list, rules)| AccessList::new(list, rules))
            .transpose()?;

        Ok(Self {
            domain: model.domain,
            certificate_name: model.certificate_name,
            upstreams,
            access_list,
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

    pub fn update_access_list(
        &mut self,
        access_list_model: &AccessListModel,
        rules_models: &[AccessListRuleModel],
    ) -> Result<()> {
        if self.access_list.as_ref().map(|al| al.id) == Some(access_list_model.id) {
            self.access_list = Some(AccessList::new(
                access_list_model.clone(),
                rules_models.to_owned(),
            )?);
        }

        Ok(())
    }

    pub fn is_allowed(&self, ip: &IpAddr) -> bool {
        match &self.access_list {
            Some(list) => matches!(list.evaluate(ip), RuleAction::Allow),
            None => true,
        }
    }
}
