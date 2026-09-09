//! Structural block validity for ordinary (non-split, non-merge) blocks,
//! per `docs/specification/blocks.md` §3.1 and §5 rules 1-6.
//!
//! Split successors (each child still has exactly one parent, so they are
//! representable today) share this same check. Merge successors are out of
//! scope: validating a merge block's two parents requires `BlockHeader`'s
//! `prev_ref_hash_2`/`MERGE_RESULT` fields from ADR-0016, which
//! `crates/onx-data-structures` does not yet implement (tracked in
//! `ROADMAP.md`). Recomputing the load-based split/merge trigger conditions
//! themselves is a Dynamic Sharding (ONX-ARCH-007) concern this crate does
//! not model.

use crate::error::BlocksError;
use crate::flags;
use onx_data_structures::BlockHeader;
use onx_primitives::Uint256;

/// The transaction/message-execution roots recomputed from a block
/// candidate's admitted messages, to be compared against the header's
/// declared values per §3.1 rule 6 / §5 rule 4. Recomputing these from a
/// candidate's message set is an execution-layer concern outside this
/// crate's scope (no execution engine exists yet); callers supply the
/// already-recomputed values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecomputedRoots {
    pub state_root_hash: Uint256,
    pub in_msg_root_hash: Uint256,
    pub out_msg_root_hash: Uint256,
}

/// Validates that `header` is a structurally valid ordinary successor of
/// `parent`, per §3.1 rules 1-6 and §5 rules 1-6 (excluding rule 5's
/// `MERGE_RESULT`-consistency half, since that field does not exist in code
/// yet).
///
/// `parent` must be the actual header the node resolved for `header`'s
/// declared parent reference; a caller unable to resolve a parent at all
/// simply has no `BlockHeader` to pass and cannot call this function,
/// satisfying rule 1 ("unresolvable parent") at the call site. This
/// function itself only checks that the resolved `parent`'s hash actually
/// matches the declared reference, guarding against a caller mistake.
pub fn validate_ordinary_successor(
    header: &BlockHeader,
    parent: &BlockHeader,
    recomputed: RecomputedRoots,
) -> Result<(), BlocksError> {
    flags::validate_flags(header.flags.0)?;

    if header.prev_ref_hash != parent.block_hash() {
        return Err(BlocksError::UnresolvableParent);
    }

    if u64::from(header.seq_no.0) != u64::from(parent.seq_no.0) + 1 {
        return Err(BlocksError::SequenceDiscontinuity {
            expected: parent.seq_no.0.wrapping_add(1),
            actual: header.seq_no.0,
        });
    }

    if header.gen_utime.0 < parent.gen_utime.0 {
        return Err(BlocksError::NonMonotonicTime);
    }
    if header.start_lt.0 < parent.end_lt.0 || header.start_lt.0 > header.end_lt.0 {
        return Err(BlocksError::NonMonotonicTime);
    }

    if header.state_root_hash != recomputed.state_root_hash
        || header.in_msg_root_hash != recomputed.in_msg_root_hash
        || header.out_msg_root_hash != recomputed.out_msg_root_hash
    {
        return Err(BlocksError::StateRootMismatch);
    }

    if flags::is_split_commit(parent.flags.0) || flags::is_merge_commit(parent.flags.0) {
        return Err(BlocksError::InvalidSuccessorAfterCommit);
    }

    Ok(())
}
