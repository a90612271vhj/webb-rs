pub use governancebravodelegatecontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod governancebravodelegatecontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "GovernanceBravoDelegateContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static GOVERNANCEBRAVODELEGATECONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\"}],\"name\":\"NewAdmin\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"oldImplementation\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\"}],\"name\":\"NewImplementation\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"oldPendingAdmin\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\"}],\"name\":\"NewPendingAdmin\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"name\":\"ProposalCanceled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\"},{\"indexed\":false,\"internalType\":\"string[]\",\"name\":\"signatures\",\"type\":\"string[]\"},{\"indexed\":false,\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"startBlock\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"endBlock\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\"}],\"name\":\"ProposalCreated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"name\":\"ProposalExecuted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\"}],\"name\":\"ProposalQueued\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"oldProposalThreshold\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newProposalThreshold\",\"type\":\"uint256\"}],\"name\":\"ProposalThresholdSet\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"voter\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"votes\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"reason\",\"type\":\"string\"}],\"name\":\"VoteCast\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"oldVotingDelay\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newVotingDelay\",\"type\":\"uint256\"}],\"name\":\"VotingDelaySet\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"oldVotingPeriod\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newVotingPeriod\",\"type\":\"uint256\"}],\"name\":\"VotingPeriodSet\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"BALLOT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_PROPOSAL_THRESHOLD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_VOTING_DELAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_VOTING_PERIOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MIN_PROPOSAL_THRESHOLD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MIN_VOTING_DELAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MIN_VOTING_PERIOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_acceptAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"governorAlpha\",\"type\":\"address\"}],\"name\":\"_initiate\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\"}],\"name\":\"_setPendingAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newProposalThreshold\",\"type\":\"uint256\"}],\"name\":\"_setProposalThreshold\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newVotingDelay\",\"type\":\"uint256\"}],\"name\":\"_setVotingDelay\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newVotingPeriod\",\"type\":\"uint256\"}],\"name\":\"_setVotingPeriod\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"}],\"name\":\"cancel\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\"}],\"name\":\"castVote\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"castVoteBySig\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\"},{\"internalType\":\"string\",\"name\":\"reason\",\"type\":\"string\"}],\"name\":\"castVoteWithReason\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"comp\",\"outputs\":[{\"internalType\":\"contract CompInterface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"}],\"name\":\"execute\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"}],\"name\":\"getActions\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\"},{\"internalType\":\"string[]\",\"name\":\"signatures\",\"type\":\"string[]\"},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"voter\",\"type\":\"address\"}],\"name\":\"getReceipt\",\"outputs\":[{\"components\":[{\"internalType\":\"bool\",\"name\":\"hasVoted\",\"type\":\"bool\"},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\"},{\"internalType\":\"uint96\",\"name\":\"votes\",\"type\":\"uint96\"}],\"internalType\":\"struct GovernorBravoDelegateStorageV1.Receipt\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"initialProposalId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"timelock_\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"comp_\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"votingPeriod_\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"votingDelay_\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"proposalThreshold_\",\"type\":\"uint256\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"latestProposalIds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proposalCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proposalMaxOperations\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proposalThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"proposals\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startBlock\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endBlock\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"forVotes\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"againstVotes\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"abstainVotes\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"canceled\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"executed\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\"},{\"internalType\":\"string[]\",\"name\":\"signatures\",\"type\":\"string[]\"},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\"},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\"}],\"name\":\"propose\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"}],\"name\":\"queue\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"quorumVotes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\"}],\"name\":\"state\",\"outputs\":[{\"internalType\":\"enum GovernorBravoDelegateStorageV1.ProposalState\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"timelock\",\"outputs\":[{\"internalType\":\"contract TimelockInterface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"votingDelay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"votingPeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct GovernanceBravoDelegateContract<M>(
        ethers::contract::Contract<M>,
    );
    impl<M> std::ops::Deref for GovernanceBravoDelegateContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for GovernanceBravoDelegateContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(GovernanceBravoDelegateContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> GovernanceBravoDelegateContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                GOVERNANCEBRAVODELEGATECONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `BALLOT_TYPEHASH` (0xdeaaa7cc) function"]
        pub fn ballot_typehash(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([222, 170, 167, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function"]
        pub fn domain_typehash(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_PROPOSAL_THRESHOLD` (0x25fd935a) function"]
        pub fn max_proposal_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([37, 253, 147, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_VOTING_DELAY` (0xb1126263) function"]
        pub fn max_voting_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([177, 18, 98, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_VOTING_PERIOD` (0xa64e024a) function"]
        pub fn max_voting_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([166, 78, 2, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MIN_PROPOSAL_THRESHOLD` (0x791f5d23) function"]
        pub fn min_proposal_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([121, 31, 93, 35], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MIN_VOTING_DELAY` (0xe48083fe) function"]
        pub fn min_voting_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([228, 128, 131, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MIN_VOTING_PERIOD` (0x215809ca) function"]
        pub fn min_voting_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([33, 88, 9, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_acceptAdmin` (0xe9c714f2) function"]
        pub fn accept_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 199, 20, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_initiate` (0xf9d28b80) function"]
        pub fn initiate(
            &self,
            governor_alpha: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 210, 139, 128], governor_alpha)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setPendingAdmin` (0xb71d1a0c) function"]
        pub fn set_pending_admin(
            &self,
            new_pending_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 29, 26, 12], new_pending_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setProposalThreshold` (0x17ba1b8b) function"]
        pub fn set_proposal_threshold(
            &self,
            new_proposal_threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 186, 27, 139], new_proposal_threshold)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setVotingDelay` (0x1dfb1b5a) function"]
        pub fn set_voting_delay(
            &self,
            new_voting_delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 251, 27, 90], new_voting_delay)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setVotingPeriod` (0x0ea2d98c) function"]
        pub fn set_voting_period(
            &self,
            new_voting_period: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 162, 217, 140], new_voting_period)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancel` (0x40e58ee5) function"]
        pub fn cancel(
            &self,
            proposal_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 229, 142, 229], proposal_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `castVote` (0x56781388) function"]
        pub fn cast_vote(
            &self,
            proposal_id: ethers::core::types::U256,
            support: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 120, 19, 136], (proposal_id, support))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `castVoteBySig` (0x3bccf4fd) function"]
        pub fn cast_vote_by_sig(
            &self,
            proposal_id: ethers::core::types::U256,
            support: u8,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [59, 204, 244, 253],
                    (proposal_id, support, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `castVoteWithReason` (0x7b3c71d3) function"]
        pub fn cast_vote_with_reason(
            &self,
            proposal_id: ethers::core::types::U256,
            support: u8,
            reason: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 60, 113, 211],
                    (proposal_id, support, reason),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `comp` (0x109d0af8) function"]
        pub fn comp(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([16, 157, 10, 248], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0xfe0d94c1) function"]
        pub fn execute(
            &self,
            proposal_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 13, 148, 193], proposal_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getActions` (0x328dd982) function"]
        pub fn get_actions(
            &self,
            proposal_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                Vec<ethers::core::types::Address>,
                Vec<ethers::core::types::U256>,
                Vec<String>,
                Vec<Vec<u8>>,
            ),
        > {
            self.0
                .method_hash([50, 141, 217, 130], proposal_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReceipt` (0xe23a9a52) function"]
        pub fn get_receipt(
            &self,
            proposal_id: ethers::core::types::U256,
            voter: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (bool, u8, u128)>
        {
            self.0
                .method_hash([226, 58, 154, 82], (proposal_id, voter))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementation` (0x5c60da1b) function"]
        pub fn implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialProposalId` (0xfc4eee42) function"]
        pub fn initial_proposal_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([252, 78, 238, 66], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xd13f90b4) function"]
        pub fn initialize(
            &self,
            timelock: ethers::core::types::Address,
            comp: ethers::core::types::Address,
            voting_period: ethers::core::types::U256,
            voting_delay: ethers::core::types::U256,
            proposal_threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [209, 63, 144, 180],
                    (
                        timelock,
                        comp,
                        voting_period,
                        voting_delay,
                        proposal_threshold,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestProposalIds` (0x17977c61) function"]
        pub fn latest_proposal_ids(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([23, 151, 124, 97], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingAdmin` (0x26782247) function"]
        pub fn pending_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposalCount` (0xda35c664) function"]
        pub fn proposal_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([218, 53, 198, 100], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposalMaxOperations` (0x7bdbe4d0) function"]
        pub fn proposal_max_operations(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([123, 219, 228, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposalThreshold` (0xb58131b0) function"]
        pub fn proposal_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([181, 129, 49, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposals` (0x013cf08b) function"]
        pub fn proposals(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                bool,
            ),
        > {
            self.0
                .method_hash([1, 60, 240, 139], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `propose` (0xda95691a) function"]
        pub fn propose(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            signatures: ::std::vec::Vec<String>,
            calldatas: ::std::vec::Vec<Vec<u8>>,
            description: String,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash(
                    [218, 149, 105, 26],
                    (targets, values, signatures, calldatas, description),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queue` (0xddf0b009) function"]
        pub fn queue(
            &self,
            proposal_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 240, 176, 9], proposal_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quorumVotes` (0x24bc1a64) function"]
        pub fn quorum_votes(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([36, 188, 26, 100], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state` (0x3e4f49e6) function"]
        pub fn state(
            &self,
            proposal_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([62, 79, 73, 230], proposal_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `timelock` (0xd33219b4) function"]
        pub fn timelock(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([211, 50, 25, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `votingDelay` (0x3932abb1) function"]
        pub fn voting_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([57, 50, 171, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `votingPeriod` (0x02a251a3) function"]
        pub fn voting_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([2, 162, 81, 163], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewAdmin` event"]
        pub fn new_admin_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewImplementation` event"]
        pub fn new_implementation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewImplementationFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPendingAdmin` event"]
        pub fn new_pending_admin_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPendingAdminFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposalCanceled` event"]
        pub fn proposal_canceled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposalCanceledFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposalCreated` event"]
        pub fn proposal_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposalCreatedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposalExecuted` event"]
        pub fn proposal_executed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposalExecutedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposalQueued` event"]
        pub fn proposal_queued_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposalQueuedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposalThresholdSet` event"]
        pub fn proposal_threshold_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposalThresholdSetFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `VoteCast` event"]
        pub fn vote_cast_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VoteCastFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VotingDelaySet` event"]
        pub fn voting_delay_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VotingDelaySetFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `VotingPeriodSet` event"]
        pub fn voting_period_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VotingPeriodSetFilter>
        {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            GovernanceBravoDelegateContractEvents,
        > {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "NewAdmin", abi = "NewAdmin(address,address)")]
    pub struct NewAdminFilter {
        pub old_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "NewImplementation",
        abi = "NewImplementation(address,address)"
    )]
    pub struct NewImplementationFilter {
        pub old_implementation: ethers::core::types::Address,
        pub new_implementation: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "NewPendingAdmin",
        abi = "NewPendingAdmin(address,address)"
    )]
    pub struct NewPendingAdminFilter {
        pub old_pending_admin: ethers::core::types::Address,
        pub new_pending_admin: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "ProposalCanceled", abi = "ProposalCanceled(uint256)")]
    pub struct ProposalCanceledFilter {
        pub id: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "ProposalCreated",
        abi = "ProposalCreated(uint256,address,address[],uint256[],string[],bytes[],uint256,uint256,string)"
    )]
    pub struct ProposalCreatedFilter {
        pub id: ethers::core::types::U256,
        pub proposer: ethers::core::types::Address,
        pub targets: Vec<ethers::core::types::Address>,
        pub values: Vec<ethers::core::types::U256>,
        pub signatures: Vec<String>,
        pub calldatas: Vec<Vec<u8>>,
        pub start_block: ethers::core::types::U256,
        pub end_block: ethers::core::types::U256,
        pub description: String,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "ProposalExecuted", abi = "ProposalExecuted(uint256)")]
    pub struct ProposalExecutedFilter {
        pub id: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "ProposalQueued",
        abi = "ProposalQueued(uint256,uint256)"
    )]
    pub struct ProposalQueuedFilter {
        pub id: ethers::core::types::U256,
        pub eta: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "ProposalThresholdSet",
        abi = "ProposalThresholdSet(uint256,uint256)"
    )]
    pub struct ProposalThresholdSetFilter {
        pub old_proposal_threshold: ethers::core::types::U256,
        pub new_proposal_threshold: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "VoteCast",
        abi = "VoteCast(address,uint256,uint8,uint256,string)"
    )]
    pub struct VoteCastFilter {
        #[ethevent(indexed)]
        pub voter: ethers::core::types::Address,
        pub proposal_id: ethers::core::types::U256,
        pub support: u8,
        pub votes: ethers::core::types::U256,
        pub reason: String,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "VotingDelaySet",
        abi = "VotingDelaySet(uint256,uint256)"
    )]
    pub struct VotingDelaySetFilter {
        pub old_voting_delay: ethers::core::types::U256,
        pub new_voting_delay: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "VotingPeriodSet",
        abi = "VotingPeriodSet(uint256,uint256)"
    )]
    pub struct VotingPeriodSetFilter {
        pub old_voting_period: ethers::core::types::U256,
        pub new_voting_period: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum GovernanceBravoDelegateContractEvents {
        NewAdminFilter(NewAdminFilter),
        NewImplementationFilter(NewImplementationFilter),
        NewPendingAdminFilter(NewPendingAdminFilter),
        ProposalCanceledFilter(ProposalCanceledFilter),
        ProposalCreatedFilter(ProposalCreatedFilter),
        ProposalExecutedFilter(ProposalExecutedFilter),
        ProposalQueuedFilter(ProposalQueuedFilter),
        ProposalThresholdSetFilter(ProposalThresholdSetFilter),
        VoteCastFilter(VoteCastFilter),
        VotingDelaySetFilter(VotingDelaySetFilter),
        VotingPeriodSetFilter(VotingPeriodSetFilter),
    }
    impl ethers::core::abi::Tokenizable for GovernanceBravoDelegateContractEvents {
        fn from_token(
            token: ethers::core::abi::Token,
        ) -> Result<Self, ethers::core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewAdminFilter::from_token(token.clone()) {
                return Ok(
                    GovernanceBravoDelegateContractEvents::NewAdminFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                NewImplementationFilter::from_token(token.clone())
            {
                return Ok (GovernanceBravoDelegateContractEvents :: NewImplementationFilter (decoded));
            }
            if let Ok(decoded) =
                NewPendingAdminFilter::from_token(token.clone())
            {
                return Ok (GovernanceBravoDelegateContractEvents :: NewPendingAdminFilter (decoded));
            }
            if let Ok(decoded) =
                ProposalCanceledFilter::from_token(token.clone())
            {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalCanceledFilter (decoded));
            }
            if let Ok(decoded) =
                ProposalCreatedFilter::from_token(token.clone())
            {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalCreatedFilter (decoded));
            }
            if let Ok(decoded) =
                ProposalExecutedFilter::from_token(token.clone())
            {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalExecutedFilter (decoded));
            }
            if let Ok(decoded) = ProposalQueuedFilter::from_token(token.clone())
            {
                return Ok(
                    GovernanceBravoDelegateContractEvents::ProposalQueuedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                ProposalThresholdSetFilter::from_token(token.clone())
            {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalThresholdSetFilter (decoded));
            }
            if let Ok(decoded) = VoteCastFilter::from_token(token.clone()) {
                return Ok(
                    GovernanceBravoDelegateContractEvents::VoteCastFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = VotingDelaySetFilter::from_token(token.clone())
            {
                return Ok(
                    GovernanceBravoDelegateContractEvents::VotingDelaySetFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                VotingPeriodSetFilter::from_token(token.clone())
            {
                return Ok (GovernanceBravoDelegateContractEvents :: VotingPeriodSetFilter (decoded));
            }
            Err(ethers::core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers::core::abi::Token {
            match self { GovernanceBravoDelegateContractEvents :: NewAdminFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: NewImplementationFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: NewPendingAdminFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: ProposalCanceledFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: ProposalCreatedFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: ProposalExecutedFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: ProposalQueuedFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: ProposalThresholdSetFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: VoteCastFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: VotingDelaySetFilter (element) => element . into_token () , GovernanceBravoDelegateContractEvents :: VotingPeriodSetFilter (element) => element . into_token () }
        }
    }
    impl ethers::core::abi::TokenizableItem
        for GovernanceBravoDelegateContractEvents
    {
    }
    impl ethers::contract::EthLogDecode for GovernanceBravoDelegateContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(
                    GovernanceBravoDelegateContractEvents::NewAdminFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = NewImplementationFilter::decode_log(log) {
                return Ok (GovernanceBravoDelegateContractEvents :: NewImplementationFilter (decoded));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok (GovernanceBravoDelegateContractEvents :: NewPendingAdminFilter (decoded));
            }
            if let Ok(decoded) = ProposalCanceledFilter::decode_log(log) {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalCanceledFilter (decoded));
            }
            if let Ok(decoded) = ProposalCreatedFilter::decode_log(log) {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalCreatedFilter (decoded));
            }
            if let Ok(decoded) = ProposalExecutedFilter::decode_log(log) {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalExecutedFilter (decoded));
            }
            if let Ok(decoded) = ProposalQueuedFilter::decode_log(log) {
                return Ok(
                    GovernanceBravoDelegateContractEvents::ProposalQueuedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ProposalThresholdSetFilter::decode_log(log) {
                return Ok (GovernanceBravoDelegateContractEvents :: ProposalThresholdSetFilter (decoded));
            }
            if let Ok(decoded) = VoteCastFilter::decode_log(log) {
                return Ok(
                    GovernanceBravoDelegateContractEvents::VoteCastFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = VotingDelaySetFilter::decode_log(log) {
                return Ok(
                    GovernanceBravoDelegateContractEvents::VotingDelaySetFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = VotingPeriodSetFilter::decode_log(log) {
                return Ok (GovernanceBravoDelegateContractEvents :: VotingPeriodSetFilter (decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    #[doc = "`Receipt(bool,uint8,uint96)`"]
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType,
    )]
    pub struct Receipt {
        pub has_voted: bool,
        pub support: u8,
        pub votes: u128,
    }
}
