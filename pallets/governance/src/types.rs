use codec::{Encode, Decode};
use sp_runtime::{RuntimeDebug, DispatchError};
use sp_runtime::traits::{One,Zero, Bounded, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, Saturating};
use sp_std::vec::Vec;
use primitives::{Balance,CountryId, AccountId,BlockNumber,ProposalId, ReferendumId};
use frame_support::traits::Currency;

use crate::*;

#[derive(Encode, Decode,  Clone, PartialEq, Eq, RuntimeDebug)]
pub enum VoteThreshold {
    StandardQualifiedMajority, // 72%+ 72%+ representation
    TwoThirdsSupermajority, // 66%+
    ThreeFifthsSupermajority, // 60%+
    ReinforcedQualifiedMajority, // 55%+ 65%+ representation
    AbsoluteMajority, // 50%+s
    RelativeMajority, // Most votes
}

#[derive(Encode, Decode,  Clone, PartialEq, Eq, RuntimeDebug)]
pub enum CountryParameter {
    MaxProposals(u8),
    MaxParametersPerProposal(u8),
    SetReferendumJury(AccountId),
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct ReferendumParameters<BlockNumber> {
    pub(crate) voting_threshold: Option<VoteThreshold>,
    pub(crate) min_proposal_launch_period: BlockNumber,//ProposalLaunchPeriod, // number of blocks
    pub(crate) voting_period: BlockNumber, // number of blocks
    pub(crate) enactment_period: BlockNumber, // number of blocks
    pub(crate) max_params_per_proposal: u8,
    pub(crate) max_proposals_per_country: u8,
}




#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Vote {
    pub(crate) aye: bool,
    // pub(crate) who: AccountId,
    // pub(crate) balance: Balance,
}

/// Tally Struct
#[derive(Encode, Decode,Default, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Tally {
    pub(crate) ayes: u32,
    pub(crate) nays: u32,
    pub(crate) turnout: u32,
}

impl Tally {

    /// Add an account's vote into the tally.
    pub fn add(
		&mut self,
		vote: Vote,
	) -> Option<()> {
        match vote.aye {
            true => self.ayes = self.ayes.checked_add(One::one())?,
            false => self.nays = self.nays.checked_add(One::one())?,
        }
        self.turnout = self.ayes.checked_add(One::one())?;
		Some(())
	}

    /// Add an account's vote into the tally.
    pub fn remove(
		&mut self,
		vote: Vote,
	) -> Option<()> {
        match vote.aye {
            true => self.ayes = self.ayes.checked_sub(One::one())?,
            false => self.nays = self.nays.checked_sub(One::one())?,
        }
        self.turnout = self.ayes.checked_sub(One::one())?;
		Some(())
	}


}


#[derive(Encode, Decode, Default,  Clone, PartialEq, Eq, RuntimeDebug)]
pub struct VotingRecord {
    pub(crate) votes: Vec<(ReferendumId,Vote)>
}


#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct ProposalInfo<AccountId,BlockNumber,CountryParameter> {
    pub(crate) proposed_by: AccountId,
    pub(crate) parameters: Vec<CountryParameter>,
    pub(crate) description: Vec<u8>, // link to proposal description
    pub(crate) referendum_launch_block: BlockNumber,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct ReferendumStatus<BlockNumber> {
    pub(crate) end: BlockNumber,
    pub(crate) country: CountryId,
    pub(crate) proposal: ProposalId,
    pub(crate) tally: Tally,
    pub(crate) threshold: Option<VoteThreshold>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum ReferendumInfo<BlockNumber> {
    Ongoing(ReferendumStatus<BlockNumber>),
    Finished{passed: bool, end: BlockNumber},
}