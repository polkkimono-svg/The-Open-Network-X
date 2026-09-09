//! ONX block structural validity, masterchain coupling, and split/merge
//! announcement flags.
//!
//! Implements `docs/specification/blocks.md` (ADR-0006) for the
//! non-split/non-merge case: structural validity of an ordinary successor
//! block (§3.1), masterchain coupling and canonicality via the
//! `Masterchain Block Extra` shard-configuration commitment (§3.3, §4.1),
//! and the split/merge announcement flag bit layout and its own
//! well-formedness rules (§3.4, §4.2).
//!
//! **Merge-block validation is intentionally not implemented here.** A
//! merge block's two-parent structural check requires `BlockHeader`'s
//! `prev_ref_hash_2` field and `MERGE_RESULT` flag, decided by ADR-0016
//! (resolving ONX-ARCH-013) but not yet added to
//! `crates/onx-data-structures`' `BlockHeader` implementation — see
//! `ROADMAP.md`. Split successors are validated by the same
//! [`validity::validate_ordinary_successor`] as any other block, since each
//! split child still has exactly one parent; the load-based trigger
//! conditions and validator task-group mechanics for both split and merge
//! remain a Dynamic Sharding (ONX-ARCH-007) concern this crate does not
//! model.

pub mod error;
pub mod flags;
pub mod masterchain;
pub mod validity;

pub use error::BlocksError;
pub use masterchain::{validate_master_ref, MasterchainBlockExtra, ShardEntry};
pub use validity::{validate_ordinary_successor, RecomputedRoots};
