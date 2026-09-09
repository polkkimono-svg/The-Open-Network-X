#![no_main]

use libfuzzer_sys::fuzz_target;
use onx_blocks::flags::validate_flags;
use onx_consensus::{BftQuorumTracker, BlockSignatureWithDepth, ValidatorSetEntry};
use onx_data_structures::{BlockHeader, ShardIdent, WorkchainIdent};
use onx_primitives::{PublicKey, Signature, Uint256, Uint64};

// Parses attacker-controlled headers and signature material, then exercises
// flag validation and BFT vote verification without accepting malformed input.
fuzz_target!(|data: &[u8]| {
    if let Ok(header) = BlockHeader::from_bytes(data) {
        let _ = validate_flags(header.flags.0);
    }
    if data.len() < PublicKey::BYTE_LEN + Signature::BYTE_LEN + 12 {
        return;
    }
    let key_end = PublicKey::BYTE_LEN;
    let signature_end = key_end + Signature::BYTE_LEN;
    let Ok(public_key) = PublicKey::decode_exact(&data[..key_end]) else {
        return;
    };
    let Ok(signature) = Signature::decode_exact(&data[key_end..signature_end]) else {
        return;
    };
    let validator_id =
        u32::from_be_bytes(data[signature_end..signature_end + 4].try_into().unwrap());
    let stake = u64::from_be_bytes(
        data[signature_end + 4..signature_end + 12]
            .try_into()
            .unwrap(),
    );
    let validator = ValidatorSetEntry {
        validator_id,
        public_key,
        actual_stake: Uint64::from(stake),
    };
    let mut tracker = BftQuorumTracker::new(
        ShardIdent::root(WorkchainIdent::BASIC),
        Uint256([0; 32]),
        stake,
    );
    let vote = BlockSignatureWithDepth {
        validator_id,
        depth: 0,
        signature,
    };
    let _ = tracker.add_vote(&vote, &validator);
    let _ = tracker.verify_quorum();
});
