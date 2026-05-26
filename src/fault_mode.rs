use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FaultMode {
    Honest,
    Offline,
    Byzantine(ByzantineMode),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ByzantineMode {
    VoteForFakeBlock,
    LeaderProducesConflictingBlocks,
}
