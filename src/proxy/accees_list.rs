use std::net::IpAddr;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use ipnet::IpNet;

use crate::database::models::{AccessListModel, AccessListRuleModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Allow,
    Deny,
}

impl TryFrom<&str> for RuleAction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            _ => Err(anyhow!("Invalid rule action: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAddress {
    Ip(IpAddr),
    Cidr(IpNet),
}

impl RuleAddress {
    pub fn matches(&self, ip: &IpAddr) -> bool {
        match self {
            RuleAddress::Ip(rule_ip) => rule_ip == ip,
            RuleAddress::Cidr(net) => net.contains(ip),
        }
    }
}

impl TryFrom<&str> for RuleAddress {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.contains('/') {
            let net = IpNet::from_str(value)
                .map_err(|e| anyhow!("Invalid CIDR address '{value}': {e}"))?;
            Ok(RuleAddress::Cidr(net))
        } else {
            let ip = IpAddr::from_str(value)
                .map_err(|e| anyhow!("Invalid IP address '{value}': {e}"))?;
            Ok(RuleAddress::Ip(ip))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessRule {
    pub action: RuleAction,
    pub address: RuleAddress,
}

impl AccessRule {
    pub fn new(action: RuleAction, address: RuleAddress) -> Self {
        Self { action, address }
    }

    pub fn matches(&self, ip: &IpAddr) -> bool {
        self.address.matches(ip)
    }
}

impl TryFrom<AccessListRuleModel> for AccessRule {
    type Error = anyhow::Error;

    fn try_from(model: AccessListRuleModel) -> Result<Self> {
        Ok(AccessRule::new(
            RuleAction::try_from(model.action.as_str())?,
            RuleAddress::try_from(model.address.as_str())?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessList {
    pub id: i32,
    name: String,
    rules: Vec<AccessRule>,
}

impl AccessList {
    pub fn new(list: AccessListModel, mut rules: Vec<AccessListRuleModel>) -> Result<Self> {
        rules.sort_by_key(|r| r.sort_order);

        let rules = rules
            .into_iter()
            .map(AccessRule::try_from)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            id: list.id,
            name: list.name,
            rules,
        })
    }
}

impl AccessList {
    pub fn evaluate(&self, ip: &IpAddr) -> RuleAction {
        self.rules
            .iter()
            .find(|r| r.matches(ip))
            .map_or(RuleAction::Deny, |r| r.action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn make_model(id: i32, action: &str, address: &str, sort_order: i32) -> AccessListRuleModel {
        AccessListRuleModel {
            id,
            access_list_id: 1,
            action: action.to_string(),
            address: address.to_string(),
            sort_order,
            created_at: Utc::now(),
        }
    }

    fn make_list(rules: Vec<AccessListRuleModel>) -> AccessList {
        AccessList::new(
            AccessListModel {
                id: 1,
                name: "test".to_string(),
                created_at: Default::default(),
            },
            rules,
        )
        .unwrap()
    }

    fn ip(s: &str) -> IpAddr {
        IpAddr::from_str(s).unwrap()
    }

    #[test]
    fn test_rule_action_allow() {
        assert_eq!(RuleAction::try_from("allow").unwrap(), RuleAction::Allow);
    }

    #[test]
    fn test_rule_action_deny() {
        assert_eq!(RuleAction::try_from("deny").unwrap(), RuleAction::Deny);
    }

    #[test]
    fn test_rule_action_invalid() {
        assert!(RuleAction::try_from("permit").is_err());
        assert!(RuleAction::try_from("").is_err());
    }

    #[test]
    fn test_rule_address_ipv4() {
        assert_eq!(
            RuleAddress::try_from("1.2.3.4").unwrap(),
            RuleAddress::Ip(ip("1.2.3.4"))
        );
    }

    #[test]
    fn test_rule_address_ipv6() {
        assert_eq!(
            RuleAddress::try_from("::1").unwrap(),
            RuleAddress::Ip(ip("::1"))
        );
    }

    #[test]
    fn test_rule_address_cidr_v4() {
        assert!(matches!(
            RuleAddress::try_from("10.0.0.0/8").unwrap(),
            RuleAddress::Cidr(_)
        ));
    }

    #[test]
    fn test_rule_address_cidr_v6() {
        assert!(matches!(
            RuleAddress::try_from("2001:db8::/32").unwrap(),
            RuleAddress::Cidr(_)
        ));
    }

    #[test]
    fn test_rule_address_invalid() {
        assert!(RuleAddress::try_from("not-an-ip").is_err());
        assert!(RuleAddress::try_from("999.999.999.999").is_err());
        assert!(RuleAddress::try_from("10.0.0.0/33").is_err());
    }

    #[test]
    fn test_exact_ip_matches() {
        let addr = RuleAddress::try_from("192.168.1.1").unwrap();
        assert!(addr.matches(&ip("192.168.1.1")));
        assert!(!addr.matches(&ip("192.168.1.2")));
    }

    #[test]
    fn test_cidr_matches() {
        let addr = RuleAddress::try_from("10.0.0.0/8").unwrap();
        assert!(addr.matches(&ip("10.1.2.3")));
        assert!(addr.matches(&ip("10.255.255.255")));
        assert!(!addr.matches(&ip("11.0.0.1")));
    }

    #[test]
    fn test_cidr_v6_matches() {
        let addr = RuleAddress::try_from("2001:db8::/32").unwrap();
        assert!(addr.matches(&ip("2001:db8::1")));
        assert!(!addr.matches(&ip("2001:db9::1")));
    }

    #[test]
    fn test_no_rules_defaults_to_deny() {
        let list = make_list(vec![]);
        assert_eq!(list.evaluate(&ip("1.2.3.4")), RuleAction::Deny);
    }

    #[test]
    fn test_single_allow_rule_matches() {
        let list = make_list(vec![make_model(1, "allow", "1.2.3.4", 0)]);
        assert_eq!(list.evaluate(&ip("1.2.3.4")), RuleAction::Allow);
    }

    #[test]
    fn test_single_deny_rule_matches() {
        let list = make_list(vec![make_model(1, "deny", "1.2.3.4", 0)]);
        assert_eq!(list.evaluate(&ip("1.2.3.4")), RuleAction::Deny);
    }

    #[test]
    fn test_no_matching_rule_defaults_to_deny() {
        let list = make_list(vec![make_model(1, "allow", "1.2.3.4", 0)]);
        assert_eq!(list.evaluate(&ip("9.9.9.9")), RuleAction::Deny);
    }

    #[test]
    fn test_cidr_allow_rule() {
        let list = make_list(vec![make_model(1, "allow", "192.168.0.0/16", 0)]);
        assert_eq!(list.evaluate(&ip("192.168.5.10")), RuleAction::Allow);
        assert_eq!(list.evaluate(&ip("10.0.0.1")), RuleAction::Deny);
    }

    #[test]
    fn test_first_matching_rule_wins() {
        let list = make_list(vec![
            make_model(1, "deny", "10.0.0.0/8", 0),
            make_model(2, "allow", "10.0.0.1", 1),
        ]);
        assert_eq!(list.evaluate(&ip("10.0.0.1")), RuleAction::Deny);
    }

    #[test]
    fn test_sort_order_respected() {
        let list = make_list(vec![
            make_model(1, "deny", "10.0.0.0/8", 10),
            make_model(2, "allow", "10.0.0.1", 1),
        ]);
        assert_eq!(list.evaluate(&ip("10.0.0.1")), RuleAction::Allow);
    }
}
