use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultMode {
    Honest,
    Offline,
    NoVote,
    Byzantine, // for now, it signs votes for conflicting blocks in the same slot
}
