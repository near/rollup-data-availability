use crate::Maintainer;
use near_sdk::{
    env::log_str,
    serde::{Deserialize, Serialize},
    serde_json,
};

const CONTRACT_STANDARD_NAME: &str = "nepXXX";
const CONTRACT_STANDARD_VERSION: &str = "1.0.0";

/// Interface to capture data about an event.
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl std::fmt::Display for EventLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        ))
    }
}

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    AddMaintainer(AddMaintainerLog),
}

/// An event log to capture a maintainer inclusion.
///
/// Arguments
/// * `owner_id`: "account.near"
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AddMaintainerLog {
    pub maintainer: Maintainer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Log the submission of a new maintainer by `log_str`ing an [`EventLogVariant::AddMaintainerLog`].
pub(crate) fn log_maintainer_inclusion(maintainer: Maintainer) {
    let log = EventLog {
        standard: CONTRACT_STANDARD_NAME.to_string(),
        version: CONTRACT_STANDARD_VERSION.to_string(),
        event: EventLogVariant::AddMaintainer(AddMaintainerLog {
            maintainer,
            memo: None,
        }),
    };
    log_str(&log.to_string());
}
