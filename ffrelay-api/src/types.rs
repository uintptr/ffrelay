use bon::Builder;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Deserialize, Tabled)]
pub struct FirefoxEmailRelay {
    pub id: u64,
    pub full_address: String,
    pub description: String,
    pub num_blocked: u64,
    pub num_forwarded: u64,
    pub num_replied: u64,
    pub num_spam: u64,
}

impl FirefoxEmailRelay {
    pub fn is_domain(&self) -> bool {
        if let Some((_, dom)) = self.full_address.split_once('@') {
            !dom.eq("mozmail.com")
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Builder)]
pub struct FirefoxEmailRelayRequest {
    description: String,
    #[builder(default = true)]
    enabled: bool,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, Tabled)]
pub struct FirefoxRelayProfile {
    pub id: u64,
    pub api_token: String,
    pub at_mask_limit: bool,
    pub avatar: String,
    pub date_subscribed: String,
    pub emails_blocked: u64,
    pub emails_forwarded: u64,
    pub emails_replied: u64,
    pub has_megabundle: bool,
    pub has_phone: bool,
    pub has_premium: bool,
    pub has_vpn: bool,
    pub level_one_trackers_blocked: u64,
    pub metrics_enabled: bool,
    pub next_email_try: String,
    pub onboarding_free_state: u32,
    pub onboarding_state: u32,
    pub remove_level_one_email_trackers: bool,
    pub server_storage: bool,
    pub store_phone_log: bool,
    pub subdomain: String,
    pub total_masks: u64,
}
