//! Minimum Withdrawal Limit Proposal.
use crate::ProposalHeader;

/// Minimum Withdrawal Limit Proposal:
/// the format of the proposal looks like this:
/// ```text
/// ┌────────────────────┬────────────────────────┐
/// │                    │                        │
/// │ ProposalHeader 40B │ MinWithdrawalLimit 32B │
/// │                    │                        │
/// └────────────────────┴────────────────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MinWithdrawalLimitProposal {
    header: ProposalHeader,
    min_withdrawal_limit: [u8; 32],
}

impl MinWithdrawalLimitProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH + 32; // min_withdrawal_limit

    /// Creates a new min withdrawal limit proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, min_limit: [u8; 32]) -> Self {
        Self {
            header,
            min_withdrawal_limit: min_limit,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the min withdrawal limit.
    #[must_use]
    pub const fn min_withdrawal_limit(&self) -> [u8; 32] {
        self.min_withdrawal_limit
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        let f = t;
        let t = t + 32;
        bytes[f..t].copy_from_slice(&self.min_withdrawal_limit);
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; MinWithdrawalLimitProposal::LENGTH]>
    for MinWithdrawalLimitProposal
{
    fn from(bytes: [u8; MinWithdrawalLimitProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let f = t;
        let t = t + 32;
        let mut min_withdrawal_limit = [0u8; 32];
        min_withdrawal_limit.copy_from_slice(&bytes[f..t]);
        Self::new(header, min_withdrawal_limit)
    }
}

impl From<MinWithdrawalLimitProposal>
    for [u8; MinWithdrawalLimitProposal::LENGTH]
{
    fn from(proposal: MinWithdrawalLimitProposal) -> Self {
        proposal.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ChainId, ChainType, FunctionSignature, Nonce, ResourceId, TargetSystem,
    };

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain_type = ChainType::Evm;
        let target_chain_id = ChainId::from(4);
        let resource_id =
            ResourceId::new(target_system, target_chain_type, target_chain_id);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let min_withdrawal_limit = hex_literal::hex!(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
        let proposal =
            MinWithdrawalLimitProposal::new(header, min_withdrawal_limit);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
        let proposal = MinWithdrawalLimitProposal::from(bytes);
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain_type = resource_id.chain_type();
        let target_chain_id = resource_id.chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        let min_withdrawal_limit = proposal.min_withdrawal_limit();
        assert_eq!(
            target_system,
            TargetSystem::new_contract_address(hex_literal::hex!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ))
        );
        assert_eq!(target_chain_type, ChainType::Evm);
        assert_eq!(target_chain_id, ChainId::from(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("cafebabe"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            min_withdrawal_limit,
            hex_literal::hex!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
            )
        );
    }
}
