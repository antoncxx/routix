use crate::{database::models::UpstreamModel, proxy::ProxyHost};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct HostsManager {
    hosts: RwLock<HashMap<String, Arc<ProxyHost>>>,
}

impl HostsManager {
    pub fn new() -> Self {
        Self {
            hosts: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, domain: &str) -> Option<Arc<ProxyHost>> {
        self.hosts.read().await.get(domain).cloned()
    }

    pub async fn add(&self, host: ProxyHost) {
        self.hosts
            .write()
            .await
            .insert(host.domain.clone(), Arc::new(host));
    }

    pub async fn update(&self, host: ProxyHost) {
        self.add(host).await;
    }

    pub async fn remove(&self, domain: &str) {
        self.hosts.write().await.remove(domain);
    }

    pub async fn update_upstream(&self, upstream: &UpstreamModel) {
        let mut hosts = self.hosts.write().await;

        for (_, proxy_host) in hosts.iter_mut() {
            let _ = Arc::make_mut(proxy_host).update_upstream(upstream);
        }
    }
}
