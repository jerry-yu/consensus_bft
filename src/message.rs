use bincode;
use cita_cloud_proto::common::ProposalWithProof;
use cita_cloud_proto::consensus::{
    consensus_service_server::ConsensusService, ConsensusConfiguration,
};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// enum BftMsgTyp {
//     Proposal,
//     Prevote,
//     Precommit,
// }

#[derive(Debug)]
pub enum BftSvrMsg {
    Conf(ConsensusConfiguration),
    PProof(ProposalWithProof),
}

pub enum BftToCtlMsg {}

pub enum CtlBackBftMsg {}

pub enum BftToNetMsg {}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Vote {
    pub sender: Vec<u8>,
    pub proposal: Vec<u8>,
    pub signature: Vec<u8>,
}

impl Vote {
    pub fn new() -> Self {
        Vote::default()
    }
}

impl Into<Vec<u8>> for Vote {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CompactSignedProposal {
    pub proposal: CompactProposal,
    pub sig: Vec<u8>,
}

impl CompactSignedProposal {
    pub fn new() -> Self {
        CompactSignedProposal::default()
    }
}

impl Into<Vec<u8>> for CompactSignedProposal {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CompactProposal {
    pub block: Vec<u8>,
    pub lock_round: u64,
    pub lock_votes: Vec<Vote>,
    pub round: u64,
    pub height: u64,
}

impl CompactProposal {
    pub fn new() -> Self {
        CompactProposal::default()
    }
}

impl Into<Vec<u8>> for CompactProposal {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}
